#![allow(dead_code)]

use arrow::{
    datatypes::{DataType, Field, IntervalUnit, TimeUnit},
    error::ArrowError,
};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::Value::String as VString;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub enum SchemaFileType {
    Json,
    Parquet,
}

impl SchemaFileType {
    pub fn s3_path(&self, name: String) -> url::Url {
        let extension = match self {
            SchemaFileType::Json => "json",
            SchemaFileType::Parquet => "parquet",
        };

        let path = std::env::var("S3_PATH").unwrap_or_else(|_| "ruspie".to_string());
        let path: String = format!("s3://{}/{}.{}", path, name, extension);
        url::Url::parse(&path).unwrap()
    }
}

pub struct S3FileContext<H: object_store::ObjectStore> {
    pub schema_file_type: SchemaFileType,
    pub schema_file_name: String,
    pub object_store: Arc<H>,
    schemas: Vec<SchemaFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SchemaFile {
    tables: Vec<TableItem>,
}

#[derive(Deserialize, Debug)]
struct SchemaResponse {
    fields: Vec<ColumnItemFetch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableItem {
    name: String,
    extension: String,
    schema: Schema,
}

impl Default for TableItem {
    fn default() -> Self {
        TableItem {
            name: "".to_string(),
            extension: "".to_string(),
            schema: default_schema(),
        }
    }
}

fn default_schema() -> Schema {
    Schema { columns: vec![] }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Schema {
    columns: Vec<ColumnItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColumnItemFetch {
    name: String,
    data_type: arrow::datatypes::DataType,
    nullable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColumnItem {
    name: String,
    #[serde(
        serialize_with = "serialize_data_type",
        deserialize_with = "deserialize_data_type"
    )]
    data_type: arrow::datatypes::DataType,
    nullable: Option<bool>,
}

impl From<ColumnItemFetch> for ColumnItem {
    fn from(item: ColumnItemFetch) -> Self {
        ColumnItem {
            name: item.name,
            data_type: item.data_type,
            nullable: item.nullable,
        }
    }
}

fn deserialize_data_type<'de, D>(deserializer: D) -> Result<arrow::datatypes::DataType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer).unwrap_or_else(|e| {
        panic!("Unable to deserialize data type: {:?}", e);
    });
    let value: Value = serde_json::from_str(&s).unwrap();

    let data_type = value_to_data_type(&value).unwrap_or_else(|e| {
        panic!("Unable to deserialize data type: {:?}", e);
    });
    Ok(data_type)
}

fn serialize_data_type<S>(
    data_type: &arrow::datatypes::DataType,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&data_type.to_json().to_string())
}

impl<H: object_store::ObjectStore> S3FileContext<H> {
    pub fn default(object_store: Arc<H>) -> anyhow::Result<S3FileContext<H>> {
        let schema_file_type = SchemaFileType::Json;
        let schema_file_name = "schemas".to_string();
        let schemas = vec![SchemaFile { tables: vec![] }];

        Ok(S3FileContext {
            schema_file_type,
            schema_file_name,
            object_store,
            schemas,
        })
    }

    async fn list_files(&self) -> anyhow::Result<Vec<TableItem>> {
        let mut list_stream = self.object_store.list(None).await.unwrap();
        let mut files: Vec<TableItem> = vec![];

        while let Some(file) = list_stream.next().await {
            let file = file.map_err(|e| println!("{:?}", e)).unwrap();
            if file.location.extension() == Some("parquet")
                || file.location.extension() == Some("csv")
            {
                let mut table_item = TableItem::default();
                table_item.extension = file.location.extension().unwrap().to_string();
                table_item.name = file
                    .location
                    .filename()
                    .unwrap()
                    .to_string()
                    .split('.')
                    .next()
                    .unwrap()
                    .to_string();
                files.push(table_item);
            }
        }

        Ok(files)
    }

    async fn fetch_schema(&mut self) -> anyhow::Result<()> {
        let path = self.schema_file_type.s3_path(self.schema_file_name.clone());
        let schemas = match self
            .object_store
            .get(&path.path().try_into().unwrap())
            .await
        {
            Ok(data) => {
                let data: Vec<u8> = data.bytes().await.unwrap().to_vec();
                let contents: String = String::from_utf8(data).unwrap();
                serde_json::from_str(&contents).unwrap_or_else(|e| {
                    println!("Error parsing `schemas.json`: {:?}", e);
                    vec![SchemaFile { tables: vec![] }]
                })
            }
            Err(e) => {
                println!(
                    "Error Fetching `schemas.json`: {:?},Will create a new one.",
                    e
                );
                vec![SchemaFile { tables: vec![] }]
            }
        };
        self.schemas = schemas;
        Ok(())
    }

    async fn schemas_to_fetch(&mut self) -> Vec<TableItem> {
        let files_in_s3 = self.list_files().await.unwrap();
        let _ = self.fetch_schema().await.unwrap();
        let mut schemas_to_fetched: Vec<TableItem> = vec![];

        for file in files_in_s3 {
            let mut found = false;
            for schema in self.schemas[0].tables.iter() {
                if file.name == schema.name && file.extension == schema.extension {
                    found = true;
                    break;
                }
            }
            if !found {
                let table_item = TableItem {
                    name: file.name.clone(),
                    extension: file.extension.clone(),
                    ..Default::default()
                };
                schemas_to_fetched.push(table_item);
            }
        }

        schemas_to_fetched
    }

    pub async fn fetch_schemas_from_ruspie(&mut self) -> anyhow::Result<()> {
        let schemas_to_fetch = self.schemas_to_fetch().await;
        println!(
            "before fetching ==============>{:?}",
            self.schemas[0].tables.len()
        );
        for schema in schemas_to_fetch {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());
            headers.insert("Accept", "application/json".parse().unwrap());
            headers.insert("FILE-EXTENSION", schema.extension.parse().unwrap());
            println!(
                "file extension {:?}==============> {:?}",
                schema.name, schema.extension
            );
            let client = reqwest::Client::new();
            let url =
                std::env::var("RUSPIE_URL").unwrap_or_else(|_| "http://0.0.0.0:8080".to_string());
            let url = format!("{}/api/schema/{}", url, schema.name);

            let response = client.get(&url).headers(headers).send().await;
            match response {
                Ok(d) => {
                    let schema_resp: Result<TableItem, reqwest::Error> =
                        d.json().await.and_then(|schema_resp: SchemaResponse| {
                            Ok(TableItem {
                                name: schema.name,
                                extension: schema.extension,
                                schema: Schema {
                                    columns: schema_resp
                                        .fields
                                        .iter()
                                        .map(|f| f.clone().into())
                                        .collect(),
                                },
                            })
                        });
                    match schema_resp {
                        Ok(schema) => {
                            self.schemas[0].tables.push(schema);
                        }
                        Err(e) => println!("Error in fetching schema {:?}", e.to_string()),
                    }
                }
                Err(e) => println!("error: {:?}", e),
            }
        }

        println!(
            "after fetching ==============> {:?}",
            self.schemas[0].tables.len()
        );
        self.write_to_json().await.unwrap();
        let _ = self.put_object_to_s3(&self.schema_file_name).await;
        Ok(())
    }
    async fn write_to_json(&mut self) -> anyhow::Result<()> {
        let content = serde_json::to_string(&self.schemas).unwrap();
        let mut file = tokio::fs::File::create("schemas.json").await?;
        file.write_all(content.as_bytes()).await?;
        Ok(())
    }

    async fn put_object_to_s3(&self, path: &str) -> anyhow::Result<()> {
        let mut file = tokio::fs::File::open("schemas.json").await.unwrap();
        let path = self.schema_file_type.s3_path(path.to_string());
        let mut data = Vec::new();
        file.read_to_end(&mut data).await?;
        let location = path.path().try_into().unwrap();
        self.object_store.put(&location, data.into()).await?;
        Ok(())
    }
}
fn value_to_data_type(json: &Value) -> Result<DataType, ArrowError> {
    let default_field = Field::new("", DataType::Boolean, true);
    match *json {
        Value::Object(ref map) => match map.get("name") {
            Some(s) if s == "null" => Ok(DataType::Null),
            Some(s) if s == "bool" => Ok(DataType::Boolean),
            Some(s) if s == "binary" => Ok(DataType::Binary),
            Some(s) if s == "largebinary" => Ok(DataType::LargeBinary),
            Some(s) if s == "utf8" => Ok(DataType::Utf8),
            Some(s) if s == "largeutf8" => Ok(DataType::LargeUtf8),
            Some(s) if s == "fixedsizebinary" => {
                // return a list with any type as its child isn't defined in the map
                if let Some(Value::Number(size)) = map.get("byteWidth") {
                    Ok(DataType::FixedSizeBinary(size.as_i64().unwrap() as i32))
                } else {
                    Err(ArrowError::ParseError(
                        "Expecting a byteWidth for fixedsizebinary".to_string(),
                    ))
                }
            }
            Some(s) if s == "decimal" => {
                // return a list with any type as its child isn't defined in the map
                let precision = match map.get("precision") {
                    Some(p) => Ok(p.as_u64().unwrap() as usize),
                    None => Err(ArrowError::ParseError(
                        "Expecting a precision for decimal".to_string(),
                    )),
                };
                let scale = match map.get("scale") {
                    Some(s) => Ok(s.as_u64().unwrap() as usize),
                    _ => Err(ArrowError::ParseError(
                        "Expecting a scale for decimal".to_string(),
                    )),
                };

                Ok(DataType::Decimal(precision?, scale?))
            }
            Some(s) if s == "floatingpoint" => match map.get("precision") {
                Some(p) if p == "HALF" => Ok(DataType::Float16),
                Some(p) if p == "SINGLE" => Ok(DataType::Float32),
                Some(p) if p == "DOUBLE" => Ok(DataType::Float64),
                _ => Err(ArrowError::ParseError(
                    "floatingpoint precision missing or invalid".to_string(),
                )),
            },
            Some(s) if s == "timestamp" => {
                let unit = match map.get("unit") {
                    Some(p) if p == "SECOND" => Ok(TimeUnit::Second),
                    Some(p) if p == "MILLISECOND" => Ok(TimeUnit::Millisecond),
                    Some(p) if p == "MICROSECOND" => Ok(TimeUnit::Microsecond),
                    Some(p) if p == "NANOSECOND" => Ok(TimeUnit::Nanosecond),
                    _ => Err(ArrowError::ParseError(
                        "timestamp unit missing or invalid".to_string(),
                    )),
                };
                let tz = match map.get("timezone") {
                    None => Ok(None),
                    Some(VString(tz)) => Ok(Some(tz.clone())),
                    _ => Err(ArrowError::ParseError(
                        "timezone must be a string".to_string(),
                    )),
                };
                Ok(DataType::Timestamp(unit?, tz?))
            }
            Some(s) if s == "date" => match map.get("unit") {
                Some(p) if p == "DAY" => Ok(DataType::Date32),
                Some(p) if p == "MILLISECOND" => Ok(DataType::Date64),
                _ => Err(ArrowError::ParseError(
                    "date unit missing or invalid".to_string(),
                )),
            },
            Some(s) if s == "time" => {
                let unit = match map.get("unit") {
                    Some(p) if p == "SECOND" => Ok(TimeUnit::Second),
                    Some(p) if p == "MILLISECOND" => Ok(TimeUnit::Millisecond),
                    Some(p) if p == "MICROSECOND" => Ok(TimeUnit::Microsecond),
                    Some(p) if p == "NANOSECOND" => Ok(TimeUnit::Nanosecond),
                    _ => Err(ArrowError::ParseError(
                        "time unit missing or invalid".to_string(),
                    )),
                };
                match map.get("bitWidth") {
                    Some(p) if p == 32 => Ok(DataType::Time32(unit?)),
                    Some(p) if p == 64 => Ok(DataType::Time64(unit?)),
                    _ => Err(ArrowError::ParseError(
                        "time bitWidth missing or invalid".to_string(),
                    )),
                }
            }
            Some(s) if s == "duration" => match map.get("unit") {
                Some(p) if p == "SECOND" => Ok(DataType::Duration(TimeUnit::Second)),
                Some(p) if p == "MILLISECOND" => Ok(DataType::Duration(TimeUnit::Millisecond)),
                Some(p) if p == "MICROSECOND" => Ok(DataType::Duration(TimeUnit::Microsecond)),
                Some(p) if p == "NANOSECOND" => Ok(DataType::Duration(TimeUnit::Nanosecond)),
                _ => Err(ArrowError::ParseError(
                    "time unit missing or invalid".to_string(),
                )),
            },
            Some(s) if s == "interval" => match map.get("unit") {
                Some(p) if p == "DAY_TIME" => Ok(DataType::Interval(IntervalUnit::DayTime)),
                Some(p) if p == "YEAR_MONTH" => Ok(DataType::Interval(IntervalUnit::YearMonth)),
                _ => Err(ArrowError::ParseError(
                    "interval unit missing or invalid".to_string(),
                )),
            },
            Some(s) if s == "int" => match map.get("isSigned") {
                Some(&Value::Bool(true)) => match map.get("bitWidth") {
                    Some(&Value::Number(ref n)) => match n.as_u64() {
                        Some(8) => Ok(DataType::Int8),
                        Some(16) => Ok(DataType::Int16),
                        Some(32) => Ok(DataType::Int32),
                        Some(64) => Ok(DataType::Int64),
                        _ => Err(ArrowError::ParseError(
                            "int bitWidth missing or invalid".to_string(),
                        )),
                    },
                    _ => Err(ArrowError::ParseError(
                        "int bitWidth missing or invalid".to_string(),
                    )),
                },
                Some(&Value::Bool(false)) => match map.get("bitWidth") {
                    Some(&Value::Number(ref n)) => match n.as_u64() {
                        Some(8) => Ok(DataType::UInt8),
                        Some(16) => Ok(DataType::UInt16),
                        Some(32) => Ok(DataType::UInt32),
                        Some(64) => Ok(DataType::UInt64),
                        _ => Err(ArrowError::ParseError(
                            "int bitWidth missing or invalid".to_string(),
                        )),
                    },
                    _ => Err(ArrowError::ParseError(
                        "int bitWidth missing or invalid".to_string(),
                    )),
                },
                _ => Err(ArrowError::ParseError(
                    "int signed missing or invalid".to_string(),
                )),
            },
            Some(s) if s == "list" => {
                // return a list with any type as its child isn't defined in the map
                Ok(DataType::List(Box::new(default_field)))
            }
            Some(s) if s == "largelist" => {
                // return a largelist with any type as its child isn't defined in the map
                Ok(DataType::LargeList(Box::new(default_field)))
            }
            Some(s) if s == "fixedsizelist" => {
                // return a list with any type as its child isn't defined in the map
                if let Some(Value::Number(size)) = map.get("listSize") {
                    Ok(DataType::FixedSizeList(
                        Box::new(default_field),
                        size.as_i64().unwrap() as i32,
                    ))
                } else {
                    Err(ArrowError::ParseError(
                        "Expecting a listSize for fixedsizelist".to_string(),
                    ))
                }
            }
            Some(s) if s == "struct" => {
                // return an empty `struct` type as its children aren't defined in the map
                Ok(DataType::Struct(vec![]))
            }
            Some(s) if s == "map" => {
                if let Some(Value::Bool(keys_sorted)) = map.get("keysSorted") {
                    // Return a map with an empty type as its children aren't defined in the map
                    Ok(DataType::Map(Box::new(default_field), *keys_sorted))
                } else {
                    Err(ArrowError::ParseError(
                        "Expecting a keysSorted for map".to_string(),
                    ))
                }
            }
            Some(other) => Err(ArrowError::ParseError(format!(
                "invalid or unsupported type name: {} in {:?}",
                other, json
            ))),
            None => Err(ArrowError::ParseError("type name missing".to_string())),
        },
        _ => Err(ArrowError::ParseError(
            "invalid json value type".to_string(),
        )),
    }
}

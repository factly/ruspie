use columnq::arrow::{
    datatypes::{DataType, Field, IntervalUnit, TimeUnit},
    error::ArrowError,
};
use columnq::table::{TableColumn, TableSchema};
use serde::Deserialize;
use serde_json::Value;
use serde_json::Value::String as VString;
pub mod api_context;
pub mod auth;
pub mod loaders;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Schemas {
    pub tables: Vec<Schema>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Schema {
    name: String,
    extension: String,
    schema: TableSchemaFetch,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct TableSchemaFetch {
    columns: Vec<ColumnItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnItem {
    name: String,
    #[serde(deserialize_with = "deserialize_data_type")]
    data_type: DataType,
    nullable: Option<bool>,
}

fn deserialize_data_type<'de, D>(deserializer: D) -> Result<DataType, D::Error>
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

impl From<TableSchemaFetch> for TableSchema {
    fn from(value: TableSchemaFetch) -> Self {
        let columns: Vec<TableColumn> = value
            .columns
            .iter()
            .map(|c| TableColumn {
                name: c.name.clone(),
                data_type: c.data_type.clone(),
                nullable: c.nullable.unwrap_or(false),
            })
            .collect();
        Self { columns }
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

                Ok(DataType::Decimal128(precision? as u8, scale? as u8))
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

pub mod graph;
pub mod rest;
pub mod schema;
pub mod sql;
pub mod routes;

use columnq::table::TableLoadOption;
use columnq::table::TableSource;

pub fn get_table_source(table: &str) -> TableSource {
    let table = table.split(".").collect::<Vec<&str>>();
    let table_name = table[0];
    let extention = table[1];
    let mut map = serde_json::Map::new();
    map.insert(
        String::from("format"),
        serde_json::Value::String(extention.to_owned()),
    );
    map.insert(
        String::from("use_memory_table"),
        serde_json::Value::Bool(false)
    );
    let opt: TableLoadOption = serde_json::from_value(serde_json::Value::Object(map)).unwrap();
    TableSource::new(
        table_name,
        format!(
            "./test/{}",
            table_name.clone().trim().to_owned() +"." + extention
        ),
    )
    .with_option(opt)
}

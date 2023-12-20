use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub fn read_file(fname: &str) -> Map<String, Value> {
    // let mut file = File::open(fname.to_string()).unwrap();
    let mut file = File::open(fname.to_string()).expect("errrrr opening file");

    let mut data = String::new();
    
    let ssiz = file.read_to_string(&mut data).unwrap();
    println!("read count is: {}", ssiz);
    if ssiz == 0 {
        data = "{}".to_string();
    }
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();

    return state
}

pub fn write_to_file(fname: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(fname.to_string(), new_data.to_string()).expect("Unable to write file");
}



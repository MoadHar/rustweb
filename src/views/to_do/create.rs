use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::processes::process_input;


pub async fn create(req: HttpRequest) -> String {
    //1. reading json file content a json Map
    let state: Map<String, Value> = read_file("./state.json");

    //2. get title input from request
    let title: String = req.match_info().get("title")
        .unwrap().to_string();

    //3. get the item
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);
    
    //4.
    process_input(item, "create".to_string(), &state);

    //5.
    return format!("{} created", title);
}

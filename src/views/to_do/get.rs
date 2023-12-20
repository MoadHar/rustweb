use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;

pub async fn get() -> impl Responder {
//pub async fn get() -> Map<String, Value> {
     let state: Map<String, Value> = read_file("./state.json");
     return web::Json(state);
     //return state;
 }

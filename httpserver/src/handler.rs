use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;


pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(&self, file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}


pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebSocketHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    status: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, None)
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let router: Vec<&str> = s.split("/").collect();
        match router[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Some("OK")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    }else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => PageNotFoundHandler::handle(req),
            },
        }
    }
}

impl  WebSocketHandler {
   fn load_json(json_path: &str) -> Vec<OrderStatus> {
    let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let full_path = format!("{}/{}", default_path, json_path);
    let contents = fs::read_to_string(full_path).expect("Unable to read file");
    serde_json::from_str(&contents).expect("Unable to parse JSON")
   }
}


impl Handler for WebSocketHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("200", None, Some("OK"))
    }
}
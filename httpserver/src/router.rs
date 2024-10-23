use super::handler::{PageNotFoundHandler, StaticPageHandler, WebSocketHandler};
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            http::httprequest::Method::Get => match &req.resource {
                http::httprequest::Resource::Path(s) => {
                    let router:Vec<&str> = s.split("/").collect();
                    match router[1] {
                        "api" => {
                            let resp: HttpResponse = WebSocketHandler::handle(req, stream);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(req, stream);
                            let _ = resp.send_response(stream);
                        }
                    }   
                    
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(stream);
                let _ = resp.send_response(stream);
            }
        }
    }
}

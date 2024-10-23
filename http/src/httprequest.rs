use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

// 将字符串转换为方法
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "get" => Method::Get,
            "post" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "http/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl HttpRequest {
    fn from_request(s: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path(String::from(""));
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = String::from("");

        for line in s.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = parse_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = parse_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.is_empty() {
                continue;
            } else {
                parsed_msg_body = line.to_string();
            }
        }
        
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
    
}

fn parse_request_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = Method::from(words.next().unwrap());
    let resource = Resource::Path(words.next().unwrap().to_string());
    let version = Version::from(words.next().unwrap());

    (method, resource, version)
}

fn parse_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let key = header_items.next().unwrap().to_string();
    let value = header_items.next().unwrap().to_string();

    (key, value)
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    // #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    // #[test]
   fn test_read_http(){
    let s = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello, world!\"}");
    let mut headers_expected = HashMap::new();
    headers_expected.insert(String::from("Host"), String::from("localhost:3000"));
    headers_expected.insert(String::from("Accept"), String::from("*/*"));
    headers_expected.insert(String::from("User-Agent"), String::from("curl/7.55.1"));

    let req = HttpRequest::from_request(s);
    
    assert_eq!(req.method, Method::Get);
    assert_eq!(req.version, Version::V1_1);
    assert_eq!(req.resource, Resource::Path(String::from("/greeting")));
    assert_eq!(req.headers, headers_expected);
    assert_eq!(req.msg_body, "{\"message\": \"Hello, world!\"}");
   }
}
 


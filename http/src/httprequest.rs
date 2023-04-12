use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    Http10,
    Http11,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::Http11,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub resource: Resource,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_version = Version::Http11;
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = parse_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = parse_header_line(line);
                parsed_headers.insert(key, value);
            } else if line == "" {
                parsed_body = "";
            } else {
                parsed_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            resource: parsed_resource,
            version: parsed_version,
            headers: parsed_headers,
            body: parsed_body.to_string(),
        }
    }
}

fn parse_request_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap().into();
    let resource = Resource::Path(words.next().unwrap().to_string());
    let version = words.next().unwrap().into();

    (method, resource, version)
}

fn parse_header_line(line: &str) -> (String, String) {
    let mut words = line.split(": ");
    let key = words.next().unwrap().to_string();
    let value = words.next().unwrap().to_string();

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_method_from() {
        assert_eq!(Method::from("GET"), Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::Http11);
    }

    #[test]
    fn test_version_from() {
        assert_eq!(Version::from("HTTP/1.1"), Version::Http11);
    }

    #[test]
    fn test_read_http_request() {
        let s = "GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n\r\n".to_string();
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".to_string(), "localhost:3000".to_string());
        headers_expected.insert("User-Agent".to_string(), "curl/7.54.0".to_string());
        headers_expected.insert("Accept".to_string(), "*/*".to_string());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(Version::Http11, req.version);
        assert_eq!(headers_expected, req.headers);
        assert_eq!("".to_string(), req.body);
    }
}

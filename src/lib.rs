pub fn parse_request(http_request: Vec<String>) -> (String, u16) {
    let request_line = &http_request[0]; // "GET http://httpbin.org/ip HTTP/1.1"
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let url = parts[1]; // This gives us "http://httpbin.org/ip"
    let host_and_path = url.trim_start_matches("http://").trim_start_matches("https://"); 
    let parts: Vec<&str> = host_and_path.split('/').collect();
    let host = parts[0]; // This gives us "httpbin.org"
    let host_parts: Vec<&str> = host.split(':').collect();
    let host_name = host_parts[0]; // This gives us "httpbin.org"
    let port = if host_parts.len() == 2 {
        host_parts[1].parse::<u16>().unwrap() // Parse the port if explicitly stated
    } else {
        if url.starts_with("https://") { 443 } else { 80 } // Default ports
    };
    return (host_name.to_owned(), port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request_http_with_port() {
        let request = vec![
            String::from("GET http://example.com:8080/path HTTP/1.1"),
            String::from("Host: example.com"),
            String::from("User-Agent: test-agent"),
        ];
        let (host, port) = parse_request(request);
        assert_eq!(host, "example.com");
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_parse_request_https_without_port() {
        let request = vec![
            String::from("GET https://example.com/path HTTP/1.1"),
            String::from("Host: example.com"),
            String::from("User-Agent: test-agent"),
        ];
        let (host, port) = parse_request(request);
        assert_eq!(host, "example.com");
        assert_eq!(port, 443);  // HTTPS default port
    }

    #[test]
    fn test_parse_request_http_without_port() {
        let request = vec![
            String::from("GET http://example.com/path HTTP/1.1"),
            String::from("Host: example.com"),
            String::from("User-Agent: test-agent"),
        ];
        let (host, port) = parse_request(request);
        assert_eq!(host, "example.com");
        assert_eq!(port, 80);  // HTTP default port
    }
}
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound => match server {
            Ok(url) => url.to_string(),
            Err(message) => format!("Not found: {}", message),
        },
        Security::UnexpectedUrl => match server {
            Ok(url) => panic!("{}", url),
            Err(message) => message.to_string(),
        },
    }
}
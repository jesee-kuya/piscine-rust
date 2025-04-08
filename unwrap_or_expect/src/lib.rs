pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => match server {
            Ok(file) => file.to_string(),
            Err(_) => panic!(),
        },
        Security::Message => match server {
            Ok(file) => file.to_string(),
            Err(_) => panic!("ERROR: program stops"),
        },
        Security::Warning => match server {
            Ok(file) => file.to_string(),
            Err(_) => "WARNING: check the server".to_string(),
        },
        Security::NotFound => match server {
            Ok(file) => file.to_string(),
            Err(e) => "Not found: ".to_string() + e,
        },
        Security::UnexpectedUrl => match server {
            Ok(file) => panic!("{file}"),
            Err(e) => e.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

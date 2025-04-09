pub use std::collections::HashMap;
pub use chrono::{ Utc };

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: HashMap<String, String>,
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let mut map = HashMap::new();
        map.insert(field_name.to_string(), field_value.to_string());
        let utc = format!("{}",Utc::now().format("%Y-%m-%d %H:%M:%S"));

        Self {
            form_values: map,
            date: utc,
            err: err.to_owned(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: &str, password: &str) -> Self {
        Self {
            name: format!("{name}"),
            password: format!("{password}"),
        }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        let has_alphanum = self.password.chars().any(|c| c.is_alphanumeric());
        let has_numbers = self.password.chars().any(|c| c.is_numeric()); 
        let has_symbol = self.password.chars().any(|c| !c.is_alphanumeric() && !c.is_whitespace());
        if self.name.trim().is_empty() {
            Err(FormError::new("name", format!("{}",self.name), "Username is empty"))
        } else if self.password.len() < 8{
            Err(FormError::new("password", format!("{}", self.password), "Password should be at least 8 characters long"))
        } else if !(has_alphanum && has_symbol && has_numbers) {
            Err(FormError::new("password", format!("{}", self.password), "Password should be a combination of ASCII numbers, letters and symbols"))
        }else {
            Ok(())
        }
    }
}

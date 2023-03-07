use regex::Regex;
use serde::ser::{Serialize, Serializer};

pub struct Line {
    line: String,
}

impl Line {
    pub fn new(line: &str) -> Line {
        return Line {
            line: line.to_string(),
        };
    }

    pub fn get_id(&self) -> Option<String> {
        let re = Regex::new(r"^.*(reqId.:.|request_id=)(.{36}).*$").unwrap();
        return re
            .captures(&self.line)
            .map(|request_id| request_id.get(2).unwrap().as_str().to_string());
    }

    pub fn get_error_message(&self) -> Option<String> {
        let re = Regex::new(r"^.*message.:.(.*?).,.stack.*$").unwrap();
        let Some(err_msg) = re.captures(&self.line) else { return None; };
        let Some(err_msg) = err_msg.get(1) else { return None; };

        return Some(err_msg.as_str().to_string());
    }

    pub fn is_relevant(&self) -> bool {
        let ignorable_errors = vec![
            "\"errorStatus\":400,",
            "\"errorStatus\":401,",
            "\"errorStatus\":404,",
            "\"statusCode\":401,",
            "\"statusCode\":402,",
        ];

        if !self.line.contains("status code 50") {
            return false;
        }

        let contains_ignorable_error = ignorable_errors.iter().any(|e| self.line.contains(e));
        if contains_ignorable_error {
            return false;
        }

        return true;
    }
}

impl Serialize for Line {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_str(&self.line);
    }
}
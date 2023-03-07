use lazy_static::lazy_static;
use regex::Regex;
use serde::ser::{Serialize, Serializer};

lazy_static! {
    static ref REQ_ID: Regex = Regex::new(r"^.*(reqId.:.|request_id=)(.{36}).*$").unwrap();
    static ref ERR_MSG: Regex = Regex::new(r"^.*message.:.(.*?).,.stack.*$").unwrap();
}
pub struct Line<'a> {
    line: &'a str,
}

impl<'a> Line<'a> {
    pub fn new(line: &str) -> Line {
        return Line { line };
    }

    pub fn get_id(&self) -> Option<&str> {
        return REQ_ID
            .captures(&self.line)
            .map(|request_id| request_id.get(2).unwrap().as_str());
    }

    pub fn get_error_message(&self) -> Option<&str> {
        let Some(err_msg) = ERR_MSG.captures(&self.line) else { return None; };
        let Some(err_msg) = err_msg.get(1) else { return None; };

        return Some(err_msg.as_str());
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

impl<'a> Serialize for Line<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_str(&self.line);
    }
}

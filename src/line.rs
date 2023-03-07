use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
};
use serde::ser::{Serialize, Serializer};

#[derive(Clone)]
pub struct Line {
    line: String,
}

impl Line {
    pub fn new(line: &str) -> Line {
        let line = line.to_string();
        return Line { line };
    }

    pub fn get_id(&self) -> Option<&str> {
        let Some((req_id, _)): Option<(&str, &str)> = alt((
            take_until::<&str, &str, nom::error::Error<&str>>("reqId\":\""),
            take_until::<&str, &str, nom::error::Error<&str>>("request_id="),
        ))(self.line.as_str())
        .ok() else { return None;};

        let Some((req_id, _)): Option<(&str, &str)> = alt((
            tag::<&str, &str, nom::error::Error<&str>>("reqId\":\""),
            tag::<&str, &str, nom::error::Error<&str>>("request_id="),
        ))(req_id)
        .ok() else { return None;};

        let Some((_, req_id)): Option<(&str, &str)> =
            take::<usize, &str, nom::error::Error<&str>>(36)(req_id).ok() else { return None;};

        return Some(req_id);
    }

    pub fn get_error_message(&self) -> Option<&str> {
        let Some((err_msg,_)): Option<(&str,&str)> = take_until::<&str, &str, nom::error::Error<&str>>("message\":\"")(self.line.as_str()).ok() else { return None; };
        let Some((err_msg,_)): Option<(&str,&str)> = tag::<&str, &str, nom::error::Error<&str>>("message\":\"")(err_msg).ok() else { return None; };
        let Some((_,err_msg)): Option<(&str,&str)> = take_until::<&str, &str, nom::error::Error<&str>>("\",\"stack")(err_msg).ok() else { return None; };

        return Some(err_msg);
    }

    pub fn is_relevant(&self) -> bool {
        let ignorable_errors = vec![
            "\"errorStatus\":400,",
            "\"errorStatus\":401,",
            "\"errorStatus\":404,",
            "\"statusCode\":401,",
            "\"statusCode\":402,",
        ];

        if !self.line.contains("level\":50") {
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

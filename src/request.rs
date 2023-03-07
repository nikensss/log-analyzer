use crate::line::Line;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Request {
    lines: Vec<Line>,
    id: Option<String>,
    error_message: Option<String>,
}

impl Request {
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);

        if self.id.is_none() {
            if let Some(id) = self.get_id() {
                self.id = Some(id);
            }
        }

        if self.error_message.is_none() {
            if let Some(err_msg) = self.get_error_message() {
                self.error_message = Some(err_msg);
            }
        }
    }

    fn get_id(&self) -> Option<String> {
        if self.id.is_some() {
            return self.id.clone();
        }

        if let Some(line) = &self.lines.last() {
            if let Some(request_id) = line.get_id() {
                return Some(request_id);
            }
        }

        return None;
    }

    fn get_error_message(&self) -> Option<String> {
        if self.error_message.is_some() {
            return self.error_message.clone();
        }

        for line in &self.lines {
            if let Some(err_msg) = line.get_error_message() {
                return Some(err_msg.to_string());
            }
        }

        None
    }

    pub fn new() -> Request {
        return Request {
            lines: Vec::new(),
            id: None,
            error_message: None,
        };
    }
}

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Request", 3)?;
        state.serialize_field("id", &self.get_id())?;
        state.serialize_field("error_message", &self.get_error_message())?;
        state.serialize_field("logs", &self.lines)?;

        state.end()
    }
}

use crate::line::Line;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Request {
    lines: Vec<Line>,
}

impl Request {
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    fn get_id(&self) -> Option<&str> {
        if let Some(line) = &self.lines.last() {
            if let Some(request_id) = line.get_id() {
                return Some(request_id);
            }
        }

        return None;
    }

    fn get_error_message(&self) -> Option<&str> {
        for line in &self.lines {
            if line.get_error_message().is_some() {
                return line.get_error_message();
            }
        }

        None
    }

    pub fn new() -> Request {
        return Request { lines: Vec::new() };
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

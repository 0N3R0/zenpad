// Imports: --- region ---

use serde::Serialize;
use uuid::Uuid;

// Imports: --- endregion ---


// Note object: --- region ---

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    title: String,
    content: String,
    uuid: String,
}

impl Note {
    pub fn new(title: &str, content: &str) -> Note {
        return Self {
            title: title.to_string(),
            content: content.to_string(),
            uuid: Uuid::new_v4().to_string()
        };
    }

    pub fn display_note_information(&self) -> () {
        println!(
            "UUID: {}\nTitle: {}\nContent: {}",
            self.get_uuid(),
            self.get_title(),
            self.get_content()
        );
    }

    pub fn set_title(&mut self, title: &str) -> () {
        self.title = title.to_string();
    }
    
    pub fn set_content(&mut self, content: &str) -> () {
        self.content = content.to_string();
    }

    pub fn get_title(&self) -> &str {
        return &self.title;
    }

    pub fn get_content(&self) -> &str {
        return &self.content;
    }

    pub fn get_uuid(&self) -> &str {
        return &self.uuid;
    }
}

// Note object: --- endregion ---
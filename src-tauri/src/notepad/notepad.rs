// Imports: --- region ---

use super::types::{ NoteRef, new_note_ref };

// Imports: --- endregion --



#[derive(Debug)]
pub struct Notepad {
    notes: Vec<NoteRef>
}



// Defaults: --- region ---

impl Notepad {
    pub fn new() -> Self {
        return Self {
            notes: vec![]
        };
    }

    pub fn display_all_notes(&self) -> () {
        for note in &self.notes {
            note.lock().unwrap().display_note_information();
        }
    }
}

// Defaults: --- endregion ---



// Getters: --- region ---

impl Notepad {
    pub fn get_all_notes(&self) -> &Vec<NoteRef> {
        return &self.notes;
    }
    
    pub fn get_note(&self, uuid: &str) -> Option<NoteRef> {
        return self.get_note_index(uuid).map(|index| self.notes[index].clone());
    }

    pub fn get_note_index(&self, uuid: &str) -> Option<usize> {
        self.notes.iter().enumerate().find_map(|(i, note)| {
            if note.lock().unwrap().get_uuid() == uuid {
                return Some(i);
            };
            
            return None;
        })
    }
}

// Getters: --- endregion ---



// Modifiers: --- region ---

impl Notepad {
    pub fn create_new_note(&mut self, title: &str, content: &str) -> String {
        let note: NoteRef = new_note_ref(title, content);
        let uuid: String = self.add_new_note(note);

        return uuid;
    }

    pub fn add_new_note(&mut self, note: NoteRef) -> String {
        let passed_note= note.lock().unwrap();

        if self.get_note(passed_note.get_uuid()).is_some() {
            let new_note = self.create_new_note(passed_note.get_title(), passed_note.get_content());

            return new_note;
        }

        self.notes.push(note.clone());

        // return note.borrow().get_uuid().to_string();
        return note.lock().unwrap().get_uuid().to_string();
    }

    pub fn update_note(&mut self, uuid: &str, title: &str, content: &str) -> bool {
        if let Some(note) = self.get_note(uuid) {
            let mut current_note = note.lock().unwrap();
            current_note.set_title(title);
            current_note.set_content(content);

            return true;
        };

        return false;
    }

    pub fn delete_note(&mut self, uuid: &str) -> bool {
        if let Some(index) = self.get_note_index(uuid) {
            self.notes.remove(index);
            return true;
        };

        return false;
    }
}

// Modifiers: --- endregion ---

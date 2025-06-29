// Imports: --- region ---

// use std::rc::Rc;
// use std::cell::RefCell;

use std::sync::{Arc, Mutex};

use super::note::Note;

// Imports: --- endregion ---


// Defining types: --- region ---

// pub type NoteRef = Rc<RefCell<Note>>;
// pub fn new_note_ref (title: &str, content: &str) -> NoteRef {
//     return Rc::new(
//         RefCell::new(
//             Note::new(
//                 title,
//                 content
//             )
//         )
//     );
// }

pub type NoteRef = Arc<Mutex<Note>>;
pub fn new_note_ref (title: &str, content: &str) -> NoteRef {
    return Arc::new(
        Mutex::new(
            Note::new(
                title,
                content
            )
        )
    );
}

// Defining types: --- endregion ---
// Register modules: --- region ---

mod notepad;

// Register modules: --- endregion ---



// Import modules: --- region ---

use notepad::notepad::Notepad;
use notepad::types::NoteRef;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Import modules: --- endregion ---



// Create function commands: --- region ---

static GLOBAL_NOTEPAD: Lazy<Mutex<Notepad>> = Lazy::new(|| Mutex::new(Notepad::new()));

#[tauri::command]
fn get_all_notes() -> Vec<NoteRef> {
    let notepad = GLOBAL_NOTEPAD.lock().unwrap();
    return notepad.get_all_notes().clone();
}

#[tauri::command]
fn create_new_note(title: &str, content: &str) -> String {
    let mut notepad = GLOBAL_NOTEPAD.lock().unwrap();
    notepad.create_new_note(title, content)
}

#[tauri::command]
fn update_note(uuid: &str, title: &str, content: &str) -> bool {
    let mut notepad = GLOBAL_NOTEPAD.lock().unwrap();

    return notepad.update_note(uuid, title, content);
}

#[tauri::command]
fn delete_note(uuid: &str) -> bool {
    let mut notepad = GLOBAL_NOTEPAD.lock().unwrap();
    
    return notepad.delete_note(uuid);
}

#[tauri::command]
fn greet(name: &str) -> String {
    let a: String = String::from("Hello name, wass up!");

    return a.replace("name", name);
}

// Create function commands: --- endregion ---


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let mut notepad = GLOBAL_NOTEPAD.lock().unwrap();

    // let uuid: String = notepad.create_new_note("WTF", "THE FUCK AM I DOING HERE?");

    // dbg!("{:?}", &uuid);
    // dbg!("{:?}", notepad.get_all_notes());

    // let result = notepad.update_note(&uuid, "WAWAWAWA, WASSAP!", "HELLO THERE!");

    // let some_note = notepad.get_note(&uuid);

    // dbg!("{:?}", &result);
    // dbg!("{:?}", notepad.get_note(&uuid));

    // if let Some(note) = some_note {
    //     notepad.add_new_note(note);
    // };

    // dbg!("{:?}", notepad.get_all_notes());


    
    // tauri::Builder::default()
    // .plugin(tauri_plugin_opener::init())

    // .invoke_handler(tauri::generate_handler![create_note, update_note, delete_note, get_all_notes])

    // .run(tauri::generate_context!())
    // .expect("error while running tauri application");



    // This code was generated on start
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

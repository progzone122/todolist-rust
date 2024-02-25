use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use serde_json::{Value, json};

struct ToDoList {
    path: std::path::PathBuf
}

impl ToDoList {
    fn list_notes(&self) -> Value {
        let database_main: String = fs::read_to_string(&self.path)
            .expect("Should have been able to read the file");
        let parsed: Value = serde_json::from_str(&database_main)
            .expect("Failed to parse JSON data");
        let notes = &parsed["notes"];
        if let Some(notes) = notes.as_array() {
            println!("#    Note name");
            for (index, note) in notes.iter().enumerate() {
                println!("{}    {}", index, note["header"]);
            }
        }
        parsed
    }

    fn get_note(&self, notes: &Value, index: usize) {
        if let Some(note) = notes["notes"].get(index) {
            println!("\nHEADER---------------");
            println!("{}", note["header"]);
            println!("TEXT-----------------");
            println!("{}\n", note["text"]);
        } else {
            println!("[ERROR] Note does not exist");
        }
    }

    fn add_note(&self, header: String, text: String) {
        let database_main: String = fs::read_to_string(&self.path)
            .expect("Should have been able to read the file");
        let mut parsed: Value = serde_json::from_str(&database_main)
            .expect("Failed to parse JSON data");
        let new_note = json!({
            "header": header,
            "text": text
        });
        parsed["notes"].as_array_mut()
            .expect("Failed to get array")
            .push(new_note);

        fs::write(&self.path, serde_json::to_string(&parsed).expect("Failed to serialize JSON data"))
            .expect("Failed to write JSON data back to the file");
    }

    fn remove_note(&self, notes: &mut Value, index: usize) {
        if let Some(notes_array) = notes["notes"].as_array_mut() {
            if index < notes_array.len() {
                notes_array.swap_remove(index);
                println!("Element removed: {}", notes["notes"]);
            } else {
                println!("Index is out of bounds");
            }
        } else {
            println!("Key 'notes' is not an array");
        }
    }
}

fn main() {
    let database_path = Path::new("src/database.json");
    let td = ToDoList {
        path: database_path.to_path_buf()
    };
    let mut notes: Value;
    loop {
        notes = td.list_notes();
        println!("(a) Add a note");
        println!("(e) Edit a note");
        println!("(r) Remove a note\n");
        get_input(&td, &mut notes);
    }
}

fn clear_terminal() {
    #[cfg(target_os = "windows")]
    let clear_cmd = "cls";

    #[cfg(not(target_os = "windows"))]
    let clear_cmd = "clear";

    Command::new(clear_cmd)
        .status()
        .expect("Failed to clear terminal");
}

fn get_input(td: &ToDoList, mut notes: &mut Value) {
    println!("Select an option: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim() {
                "a" => {
                    let mut input_header = String::new();
                    let mut input_text = String::new();

                    println!("\nHeader:");
                    let _ = io::stdin().read_line(&mut input_header);
                    println!("\nText:");
                    let _ = io::stdin().read_line(&mut input_text);

                    td.add_note(input_header.trim().to_string(), input_text.trim().to_string())
                },
                "r" => {
                    println!("\nSelect the note index to remove:");

                    let mut input = String::new();

                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            if let Ok(index) = input.trim().parse::<usize>() {
                                clear_terminal();
                                td.remove_note(&mut notes, index);
                            } else {
                                eprintln!("Invalid input: please enter a valid index");
                            }
                        }
                        _ => {}
                    }
                
                },
                _ => {
                    if let Ok(index) = input.trim().parse::<usize>() {
                        clear_terminal();
                        td.get_note(&notes, index);
                    } else {
                        eprintln!("Invalid input: please enter a valid index");
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
}
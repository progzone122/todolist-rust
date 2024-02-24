use std::fs;
use std::path::Path;
use std::io;
struct ToDoList {
    path: std::path::PathBuf
}
impl ToDoList {
    fn list_notes(&self) -> json::JsonValue {
        let database_main: String = fs::read_to_string(&self.path)
            .expect("Should have been able to read the file");
        let parsed: json::JsonValue = json::parse(&database_main).unwrap();
        let notes = &parsed["notes"];
        if let json::JsonValue::Array(notes) = notes {
            println!("{}", "#    Note name");
            for (index, note) in notes.iter().enumerate() {
                println!("{}    {}", index, note["header"]);
            }
        }
        parsed
    }
    fn get_note(&self, notes: &json::JsonValue, index: usize) {
        if !notes["notes"][index].is_null() {
            println!("\nHEADER---------------");
            println!("{}", notes["notes"][index]["header"]);
            println!("TEXT-----------------");
            println!("{}\n", notes["notes"][index]["text"]);
        } else {
            println!("[ERROR] Note does not exist");
        }
    }
}
fn main() {
    let database_path = Path::new("src/database.json");
    let td = ToDoList {
        path: database_path.to_path_buf()
    };
    let mut notes: json::JsonValue;
    loop {
        notes = td.list_notes();
        get_input(&td, &notes)
    }
}
fn get_input(td: &ToDoList, notes: &json::JsonValue) {
    println!("Select an option: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Ok(index) = input.trim().parse::<usize>() {
                td.get_note(&notes, index);
            } else {
                eprintln!("Invalid input: please enter a valid index");
            }
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
}
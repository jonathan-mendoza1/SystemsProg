use std::fs::File;
use std::io::{self, Read, Write};

fn reading_from_console(mut filename:String) {
    let mut buffer = String::new();

    print!("File name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    filename = buffer.trim().to_string();
    read_file_linux(filename.to_string());
    buffer.clear();
}

fn create_and_write_to_file(filename: String, content: String) {
    let mut file = File::create(filename.to_string()).unwrap();
    bufffer.clear();

    print!("File content: ")
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap()
    writeln!(file, content.to_string()).unwrap();
}

fn read_file_linux(filename:String) {
    let output = Command::new("cat")
        .arg(filename)
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", 
    String::from_utf8_lossy(&output.stdout));
}

fn main() {
    while true{
        let message = "What do you want to do today? Type 1 to create file, 2 to read from file, 3 to finish the program: ";
        println!("{}", message);
        let choice = reading;
    
        match choice {
            1 => {
                let message = "What file do you want to create? ".to_string();
                let filename = reading_from_console(&message);
                create_and_write_to_file(&filename);
    
            }
            2 => {
                let message = "What file do you want to open? ".to_string();
                let filename = reading_from_console(&message);
                read_file_linux(filename.to_string());
            }
            3 => break,
            _ => {}
        }
    }
}

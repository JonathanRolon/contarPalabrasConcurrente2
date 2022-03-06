use std::io;

pub mod file_handler;
use file_handler::file_handler::contar_palabras;

fn main() -> io::Result<()> {

    let mut file_names:Vec<String> = Vec::new();

    file_names.push("C:\\Users\\jona_\\OneDrive\\Escritorio\\Rust\\archivo1.txt".to_string());
    file_names.push("C:\\Users\\jona_\\OneDrive\\Escritorio\\Rust\\archivo2.txt".to_string());

    contar_palabras(file_names);

    Ok(())
}

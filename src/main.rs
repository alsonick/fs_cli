use std::fs::{File, self};
use std::io;
use colored::*;
use std::io::prelude::*;

struct FileType {
    filename: String,
    extension: String,
}

fn main() {
    println!("Create files.");

    let mut create_or_delete = String::new();
    println!("Would you like to create or delete a file? ('create' or 'delete')");
    io::stdin()
        .read_line(&mut create_or_delete)
        .expect("Something went wrong");

    if create_or_delete.to_lowercase().trim() == "create" {
        let filename = get_filename_input();
        let extension = get_extension_input();
    
        let file = FileType { filename, extension };
        match create_file(file.filename, file.extension) {
            Ok(res) => {
                let colored_text = format!("Created {}.{}", res.filename.trim(), res.extension.trim())
                    .green();
                println!("{}", colored_text);
            },
            Err(err) => println!("Something went wrong: {}", err)
        }
    } else if create_or_delete.to_lowercase().trim() == "delete" {
        let mut other_operations_choice = String::new();
        println!("Would you like to do delete the file?");
        io::stdin()
            .read_line(&mut other_operations_choice)
            .expect("Something went wrong");
        
        if other_operations_choice.to_lowercase().trim() == "yes" {
            let mut delete_file_choice = String::new();
            println!("Are you sure?");
            
            io::stdin()
                .read_line(&mut delete_file_choice)
                .expect("Something went wrong");
    
            if delete_file_choice.to_lowercase().trim() == "yes" {
                let filename = get_filename_input();
                let extension = get_extension_input();
    
                delete_file(filename, extension);
            }
        }
    }
}

fn get_filename_input() -> String {
    let mut filename = String::new();
    println!("Filename:");
    io::stdin()
        .read_line(&mut filename)
        .expect("Something went wrong");
    filename
}

fn get_extension_input() -> String {
    let mut extension = String::new();
    println!("Extension:");
    io::stdin()
        .read_line(&mut extension)
        .expect("Something went wrong");
    extension
}

fn create_file(filename: String, extension: String) -> std::io::Result<FileType> {
    let file_string = format!("{}.{}", filename.trim(), extension.trim());
    let mut file = File::create(file_string)?;
    file.write_all(b"Hello, world")?;
    let todo = FileType { filename, extension };
    Ok(todo)
}

fn delete_file(filename: String, extension: String) {
    let file_string = format!("{}.{}", filename.trim(), extension.trim());
    let file = File::open(file_string.clone());

    match file {
        Ok(_) => {
            fs::remove_file(file_string)
                .expect(&"Operation failed".red());
            let colored_text = format!("Successfully deleted {}.{}", filename.trim(), extension.trim())
                .green();
            println!("{}", colored_text);
        },
        Err(err) => {
            let colored_text = format!("Something went wrong: {}", err).red();
            println!("{}", colored_text);
        }
    }
}
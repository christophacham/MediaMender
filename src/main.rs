use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use trash;
use walkdir::WalkDir;

const BATCH_SIZE: usize = 20;

fn main() {
    match prompt_for_path() {
        Some(path) => {
            let mut file_extensions = build_file_extension_map(&path);
            main_menu_loop(&mut file_extensions, &path);
        }
        None => println!("Invalid path or access denied."),
    }
}

fn main_menu_loop(file_extensions: &mut HashMap<String, Vec<PathBuf>>, path: &str) {
    loop {
        match show_main_menu() {
            MainMenuOption::ViewCounts => display_extension_counts(file_extensions),
            MainMenuOption::BrowseFiles => browse_files_by_extension(file_extensions),
            MainMenuOption::DeleteFiles => delete_files_by_extension(file_extensions),
            MainMenuOption::ShowExtensions => show_file_extensions(file_extensions),
            MainMenuOption::Refresh => *file_extensions = build_file_extension_map(path),
            MainMenuOption::Exit => break,
        }
    }
}

enum MainMenuOption {
    ViewCounts,
    BrowseFiles,
    DeleteFiles,
    ShowExtensions,
    Refresh,
    Exit,
}

fn show_main_menu() -> MainMenuOption {
    loop {
        println!("\nMain Menu:");
        println!("1. View file counts by extension");
        println!("2. Browse files by extension");
        println!("3. Delete files by extension");
        println!("4. Show file extensions");
        println!("5. Refresh file extension list");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");
        match choice.trim() {
            "1" => return MainMenuOption::ViewCounts,
            "2" => return MainMenuOption::BrowseFiles,
            "3" => return MainMenuOption::DeleteFiles,
            "4" => return MainMenuOption::ShowExtensions,
            "5" => return MainMenuOption::Refresh,
            "6" => return MainMenuOption::Exit,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn show_file_extensions(file_extensions: &HashMap<String, Vec<PathBuf>>) {
    println!("\nAvailable file extensions:");
    for extension in file_extensions.keys() {
        println!(".{}", extension);
    }
}

fn display_extension_counts(file_extensions: &HashMap<String, Vec<PathBuf>>) {
    println!("\nFile counts by extension:");
    for (extension, files) in file_extensions {
        println!("{}: {} file(s)", extension, files.len());
    }
}

fn browse_files_by_extension(file_extensions: &HashMap<String, Vec<PathBuf>>) {
    let extension = prompt_for_extension();
    if let Some(files) = file_extensions.get(&extension) {
        display_files_in_batches(files);
    } else {
        println!("No files to be found with .{} extension", extension);
    }
}

fn display_files_in_batches(files: &[PathBuf]) {
    for chunk in files.chunks(BATCH_SIZE) {
        for file in chunk {
            println!("{:?}", file);
        }

        // Check if there are more files to display.
        if chunk.len() < BATCH_SIZE {
            break; // Exit the loop if this is the last batch.
        }

        println!("Type 'next' to see more, or 'quit' to return:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim().eq_ignore_ascii_case("quit") {
            break; // Break out of the loop if the user wants to quit.
        }
    }
}

fn delete_files_by_extension(file_extensions: &HashMap<String, Vec<PathBuf>>) {
    let extension = prompt_for_extension();
    match delete_files_of_extension(file_extensions, &extension) {
        Ok(_) => println!("All .{} files have been moved to trash.", extension),
        Err(e) => println!("Error occurred: {}", e),
    }
}

fn delete_files_of_extension(
    file_extensions: &HashMap<String, Vec<PathBuf>>,
    extension: &str,
) -> Result<(), trash::Error> {
    if let Some(files) = file_extensions.get(extension) {
        for file in files {
            println!("Moving to trash: {:?}", file);
            trash::delete(file)?;
        }
    }
    Ok(())
}

fn prompt_for_extension() -> String {
    println!("\nEnter a file extension to process (without the dot):");
    let mut extension = String::new();

    io::stdin()
        .read_line(&mut extension)
        .expect("Failed to read line.");

    extension.trim().to_lowercase()
}

fn build_file_extension_map(path: &str) -> HashMap<String, Vec<PathBuf>> {
    let mut file_extensions = HashMap::<String, Vec<PathBuf>>::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let extension = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            file_extensions
                .entry(extension)
                .or_default()
                .push(path.to_path_buf());
        }
    }

    file_extensions
}

fn prompt_for_path() -> Option<String> {
    println!("Enter the path to scan:");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line.");

    let path = path.trim();
    if Path::new(path).exists() {
        Some(path.to_string())
    } else {
        None
    }
}

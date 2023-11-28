use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

const BATCH_SIZE: usize = 20;

fn main() {
    let path = prompt_for_path();
    let file_extension = build_file_extension_map(&path);
    display_extension_counts(&file_extension);
    browse_files_by_extension(file_extension);
}

fn display_extension_counts(file_extension: &HashMap<String, Vec<PathBuf>>) {
    println!("File counts by extension:");
    for (extension, files) in file_extension {
        println!("{}: {} file(s)", extension, files.len());
    }
}

fn browse_files_by_extension(file_extensions: HashMap<String, Vec<PathBuf>>) {
    loop {
        let extension = prompt_for_extension();
        if extension == "exit" {
            break;
        }

        if let Some(files) = file_extensions.get(&extension) {
            display_files_in_batches(files);
        } else {
            println!("No files to be found with .{} extension", extension);
        }
    }
}

fn display_files_in_batches(files: &[PathBuf]) {
    for chunk in files.chunks(BATCH_SIZE) {
        for file in chunk {
            println!("{:?}", file)
        }
        println!("Press Enter to see more...");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line.");
    }
}

fn prompt_for_extension() -> String {
    println!("Enter a file extension to browser (or 'exit' to quit):");
    let mut extension = String::new();

    io::stdin()
        .read_line(&mut extension)
        .expect("Failed to read line.");

    extension.trim().to_lowercase()
}

fn build_file_extension_map(path: &str) -> HashMap<String, Vec<PathBuf>> {
    let mut file_extensions = HashMap::<String, Vec<PathBuf>>::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let extensions = path.extension().unwrap_or_default().to_string_lossy().to_string();
            file_extensions.entry(extensions).or_default().push(path.to_path_buf())
        }
    }

    for files in file_extensions.values_mut() {
        files.sort();
    }

    file_extensions
}

fn prompt_for_path() -> String {
    println!("Enter the path to scan:");
    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line.");

    path.trim().to_string()
}

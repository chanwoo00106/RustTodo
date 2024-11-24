use std::fs;
use std::io;

fn list_files_and_directories(path: &std::path::Path, depth: usize) -> io::Result<()> {
    if path.is_dir() {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry_path
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("");
            println!("{:indent$}{}", "", file_name, indent = depth);

            if entry_path.is_dir() {
                list_files_and_directories(&entry_path, depth + 2)?;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    println!("{}", current_dir.display());
    list_files_and_directories(&current_dir, 0)?;

    Ok(())
}

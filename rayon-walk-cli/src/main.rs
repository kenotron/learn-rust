use rayon::prelude::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let paths = glob();
    paths.unwrap().par_iter().for_each(|entry_path| {
        
        let contents = fs::read_to_string(Path::new(entry_path)).unwrap();
        println!("{}", contents);

    });
    Ok(())
}

fn glob() -> io::Result<Vec<PathBuf>> {
    let mut stack = vec![PathBuf::from("/Users/ken/workspace/tmp1/packages")];
    let mut collection: Vec<PathBuf> = vec![];

    while let Some(current) = stack.pop() {
        let current_path = Path::new(&current);

        let package_json_path = PathBuf::from(current_path).join("package.json");
        if package_json_path.exists() {
            collection.push(package_json_path);
        }

        let dirs = fs::read_dir(current_path)?;

        for dir in dirs {
            let entry = dir?.path();
            if entry.is_dir() && !entry.to_str().unwrap().contains("node_modules") {
                stack.push(entry.to_path_buf());
            }
        }
    }

    Ok(collection)
}

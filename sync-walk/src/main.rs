use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    glob().unwrap();
}

fn glob<'a>() -> io::Result<()> {
    // Let's replace glob with something I write myself to learn more Rust!
    // let globbed = glob("/Users/ken/workspace/tmp1/packages/**/package.json").unwrap();
    // let entries: Vec<glob::GlobResult> = globbed.collect();

    // println!("entries: {:}", entries.len());

    let mut stack = vec![PathBuf::from("/Users/ken/workspace/tmp1/packages")];

    while let Some(current) = stack.pop() {
        let current_path = Path::new(&current);

        let package_json_path = PathBuf::from(current_path).join("package.json");

        if package_json_path.exists() && package_json_path.is_file() {
            println!("{:}", package_json_path.display());
        }

        let dirs = fs::read_dir(current_path)?;

        for dir in dirs {
            let entry = dir?.path();
            
            if entry.is_dir() {
                stack.push(entry.to_path_buf());
            }
        }
    }

    Ok(())
}

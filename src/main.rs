use std::io::{self, Write};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process;
use clap::Parser;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[command(version, about = "This program deletes all .DS_Store files that exist under the specified directory.", long_about = None)]
struct Cli {
    // Program argument #1
    #[arg(value_name = "Target Directory", help = "File search root. Default: current directory")]
    tdir: Option<String>,

    // Sets a custom config file
    #[arg(short, long, help = "Remove files without confirmation.")]
    force: bool
}

fn is_hidden_or_ignored(entry: &DirEntry) -> bool {
    if entry.depth() == 0 {
        return false;
    }

    entry.file_name()
         .to_str()
         .map(|s| {
             if s == ".DS_Store" { return false; }
             s.starts_with('.') || s == "node_modules" || s == "target"
         })
         .unwrap_or(false)
}

fn main() {
    let args = Cli::parse();

    let target_dir = args.tdir
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    // check if the path exists
    if !target_dir.exists() {
        eprintln!("Error: Directory '{}' does not exist.", target_dir.display());
        process::exit(1);
    }
    
    // check if the path is directory
    if !target_dir.is_dir() {
        eprintln!("Error: '{}' is not a directory.", target_dir.display());
        process::exit(1);
    }

    // find files to delete
    let mut files_to_delete = Vec::new();

    let walker = WalkDir::new(&target_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden_or_ignored(e)) {
        if let Ok(entry) = entry {
            let file_name = entry.path().file_name().unwrap_or(&OsStr::new(""));

            if file_name == ".DS_Store" {
                if !args.force {
                    println!("DETECT : {}", entry.path().display());
                }
                files_to_delete.push(entry.path().to_path_buf());
            }
        }
    }

    // delete files
    if !files_to_delete.is_empty() {

        let mut del_flag = false;
        if !args.force {
            print!("Are you sure you want to delete these files? (y/N): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.eq_ignore_ascii_case("y") {
                del_flag = true;
            }
        } else {
            del_flag = true;
        }

        if del_flag {
            for file_path in files_to_delete {
                if let Err(e) = std::fs::remove_file(&file_path) {
                    eprintln!("Failed to delete {}: {}", file_path.display(), e);
                }
            }
            println!("Files deleted successfully.");
        } else {
            println!("File deletion canceled.");
        }
    } else {
        println!("There is no .DS_Store under the target directory={:?}", target_dir);
    }
}

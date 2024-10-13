use std::io::{self, Write};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process;
use clap::Parser;
use walkdir::WalkDir;

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

fn main() {
    let args = Cli::parse();

    let mut target_dir = PathBuf::from(".");
    if let Some(tdir) = args.tdir {
        target_dir = PathBuf::from(tdir);
    }

    // check if the path exists
    if !target_dir.exists() {
        println!("Error: Directory '{}' does not exist.", target_dir.display());
        process::exit(1);
    }
    
    // check if the path is directory
    if !target_dir.is_dir() {
        println!("Error: '{}' is not a directory.", target_dir.display());
        process::exit(1);
    }

    // find files to delete
    let mut files_to_delete = Vec::new();
    for entry in WalkDir::new(target_dir.clone()) {
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
            files_to_delete.clone().into_iter().for_each(|file_path| {
                std::fs::remove_file(&file_path).unwrap();
            });
            println!("Files deleted successfully.");
        } else {
            println!("File deletion canceled.");
        }
    } else {
        println!("There is no .DS_Store under the target directory={:?}", target_dir);
    }
}

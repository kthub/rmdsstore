use std::io::{self, Write};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process;
use clap::Parser;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[command(version, about = "This program deletes all .DS_Store files that exist under the specified directory.", long_about = None)]
struct Cli {
    #[arg(value_name = "Target Directory", help = "File search root. Default: current directory")]
    tdir: Option<String>,

    #[arg(short, long, help = "Remove files without confirmation.")]
    force: bool,

    #[arg(short = 'n', long, help = "Show files that would be deleted without actually deleting them.")]
    dry_run: bool,

    #[arg(short, long, help = "Suppress non-critical output.")]
    quiet: bool,
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

    if args.force && args.dry_run {
        eprintln!("Error: --force and --dry-run cannot be used together.");
        process::exit(1);
    }

    if !target_dir.exists() {
        eprintln!("Error: Directory '{}' does not exist.", target_dir.display());
        process::exit(1);
    }

    if !target_dir.is_dir() {
        eprintln!("Error: '{}' is not a directory.", target_dir.display());
        process::exit(1);
    }

    // find files to delete
    let mut files_to_delete = Vec::new();

    let walker = WalkDir::new(&target_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden_or_ignored(e)) {
        if let Ok(entry) = entry {
            let file_name = entry.path().file_name().unwrap_or(OsStr::new(""));

            if file_name == ".DS_Store" {
                files_to_delete.push(entry.path().to_path_buf());
            }
        }
    }

    files_to_delete.sort();

    if args.dry_run || !args.force {
        for path in &files_to_delete {
            println!("DETECT : {}", path.display());
        }
    }

    if files_to_delete.is_empty() {
        if !args.quiet {
            println!("There is no .DS_Store under the target directory={}", target_dir.display());
        }
        return;
    }

    // dry-run: just show files, don't delete
    if args.dry_run {
        println!("Dry run: {} file(s) would be deleted.", files_to_delete.len());
        return;
    }

    // determine whether to proceed
    let should_delete = if args.force {
        true
    } else {
        print!("Are you sure you want to delete these files? (y/N): ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Error: failed to flush stdout: {}", e);
            process::exit(1);
        }

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error: failed to read input: {}", e);
            process::exit(1);
        }
        input.trim().eq_ignore_ascii_case("y")
    };

    if !should_delete {
        if !args.quiet {
            println!("File deletion canceled.");
        }
        return;
    }

    // delete files and track results
    let total = files_to_delete.len();
    let mut deleted = 0usize;
    let mut failed = 0usize;

    for file_path in files_to_delete {
        match std::fs::remove_file(&file_path) {
            Ok(()) => deleted += 1,
            Err(e) => {
                eprintln!("Failed to delete {}: {}", file_path.display(), e);
                failed += 1;
            }
        }
    }

    if !args.quiet {
        println!("Deleted {}/{} file(s).", deleted, total);
    }

    if failed > 0 {
        process::exit(1);
    }
}

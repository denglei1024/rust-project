use std::{path, process};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// directory
    dir: String,

    /// find pattern
    pattern: String,

    /// replace pattern
    replace: String,

    /// dry run
    /// if set, will only print the changes without applying them
    /// default: false
    /// short: 'd', long = "dry-run", default_value_t = false
    #[arg(short = 'd', long = "dry-run", default_value_t = false)]
    dry_run: bool,

    /// recursive
    /// if set, will search in subdirectories
    /// default : false
    #[arg(short = 'r', long = "recursive", default_value_t = false)]
    recursive: bool,
}

fn main() {
    let args = Cli::parse();
    // 检查目录是否存在
    let dir = path::Path::new(&args.dir);
    if !dir.exists() || !dir.is_dir() {
        eprintln!("Error: directory '{}', not found or is not a directory", args.dir);
        process::exit(-1);
    }
    if args.pattern.is_empty() {
        eprintln!("Error: pattern is empty");
        process::exit(-1);
    }
    if args.pattern == args.replace {
        eprintln!("Error: pattern and replace are the same");
        process::exit(-1);
    }
    if args.replace.is_empty() {
        eprintln!("Error: replace is empty");
        process::exit(-1);
    }
    let regex = regex::Regex::new(&args.pattern).unwrap();
    walkdir::WalkDir::new(dir)
        .min_depth(1)
        .max_depth(if args.recursive { 3 } else { 1 })
        .into_iter()
        .filter_map(|e| e.ok())
        .for_each(|entry| {
            if !entry.path().is_file() {
                return;
            }
            let file_name = entry.file_name().to_string_lossy();
            if let Some(_) = regex.captures(&file_name) {
                let new_file_name = regex.replace(&file_name, &args.replace as &str);
                println!("rename {:?} to {:?}", entry.path(), new_file_name);
                    //let new_path = entry.path().with_file_name(new_file_name.as_ref());
                    //println!("new_path: {:?}", new_path);
                    // if args.dry_run {
                    //     println!("dry run: rename {:?} to {:?}", entry.path(), new_path);
                    // } else {
                    //     std::fs::rename(entry.path(), new_path).unwrap();
                    // }
            }
        });

}
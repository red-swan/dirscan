use std::env;
// use std::{env,fs,io, path::PathBuf, path::Path};

use std::fs;
use::dirscan::count_comments;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = 
      match args.first() {
        None => panic!("Please add directory as argument to the program"),
        Some(x) => x
      };
    let sol_files: Vec<_> = 
        WalkDir::new(dir)
          .into_iter()
          .filter_map(|x| x.ok())
          .filter(|d| d.file_type().is_file())
          .filter(|d| d.path().extension().filter(|ext| ext.to_str().unwrap() == "sol").is_some())
          .collect();
      
  for file in sol_files {
    let file_text = fs::read_to_string(file.path()).expect("Unable to read file");
    // Not idiomatic
    let count = count_comments(&file_text).unwrap().1;
    println!("Filename: {:?}\t\tComment Count: {}",file, count)
  }
  
    
}
// Write a tool that can traverse a directory, 
// open all rust files, count comment lines and 
// code lines and detect imports (local vs. dependency).




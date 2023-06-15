// use std::{env,fs,io, path::PathBuf, path::Path};

use std::fs;
use::dirscan::*;
use walkdir::WalkDir;

fn main() {
  //   println!("Please specify target directory:");
    let dir = "/home/jared/oz/client-code/Compound/comet_wrapper";
    let sol_files: Vec<_> = 
        WalkDir::new(dir)
          .into_iter()
          .filter_map(|x| x.ok())
          .filter(|d| d.file_type().is_file())
          .filter(|d| d.path().extension().filter(|ext| ext.to_str().unwrap() == "sol").is_some())
          .collect();
      
  for file in sol_files {
    let asd = file.path().extension();
    let s = fs::read_to_string(file.path()).expect("Unable to read file");
    let count = count_comment_lines(&s);
    // println!("{:?}", asd);
    println!("Filename: {:?}\t\tComment Count: {}",file, count)
  }
  
    
}
// Write a tool that can traverse a directory, 
// open all rust files, count comment lines and 
// code lines and detect imports (local vs. dependency).




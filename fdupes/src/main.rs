#![allow(unused)]
use std::io::{Error, ErrorKind};
use std::fs::{self, DirEntry, File};
use std::path::{Path, PathBuf};
use std::env;
use std::process;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
extern crate md5;
use md5;

struct File {
    full_path: PathBuf,
    size: u64,
}
impl File {
    fn new(Path: PathBuf, size: u64) -> File{
        File { full_path: Path, size: size}
    }
    fn getHash(self) -> Option<String>{
        let Path = self.full_path.as_path();
        let mut file = match File::open(Path){
            Err(e) => return None,
            Ok(f)  => f,
        };
        
    }
}
struct FilesList (Vector<File>);
impl FilesList
    if root.is_dir(){
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            match dirs.subdirs.borrow_mut().as_mut(){
                                Some(x) =>  {
                                    x.push(readDir(&entry.path()));
                                    inserts += 1;
                                },
                                None => {},
                            }
                        } else {
                            if let Ok(metadata) = entry.metadata(){
                                dirs.files.push(
                                DUFileStruct{
                                    filepath: entry.path().to_path_buf(),
                                    filesize: metadata.len(),
                                });
                            }
                        }
                    }
                }
            }
        }


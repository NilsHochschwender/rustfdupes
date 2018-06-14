#![allow(unused)]
use std::io::{Error, ErrorKind, Read};
use std::fs::{self, DirEntry, File};
use std::path::{Path, PathBuf};
use std::env;
use std::process;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::iter::FromIterator;
extern crate md5;


#[derive(Debug, Clone)]
struct OwnFile {
    full_path: PathBuf,
    hash: String,
}
impl OwnFile {
    fn new(Path: PathBuf) -> OwnFile{
        OwnFile { full_path: Path, hash: String::from("")}
    }
    fn getHash(&mut self) -> u8{
        let Path = self.full_path.as_path();
        let mut file = match File::open(Path){
            Err(e) => return 1,
            Ok(f)  => f,
        };
        let mut buf: Vec<u8>;
        {
            let mut buffer = [0u8; 2000];
            match file.read(&mut buffer){
                Err(e) => return 2,
                Ok(f)  => {},
            }
            buf = buffer.to_vec();
        }
        self.hash = String::from(format!("{:x}",md5::compute(&buf)));
        0
    }
}
#[derive(Debug)]
struct FilesList {
    files: Vec<OwnFile>,
}
impl FilesList{
    fn new(root: PathBuf) -> FilesList{
        fn new(root: PathBuf) -> Vec<OwnFile>{
            let mut files: Vec<OwnFile> = Vec::new();
            if root.is_dir(){
                if let Ok(entries) = fs::read_dir(root) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Ok(file_type) = entry.file_type() {
                                if file_type.is_dir() {
                                    println!("{:?}",entry);
                                    let mut tmp = new(entry.path().to_path_buf());
                                    for i in tmp.iter_mut(){
                                        files.push(i.clone());
                                    }
                                } else {
                                    println!("{:?}",entry);
                                    let mut tmp = OwnFile::new(entry.path().to_path_buf());
                                    println!("{:?}",tmp);
                                    files.push(tmp);
                                }
                            }
                        }
                    }
                }
            }
            println!("{:?}",files);
            files
        }
        FilesList{files: new(root)}
    }
    fn getHashs(&mut self){
        for file in self.files.iter_mut(){
            let _ = file.getHash();
        }
    }
    fn getPossibleDupes(&mut self) -> FilesWithSameHashVec{
        let mut hashes: Vec<String> = Vec::new();
        for file in self.files.iter(){
            hashes.push(file.hash.to_string());
        }
        hashes.sort();
        let mut hashesNotUniq: Vec<String> = Vec::new();
        for hash in hashes.iter(){
            let mut elements: u64 = 0;
            for j in hashes.iter(){
                if hash == j {
                    elements += 1;
                }
            }
            if elements > 1 {
                hashesNotUniq.push(hash.to_string());
            }
        }
        let mut files = FilesWithSameHashVec{ files_vec: Vec::new()};
        hashesNotUniq.sort();
        hashesNotUniq.dedup();
        for hash in hashesNotUniq.iter(){
            files.files_vec.push(FilesWithSameHash{hash: hash.clone(), files: Vec::new()});
        }
        let mut filess: Vec<OwnFile> = Vec::new();
        for file in self.files.iter(){
            let mut file_clone = file.clone();
            if hashesNotUniq.contains(&file.hash){
                filess.push(file_clone);
            }
        }
        for hashStruct in files.files_vec.iter_mut(){
            for file in filess.iter(){
                let mut file_clone = file.clone();
                if hashStruct.hash == file.hash{
                    hashStruct.files.push(file_clone.full_path);
                }
            }
        }
        drop(self);
        files
    }

}
#[derive(Clone, Debug)]
struct FilesWithSameHash{
    hash: String,
    files: Vec<PathBuf>,
}
impl FilesWithSameHash{
    fn check_dupes(&mut self){
        let mut files: Vec<File> = Vec::new();
        for file in self.files.iter(){
            match File::open(file){
                Err(_) => {},
                Ok(f)  => files.push(f),
            }
        }
        if files.len() > 1 {
            let mut bytes_vec: Vec<Iterator> = Vec::new();
            let mut equal: Vec<bool> = Vec::new();
            for f in files.iter(){
                bytes_vec.push(f.bytes());
                equal.push(true);
            }
            loop{
                let mut vec:Vec<Option<Option<u8>>> = Vec::new();
                for i in 0..bytes_vec.len(){
                    vec.push(match bytes_vec[i].next(){
                        Some(n) => match n{
                            Ok(n)   => Some(Some(*n)),
                            Err(_)  => Some(None),
                        },
                        None    => None,
                    });
                }
                let mut vec_clone = vec.clone();
                vec_clone.dedup();
                if vec_clone.len() != 1{
                    for i in vec_clone.iter(){
                        let mut times = 0u32;
                        for j in vec.iter(){
                            if i == j {
                                times += 1;
                            }
                        }
                        if times == 1{
                            for j in 0..vec.len(){
                                if vec[j] == i{
                                    equal[i] = false;
                                }
                            }
                        }
                    }
                } else if vec_clone.len() == 1 && vec_clone[0].is_none() {
                    break;
                }
            }
            for i in 0..equal.len(){
                
            }
        }
    }    
}
#[derive(Clone, Debug)]
struct FilesWithSameHashVec{
    files_vec: Vec<FilesWithSameHash>,
}
//impl FilesWithSameHashVec{

//}
fn main(){
    let path = match env::current_dir(){
        Err(e) => PathBuf::from(env::home_dir().unwrap()),
        Ok(f)  => f,
    };
    let mut dupes = FilesList::new(path);
    dupes.getHashs();
    println!("{:?}",dupes);
    let mut pos_dupes = dupes.getPossibleDupes();
    println!("{:?}",pos_dupes);

}

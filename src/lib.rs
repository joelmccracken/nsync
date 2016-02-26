extern crate walkdir;
extern crate time;
extern crate crc;

use std::os::unix::raw::time_t;
use std::os::unix::fs::MetadataExt;


#[derive(Debug)]
pub struct FileState {
    path: String,
    mtime: Result<time_t, ()>
}

#[derive(Debug)]
pub struct FSState {
    root: String,
    recorded_at: time::Tm,
    files: Vec<FileState>
}

use self::walkdir::{WalkDir,DirEntry};
pub fn path_to_structure(path: String) -> FSState {
    let file_states = WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map( |e| {
            let path = e.path().to_str().
                unwrap().to_owned();

            let mtime = match e.metadata() {
                Ok(v) => Result::Ok(v.mtime()),
                Err(m) => Result::Err(())
            };

            FileState {
                path: path,
                mtime: mtime
            }
        }).collect::<Vec<_>>();

    FSState { root: path,
              recorded_at: time::now_utc(),
              files: file_states }
}

pub trait Action {
    fn perform(&self) -> Result<(), ()>;
}

struct CopyFile {
    src: FileState,
    dest: FileState
}

impl Action for CopyFile {
    fn perform(&self) -> Result <(), ()> {
        Result::Ok(())
    }
}


pub fn plan_update(src: FSState, dest: FSState) -> Vec<Box<Action>>{
    Vec::new(Box::new(CopyFile { src: "", dest: "" }))
}

#[cfg(test)]
mod test {
    use std::process::Command;
    use std::str;
    use std::fs::File;

    use super::path_to_structure;

    fn mktemp_dir(name: &str) -> String {

        let u8_slice = Command::new("mktemp")
            .arg("-t")
            .arg(name)
            .arg("-d")
            .output()
            .unwrap().stdout;

        String::from(str::from_utf8(&u8_slice)
                     .unwrap()
                     .to_owned())
            .trim()
            .to_string()
    }

    use std::io::prelude::*;



    fn mk_test_box() -> (String, String) {
        let source_dir = mktemp_dir("test_source_directory");

        let sync_to_dir = mktemp_dir("test_sync_dir");

        let mut with_file_name = source_dir.clone();
        with_file_name.push_str("/foo.txt");

        let mut f = File::create(with_file_name).unwrap();
        f.write_all(b"buzz");
        (source_dir, sync_to_dir)
    }


    #[test]
    fn test_converts_structure_to_whatever() {
        let (source_dir, sync_to_dir) = mk_test_box();
        let file_structure = path_to_structure(source_dir);
        println!("{:#?}", file_structure);
    }

    #[test]
    fn test_it_will_back_up_files() {
        let (source_dir, target_dir) = mk_test_box();
        let src_file_structure = path_to_structure(source_dir);

        let target_file_structure = path_to_structure(target_dir);


        println!("{:#?}", src_file_structure);
        println!("{:#?}", target_file_structure);
    }
}

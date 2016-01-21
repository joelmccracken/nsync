extern crate walkdir;

#[derive(Debug)]
pub struct FileState {
    path: String
}

#[derive(Debug)]
pub struct FSState {
    root: String,
    recorded_at: String,
    files: Vec<FileState>
}

use self::walkdir::{WalkDir,DirEntry};
pub fn path_to_structure(path: String) -> FSState {
    let file_states = WalkDir::new(&path).into_iter().
        filter_map(|e| e.ok()).map( |e| FileState { path: e.path().to_str()
                                                    .unwrap().to_owned()
                                                    }).collect::<Vec<_>>();
    FSState { root: path,
              recorded_at: "now lol".to_owned(),
              files: file_states }
}

#[cfg(test)]
mod test {
    use std::process::Command;
    use std::str;
    use std::fs::File;

    use super::path_to_structure;

    fn mktemp(name: &str) -> String {

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

    #[test]
    fn test_converts_structure_to_whatever() {
        let source_dir = mktemp("test_source_directory");

        let sync_to_dir = mktemp("test_sync_dir");

        let mut with_file_name = source_dir.clone();
        with_file_name.push_str("/foo.txt");

        let mut f = File::create(with_file_name).unwrap();
        f.write_all(b"buzz");

        let file_structure = path_to_structure(source_dir);
        println!("{:?}", file_structure);
    }
}

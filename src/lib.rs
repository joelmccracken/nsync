pub struct FileState {
    path: String
}

pub struct FSState {
    root: String,
    recorded_at: String,
    files: Vec<FileState>
}

pub fn path_to_structure(path: String) {

}

#[cfg(test)]
mod test {
    use std::process::Command;
    use std::str;
    use std::fs::File;

    fn mktemp(name: &str) -> String {

        let u8_slice = Command::new("mktemp")
            .arg("-t")
            .arg(name)
            .output()
            .unwrap().stdout;

        String::from(str::from_utf8(&u8_slice).unwrap().to_owned()).trim().to_string()
    }

    #[test]
    fn test_converts_structure_to_whatever() {
        let source_dir = mktemp("test_source_directory");

        let sync_to_dir = mktemp("test_sync_dir");

        let mut with_file_name = source_dir.clone();
        with_file_name.push_str("/foo.txt");

        let mut f = File::create(with_file_name);
    }
}

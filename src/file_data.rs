use std::path::Path;
use std::path::PathBuf;
use std::env::current_dir;
use std::os::unix::fs::FileTypeExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub name: String,
    pub file_type: String,
    pub size: u64,
    pub path: String,
    pub absolute: String
}

impl FileData {
    pub fn from_path(path: &PathBuf) -> FileData {
        let metadata = path.symlink_metadata().expect("metadata call failed");
        let file_type: &str;
        let curr_dir = current_dir().unwrap();
        let absolute = curr_dir.join(&path);
        let absolute = absolute.parent().unwrap();

        if metadata.is_dir() {
            file_type = "dir";
        } else {
            let tmp_type = metadata.file_type();
            if tmp_type.is_file() {
                file_type = "file";
            } else if tmp_type.is_symlink() {
                file_type = "symlink";
            } else if tmp_type.is_block_device() {
                file_type = "block-device";
            } else if tmp_type.is_char_device() {
                file_type = "char-device";
            } else if tmp_type.is_fifo() {
                file_type = "fifo";
            } else if tmp_type.is_socket() {
                file_type = "socket";
            } else {
                file_type = "unknown";
            }
        }

        let file_name: String;
        if let Some(x) = path.file_name() {
            file_name = x.to_str().unwrap().to_owned();
        } else {
            if path == Path::new(".") {
                file_name = curr_dir.file_name().unwrap().to_str().unwrap().to_owned();
            } else {
                let tmp = path.canonicalize().unwrap();
                file_name = tmp.file_name().unwrap().to_str().unwrap().to_owned();
            }
        }

        FileData {
            name: file_name.to_owned(),
            path: path.parent().unwrap().to_str().unwrap().to_owned(),
            absolute: absolute.to_str().unwrap().to_owned(),
            size: metadata.len(),
            file_type: file_type.to_owned()
        }
    }
}

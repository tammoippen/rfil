use glob;

use serde_json;

use file_data;

pub fn list(g: &str, all: bool, recursive: bool) {
    println!("{}", g);
    for entry in glob::glob(g).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file = file_data::FileData::from_path(&path);
                if all || !file.name.starts_with(".") {
                    println!("{}", serde_json::to_string(&file).unwrap());
                    if recursive {
                        list(path.join("*").to_str().unwrap(), all, recursive);
                    }
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

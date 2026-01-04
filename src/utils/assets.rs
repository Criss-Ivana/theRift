use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load_ascii_art(paths: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut assets = HashMap::new();

    for path in paths {
        let curr = fs::read_to_string(&path)
            .expect(&format!("Failed to read asset in {}", path))
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let key = Path::new(&path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        assets.insert(key, curr);
    }

    assets
}
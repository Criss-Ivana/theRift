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

pub enum AsciiOp {
    PermutateLeft,
    PermutateRight,
    RepeatHorizontal(u16),
}

pub fn ascii_op(asset: &Vec<String>, op: AsciiOp) -> Vec<String> {
    match op {
        AsciiOp::PermutateLeft => {
            let mut copy = asset.clone();
            for line in copy.iter_mut() {
                if let Some(first) = line.chars().next() {
                    line.remove(0);
                    line.push(first);
                }
            }
            copy
        }

        AsciiOp::PermutateRight => {
            let mut copy = asset.clone();
            for line in copy.iter_mut() {
                if let Some(last) = line.chars().last() {
                    line.pop();
                    line.insert(0, last);
                }
            }
            copy
        }

        AsciiOp::RepeatHorizontal(n) => asset
            .iter()
            .map(|line| line.repeat(n as usize))
            .collect(),
    }
}

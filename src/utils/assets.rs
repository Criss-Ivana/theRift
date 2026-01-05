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
    PermutateX(i16),
    RepeatX(u16),
}

pub fn ascii_op(asset: &Vec<String>, op: AsciiOp) -> Vec<String> {
    match op {
        AsciiOp::PermutateX(n) => {
            let mut copy = asset.clone();
            for line in copy.iter_mut() {
                let len = line.chars().count() as i16;
                let n = n % len;
                if n > 0 {
                    // scroll right
                    let split = len - n;
                    let (left, right): (Vec<_>, Vec<_>) = line.chars()
                        .enumerate()
                        .partition(|(i, _)| (*i as i16) < split);

                    let new_line: String = right.into_iter().map(|(_, c)| c)
                        .chain(left.into_iter().map(|(_, c)| c))
                        .collect();
                    *line = new_line;
                } else if n < 0 {
                    // scroll left
                    let n = -n;
                    let (left, right): (Vec<_>, Vec<_>) = line.chars()
                        .enumerate()
                        .partition(|(i, _)| (*i as i16) < n);

                    let new_line: String = right.into_iter().map(|(_, c)| c)
                        .chain(left.into_iter().map(|(_, c)| c))
                        .collect();
                    *line = new_line;
                }
            }
            copy
        }
        AsciiOp::RepeatX(n) => asset
            .iter()
            .map(|line| line.repeat(n as usize))
            .collect(),
    }
}

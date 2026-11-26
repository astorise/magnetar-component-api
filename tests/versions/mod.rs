//! The versions released in `wit/`, oldest first.

use std::{fs, path::Path};

pub fn released() -> Vec<String> {
    let mut versions: Vec<(Vec<u32>, String)> =
        fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("wit"))
            .expect("the wit directory")
            .filter_map(|entry| {
                let entry = entry.expect("a directory entry");
                entry.file_type().ok()?.is_dir().then(|| {
                    let name = entry.file_name().into_string().expect("a UTF-8 name");
                    (
                        name.split('.')
                            .map(|part| part.parse().expect("a number"))
                            .collect(),
                        name,
                    )
                })
            })
            .collect();
    versions.sort();
    versions.into_iter().map(|(_, name)| name).collect()
}

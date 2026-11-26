//! The repository's own bookkeeping: what is released is what is recorded,
//! and every released version has its bindings.

use sha2::{Digest, Sha256};
use std::{fs, path::Path};

mod versions;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn wit_of(version: &str) -> String {
    let path = root()
        .join("wit")
        .join(version)
        .join("model-component-graph.wit");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{path:?}: {error}"))
}

#[test]
fn each_directory_holds_the_package_it_is_named_after() {
    for version in versions::released() {
        let declared = format!("package magnetar:model-component-graph@{version};");
        assert!(
            wit_of(&version).lines().any(|line| line.trim() == declared),
            "wit/{version} does not declare `{declared}`"
        );
    }
}

#[test]
fn the_checksums_match_the_released_files() {
    let recorded = fs::read_to_string(root().join("wit/SHA256SUMS")).expect("wit/SHA256SUMS");
    let mut seen = Vec::new();
    for line in recorded.lines() {
        let (sum, path) = line.split_once("  ").expect("`<sha256>  <path>`");
        let bytes =
            fs::read(root().join("wit").join(path)).unwrap_or_else(|e| panic!("{path}: {e}"));
        let computed: String = Sha256::digest(&bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(
            computed, sum,
            "wit/{path} differs from its recorded checksum: a released version never changes"
        );
        seen.push(path.to_string());
    }
    for version in versions::released() {
        let path = format!("{version}/model-component-graph.wit");
        assert!(seen.contains(&path), "wit/{path} has no recorded checksum");
    }
}

#[test]
fn each_released_version_has_its_feature_and_its_bindings() {
    let manifest = fs::read_to_string(root().join("Cargo.toml")).expect("Cargo.toml");
    let library = fs::read_to_string(root().join("src/lib.rs")).expect("src/lib.rs");
    for version in versions::released() {
        let mut parts = version.split('.');
        let feature = format!("v{}_{}", parts.next().unwrap(), parts.next().unwrap());
        assert!(
            manifest.contains(&format!("\n{feature} = [")),
            "no feature `{feature}` for {version}"
        );
        assert!(
            library.contains(&format!("pub mod {feature} {{")),
            "no module `{feature}` for {version}"
        );
        assert!(
            library.contains(&format!("path: \"wit/{version}\"")),
            "module `{feature}` does not read wit/{version}"
        );
    }
}

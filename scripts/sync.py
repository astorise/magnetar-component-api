#!/usr/bin/env python3
"""Keep the crate in step with `wit/`.

Releasing a contract version is: add `wit/<version>/model-component-graph.wit`
and run this script. It derives, from the directories of `wit/`:

* the `vMAJOR_MINOR` features of `Cargo.toml`,
* the module of each version in `src/lib.rs`,
* the line of each version in `wit/SHA256SUMS`.

A version already recorded in `SHA256SUMS` never changes: if its file no longer
matches its recorded checksum the script stops. With `--check` it changes
nothing and fails when a file is out of date (what CI runs).
"""

import hashlib
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
WIT = ROOT / "wit"
FILE = "model-component-graph.wit"


def versions():
    found = []
    for entry in WIT.iterdir():
        if entry.is_dir():
            if not re.fullmatch(r"\d+\.\d+\.\d+", entry.name):
                sys.exit(f"wit/{entry.name} is not a version directory")
            found.append(tuple(int(part) for part in entry.name.split(".")))
    return [".".join(map(str, v)) for v in sorted(found)]


def feature(version):
    major, minor, patch = version.split(".")
    if patch != "0":
        sys.exit(f"{version}: the contract's versions are MAJOR.MINOR.0")
    return f"v{major}_{minor}"


def sha256(version):
    return hashlib.sha256((WIT / version / FILE).read_bytes()).hexdigest()


def checksums(all_versions):
    recorded = {}
    sums = WIT / "SHA256SUMS"
    if sums.exists():
        for line in sums.read_text().splitlines():
            digest, path = line.split("  ", 1)
            recorded[path] = digest
    lines = []
    for version in all_versions:
        path = f"{version}/{FILE}"
        digest = sha256(version)
        if path in recorded and recorded[path] != digest:
            sys.exit(f"wit/{path} differs from its recorded checksum: a released version never changes")
        declared = f"package magnetar:model-component-graph@{version};"
        if declared not in (WIT / version / FILE).read_text().splitlines():
            sys.exit(f"wit/{path} does not declare `{declared}`")
        lines.append(f"{digest}  {path}")
    return "\n".join(lines) + "\n"


def manifest(all_versions, text):
    block = "".join(f'{feature(v)} = ["dep:wit-bindgen"]\n' for v in all_versions)
    return re.sub(
        r"(# BEGIN versions\n).*?(# END versions\n)",
        lambda m: m.group(1) + block + m.group(2),
        text,
        flags=re.S,
    )


def library(all_versions):
    names = [feature(v) for v in all_versions]
    out = (ROOT / "scripts" / "lib.header.rs").read_text()
    clash = ",\n    ".join(
        f'all(feature = "{n}", any({", ".join(f"feature = {chr(34)}{o}{chr(34)}" for o in names if o != n)}))'
        for n in names
    )
    out += (
        "\n// The generated export macros are crate-level macros named after the\n"
        "// world, the same in every version, so two versions cannot be generated\n"
        "// in one crate. A build targets one contract version.\n"
        f"#[rustfmt::skip]\n#[cfg(any(\n    {clash}\n))]\n"
        'compile_error!("enable exactly one contract-version feature of magnetar-component-api (a build targets one version of the contract)");\n'
    )
    for v, n in zip(all_versions, names):
        others = ", ".join(f'feature = "{o}"' for o in names if o != n)
        out += f'''
/// Guest bindings of contract {v}.
#[rustfmt::skip]
#[cfg(all(feature = "{n}", not(any({others}))))]
pub mod {n} {{
    wit_bindgen::generate!({{
        path: "wit/{v}",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::{n}",
    }});
}}
'''
    return out


def main():
    check = "--check" in sys.argv[1:]
    all_versions = versions()
    if not all_versions:
        sys.exit("wit/ holds no version")
    wanted = {
        WIT / "SHA256SUMS": checksums(all_versions),
        ROOT / "Cargo.toml": manifest(all_versions, (ROOT / "Cargo.toml").read_text()),
        ROOT / "src" / "lib.rs": library(all_versions),
    }
    stale = [p for p, text in wanted.items() if not p.exists() or p.read_text() != text]
    if check:
        if stale:
            sys.exit("out of date, run scripts/sync.py: " + ", ".join(str(p.relative_to(ROOT)) for p in stale))
        return
    for path in stale:
        path.write_text(wanted[path])
        print("updated", path.relative_to(ROOT))


main()

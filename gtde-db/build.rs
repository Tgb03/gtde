// use std::{fs, path::Path, process::Command};

fn main() {
    /*
    println!("cargo:rerun_if_changed=../resources/.schemas");

    let schema_dir = Path::new("../resources/.schemas");
    let out_dir = Path::new("src/generated");
    let out_file = out_dir.join("schemas.rs");

    fs::create_dir_all(out_dir).unwrap();

    let quicktype_exe = if cfg!(target_os = "windows") {
        "quicktype.cmd"
    } else {
        "quicktype"
    };

    let schema_paths = fs::read_dir(&schema_dir).unwrap().filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            Some(path)
        } else {
            None
        }
    });

    let mut cmd = Command::new(quicktype_exe);
    cmd.arg("--src-lang")
        .arg("schema")
        .arg("-l")
        .arg("rust")
        .arg("--visibility")
        .arg("public")
        .arg("-o")
        .arg(out_file.to_str().unwrap());

    for schema in schema_paths {
        cmd.arg(schema.to_str().unwrap());
    }

    let status = cmd.status().expect("Failed to run quicktype");
    if !status.success() {
        panic!("quicktype failed to build schema generation. Exited with {status}");
    }
    */
}

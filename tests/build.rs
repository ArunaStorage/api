extern crate tonic_build;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_services()?;
    Ok(())
}

fn compile_services() -> Result<(), Box<dyn std::error::Error>> {
    let mut protos: Vec<String> = Vec::new();

    let service_entries = fs::read_dir("aruna/api/storage/services/v1/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "aruna/api/storage/services/v1/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("aruna/api/notification/services/v1/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "aruna/api/notification/services/v1/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }


    let service_entries = fs::read_dir("aruna/api/internal/v1/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "aruna/api/internal/v1/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }


    tonic_build::configure()
        .build_server(true)
        .out_dir("./tests")
        .compile(
            &protos,
            &["./".to_string(), "aruna/api/google".to_string()],
        ).unwrap();
    Ok(())
}
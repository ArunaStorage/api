extern crate tonic_build;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_services()?;
    Ok(())
}

fn compile_services() -> Result<(), Box<dyn std::error::Error>> {
    let mut protos: Vec<String> = Vec::new();

    let service_entries = fs::read_dir("aruna/api/dataproxy/services/v2/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "aruna/api/dataproxy/services/v2/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("aruna/api/hooks/services/v2/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "aruna/api/hooks/services/v2/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("aruna/api/notification_service/services/v2/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "aruna/api/notification/services/v2/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }

    let service_entries = fs::read_dir("aruna/api/storage/services/v2/")?;

    for entry in service_entries {
        let dir = entry?;
        let rel_path = format!(
            "{}{}",
            "aruna/api/storage/services/v2/",
            dir.file_name().to_str().unwrap().to_string()
        );
        protos.push(rel_path);
    }

    tonic_build::configure()
        .build_server(true)
        .out_dir("./tests")
        .compile(
            &protos,
            &[
                "./".to_string(),
                "aruna/api/google".to_string(),
                "aruna/api/protoc-gen-openapiv2".to_string(),
            ],
        )
        .unwrap();
    Ok(())
}

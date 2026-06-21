use pureos_native_runtime_bootstrap::RuntimeContract;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract = RuntimeContract::v17_39a();
    contract.validate()?;

    let output_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("pureos_native_runtime_evidence.json"));

    let evidence = contract.evidence_json();
    fs::write(&output_path, evidence.as_bytes())?;

    println!("PureOS native runtime bootstrap validation passed.");
    println!("Master: {}", contract.master_version);
    println!("Slice: {}", contract.slice_id);
    println!("Evidence: {}", output_path.display());
    Ok(())
}

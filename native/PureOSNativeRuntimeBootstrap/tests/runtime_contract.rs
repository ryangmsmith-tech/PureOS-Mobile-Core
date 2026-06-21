use pureos_native_runtime_bootstrap::RuntimeContract;

#[test]
fn evidence_reports_cloud_bootstrap_without_runtime_overclaim() {
    let contract = RuntimeContract::v17_39a();
    contract.validate().expect("v17.39A contract must validate");

    let evidence = contract.evidence_json();
    assert!(evidence.contains("\"cloud_native_bootstrap_compiled\": true"));
    assert!(evidence.contains("\"gold_ocean_city_sections_merged\": 6"));
    assert!(evidence.contains("\"native_gpu_frame_produced\": false"));
    assert!(evidence.contains("\"desktop_window_presented\": false"));
}

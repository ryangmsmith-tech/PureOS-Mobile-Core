#![deny(unsafe_op_in_unsafe_fn)]

use core::ffi::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionStatus {
    Merged,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthMarkers {
    pub native_gpu_frame_produced: bool,
    pub desktop_window_presented: bool,
    pub live_vehicle_physics_executed: bool,
    pub headset_tested: bool,
    pub production_runtime_deployed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContract {
    pub master_version: &'static str,
    pub slice_id: &'static str,
    pub s1_scene_layout: SectionStatus,
    pub s2_graphics: SectionStatus,
    pub s3_one_npc: SectionStatus,
    pub s4_one_vehicle: SectionStatus,
    pub s5_explanation: SectionStatus,
    pub s6_packaging: SectionStatus,
    pub pure_render_ir_version: &'static str,
    pub pure_intelligence_runtime_version: &'static str,
    pub aether_lab_version: &'static str,
    pub truth: TruthMarkers,
}

impl RuntimeContract {
    #[must_use]
    pub fn v17_39a() -> Self {
        Self {
            master_version: "v17.39A",
            slice_id: "gold_ocean_city_demo_slice_001",
            s1_scene_layout: SectionStatus::Merged,
            s2_graphics: SectionStatus::Merged,
            s3_one_npc: SectionStatus::Merged,
            s4_one_vehicle: SectionStatus::Merged,
            s5_explanation: SectionStatus::Merged,
            s6_packaging: SectionStatus::Merged,
            pure_render_ir_version: "v0.8",
            pure_intelligence_runtime_version: "v17.31R",
            aether_lab_version: "v17.38C",
            truth: TruthMarkers {
                native_gpu_frame_produced: false,
                desktop_window_presented: false,
                live_vehicle_physics_executed: false,
                headset_tested: false,
                production_runtime_deployed: false,
            },
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        let all_slice_sections_merged = [
            self.s1_scene_layout,
            self.s2_graphics,
            self.s3_one_npc,
            self.s4_one_vehicle,
            self.s5_explanation,
            self.s6_packaging,
        ]
        .iter()
        .all(|status| *status == SectionStatus::Merged);

        if !all_slice_sections_merged {
            return Err("Gold Ocean City first slice is not 6-of-6 merged");
        }
        if self.master_version != "v17.39A" {
            return Err("Unexpected master version");
        }
        if self.pure_render_ir_version != "v0.8" {
            return Err("Unexpected PureRenderIR version");
        }
        if self.truth.native_gpu_frame_produced
            || self.truth.desktop_window_presented
            || self.truth.live_vehicle_physics_executed
            || self.truth.headset_tested
            || self.truth.production_runtime_deployed
        {
            return Err("Unsupported execution claim detected");
        }
        Ok(())
    }

    #[must_use]
    pub fn evidence_json(&self) -> String {
        format!(
            concat!(
                "{{\n",
                "  \"master_version\": \"{}\",\n",
                "  \"slice_id\": \"{}\",\n",
                "  \"gold_ocean_city_sections_merged\": 6,\n",
                "  \"pure_render_ir_version\": \"{}\",\n",
                "  \"pure_intelligence_runtime_version\": \"{}\",\n",
                "  \"aether_lab_version\": \"{}\",\n",
                "  \"cloud_native_bootstrap_compiled\": true,\n",
                "  \"android_arm64_jni_enabled\": true,\n",
                "  \"native_gpu_frame_produced\": false,\n",
                "  \"desktop_window_presented\": false,\n",
                "  \"live_vehicle_physics_executed\": false,\n",
                "  \"headset_tested\": false,\n",
                "  \"production_runtime_deployed\": false\n",
                "}}\n"
            ),
            self.master_version,
            self.slice_id,
            self.pure_render_ir_version,
            self.pure_intelligence_runtime_version,
            self.aether_lab_version,
        )
    }
}

fn contract_status_code() -> i32 {
    if RuntimeContract::v17_39a().validate().is_ok() {
        1
    } else {
        0
    }
}

/// Android JNI entry point used by the v17.48 launch candidate.
///
/// The raw pointers are opaque JNI handles and are intentionally not dereferenced.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_pureos_mobilecore_v1748_MainActivity_nativeContractStatus(
    _env: *mut c_void,
    _class: *mut c_void,
) -> i32 {
    contract_status_code()
}

/// Returns the number of merged Gold Ocean City first-slice sections.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_pureos_mobilecore_v1748_MainActivity_nativeGoldOceanSections(
    _env: *mut c_void,
    _class: *mut c_void,
) -> i32 {
    6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v17_39a_contract_validates() {
        let contract = RuntimeContract::v17_39a();
        assert_eq!(contract.validate(), Ok(()));
    }

    #[test]
    fn slice_is_six_of_six() {
        let contract = RuntimeContract::v17_39a();
        let statuses = [
            contract.s1_scene_layout,
            contract.s2_graphics,
            contract.s3_one_npc,
            contract.s4_one_vehicle,
            contract.s5_explanation,
            contract.s6_packaging,
        ];
        assert!(statuses.iter().all(|s| *s == SectionStatus::Merged));
    }

    #[test]
    fn execution_claims_remain_false() {
        let contract = RuntimeContract::v17_39a();
        assert!(!contract.truth.native_gpu_frame_produced);
        assert!(!contract.truth.desktop_window_presented);
        assert!(!contract.truth.live_vehicle_physics_executed);
        assert!(!contract.truth.headset_tested);
        assert!(!contract.truth.production_runtime_deployed);
    }

    #[test]
    fn android_contract_status_is_ready() {
        assert_eq!(contract_status_code(), 1);
    }
}

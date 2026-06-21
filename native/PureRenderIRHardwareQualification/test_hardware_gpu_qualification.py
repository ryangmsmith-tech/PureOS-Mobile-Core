import tempfile
import unittest
from pathlib import Path

from hardware_gpu_qualification import select_hardware_icd, validate_window_receipt


class HardwareGpuQualificationTests(unittest.TestCase):
    def test_selects_non_software_icd(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "lvp_icd.x86_64.json").write_text("{}", encoding="utf-8")
            hardware = root / "nvidia_icd.json"
            hardware.write_text("{}", encoding="utf-8")
            self.assertEqual(select_hardware_icd(root.glob("*.json")), hardware)

    def test_rejects_software_only_icds(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "lvp_icd.x86_64.json").write_text("{}", encoding="utf-8")
            with self.assertRaises(RuntimeError):
                select_hardware_icd(root.glob("*.json"))

    def test_hardware_receipt_passes(self):
        receipt = {
            "validation": "passed",
            "native_window_created": True,
            "vulkan_surface_configured": True,
            "surface_frame_presentation_produced": True,
            "scripted_camera_smoke_test_executed": True,
            "interactive_keyboard_camera_controls_compiled": True,
            "hardware_gpu_frame_produced": True,
            "surface_frames_presented": 12,
            "object_count": 9,
            "android_window_presented": False,
            "scene_id": "GOC_PLAZA_BLOCKOUT_SCENE_001",
            "capture_sha256": "abc123",
            "camera_start": {"center": [0.0, 0.0]},
            "camera_end": {"center": [1.0, 0.25]},
            "adapter": {
                "backend": "Vulkan",
                "name": "NVIDIA GeForce RTX Test",
                "device_type": "DiscreteGpu",
                "hardware_accelerated": True,
                "software_adapter": False,
            },
        }
        result = validate_window_receipt(receipt)
        self.assertEqual(result["status"], "passed")
        self.assertTrue(result["hardware_gpu_admitted"])

    def test_software_receipt_fails(self):
        receipt = {
            "validation": "passed",
            "native_window_created": True,
            "vulkan_surface_configured": True,
            "surface_frame_presentation_produced": True,
            "scripted_camera_smoke_test_executed": True,
            "interactive_keyboard_camera_controls_compiled": True,
            "hardware_gpu_frame_produced": False,
            "surface_frames_presented": 12,
            "object_count": 9,
            "android_window_presented": False,
            "camera_start": {"center": [0.0, 0.0]},
            "camera_end": {"center": [1.0, 0.25]},
            "adapter": {
                "backend": "Vulkan",
                "name": "llvmpipe",
                "device_type": "Cpu",
                "hardware_accelerated": False,
                "software_adapter": True,
            },
        }
        with self.assertRaises(RuntimeError):
            validate_window_receipt(receipt)


if __name__ == "__main__":
    unittest.main()

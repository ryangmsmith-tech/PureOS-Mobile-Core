#!/usr/bin/env python3
"""PureRenderIR v0.12 hardware-GPU admission and receipt validation."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
from typing import Iterable

SOFTWARE_MARKERS = (
    "lvp",
    "lavapipe",
    "llvmpipe",
    "swiftshader",
    "software",
    "cpu",
)


def is_software_name(value: str) -> bool:
    lowered = value.casefold()
    return any(marker in lowered for marker in SOFTWARE_MARKERS)


def select_hardware_icd(paths: Iterable[Path]) -> Path:
    candidates = sorted(path for path in paths if path.is_file())
    hardware = [path for path in candidates if not is_software_name(path.name)]
    if not hardware:
        names = ", ".join(path.name for path in candidates) or "none"
        raise RuntimeError(f"no non-software Vulkan ICD found; candidates: {names}")
    return hardware[0]


def validate_window_receipt(receipt: dict) -> dict:
    required_true = (
        "native_window_created",
        "vulkan_surface_configured",
        "surface_frame_presentation_produced",
        "scripted_camera_smoke_test_executed",
        "interactive_keyboard_camera_controls_compiled",
        "hardware_gpu_frame_produced",
    )
    for key in required_true:
        if receipt.get(key) is not True:
            raise RuntimeError(f"receipt field {key!r} must be true")

    if receipt.get("validation") != "passed":
        raise RuntimeError("window presentation receipt did not pass")
    if receipt.get("surface_frames_presented", 0) < 12:
        raise RuntimeError("fewer than 12 surface frames were presented")
    if receipt.get("object_count") != 9:
        raise RuntimeError("unexpected Gold Ocean City scene object count")
    if receipt.get("android_window_presented") is not False:
        raise RuntimeError("Android presentation must remain false in Linux qualification")

    adapter = receipt.get("adapter") or {}
    if adapter.get("backend") != "Vulkan":
        raise RuntimeError("adapter backend is not Vulkan")
    if adapter.get("hardware_accelerated") is not True:
        raise RuntimeError("adapter is not marked hardware accelerated")
    if adapter.get("software_adapter") is not False:
        raise RuntimeError("software adapter was admitted")
    if is_software_name(str(adapter.get("name", ""))):
        raise RuntimeError("adapter name indicates a software implementation")
    if str(adapter.get("device_type", "")) not in {
        "DiscreteGpu",
        "IntegratedGpu",
        "VirtualGpu",
    }:
        raise RuntimeError("adapter device type is not hardware-qualified")

    camera_start = (receipt.get("camera_start") or {}).get("center")
    camera_end = (receipt.get("camera_end") or {}).get("center")
    if not camera_start or not camera_end or camera_start == camera_end:
        raise RuntimeError("camera smoke test did not move")

    return {
        "qualification_version": "0.12",
        "status": "passed",
        "hardware_gpu_admitted": True,
        "vulkan_surface_frames_presented": receipt["surface_frames_presented"],
        "adapter": adapter,
        "scene_id": receipt.get("scene_id"),
        "capture_sha256": receipt.get("capture_sha256"),
        "physical_monitor_observed": False,
        "android_window_presented": False,
        "headset_presented": False,
        "production_deployed": False,
    }


def write_json(path: Path, value: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def command_select_icd(args: argparse.Namespace) -> None:
    root = Path(args.directory)
    selected = select_hardware_icd(root.glob("*.json"))
    print(selected)


def command_validate(args: argparse.Namespace) -> None:
    receipt_path = Path(args.receipt)
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    qualification = validate_window_receipt(receipt)
    write_json(Path(args.output), qualification)
    print(json.dumps(qualification, indent=2, sort_keys=True))


def command_environment(args: argparse.Namespace) -> None:
    report = {
        "qualification_version": "0.12",
        "display": os.environ.get("DISPLAY", ""),
        "wayland_display": os.environ.get("WAYLAND_DISPLAY", ""),
        "vulkan_icd_filenames": os.environ.get("VK_ICD_FILENAMES", ""),
        "runner_name": os.environ.get("RUNNER_NAME", ""),
        "runner_os": os.environ.get("RUNNER_OS", ""),
        "runner_arch": os.environ.get("RUNNER_ARCH", ""),
    }
    write_json(Path(args.output), report)
    print(json.dumps(report, indent=2, sort_keys=True))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    select = subparsers.add_parser("select-icd")
    select.add_argument("directory")
    select.set_defaults(func=command_select_icd)

    validate = subparsers.add_parser("validate")
    validate.add_argument("receipt")
    validate.add_argument("output")
    validate.set_defaults(func=command_validate)

    environment = subparsers.add_parser("environment")
    environment.add_argument("output")
    environment.set_defaults(func=command_environment)
    return parser


def main() -> None:
    args = build_parser().parse_args()
    args.func(args)


if __name__ == "__main__":
    main()

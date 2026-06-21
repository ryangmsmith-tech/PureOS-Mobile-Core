# PureRenderIR v0.12 — Hardware GPU Qualification

This package prepares the existing v0.11 native-window runtime for its first physical Vulkan GPU admission test.

## What is ready

- strict rejection of Lavapipe, llvmpipe, SwiftShader, CPU, and other software adapters
- non-software Vulkan ICD selection
- validation of the v0.11 window-presentation receipt
- requirement for at least 12 presented surface frames
- requirement for the nine-object Gold Ocean City plaza scene
- requirement for camera movement during the smoke test
- environment, adapter, capture, and checksum evidence

## Required runner

The execution workflow requires a GitHub self-hosted runner with these labels:

```text
self-hosted
linux
x64
gpu
vulkan
```

The machine must already have:

- a physical Vulkan-capable GPU and working driver
- `vulkaninfo`
- an active X11 or Wayland desktop session
- permission for the runner account to open a window on that session

## Workflows

- `purerenderir_hardware_gpu_readiness.yml` validates the qualification tools on GitHub-hosted infrastructure.
- `purerenderir_hardware_gpu_qualification.yml` performs the real hardware run only when a matching self-hosted runner is connected and manually started.

## Truth boundary

A readiness-workflow pass proves only that the admission and receipt checks are correctly packaged and tested. It does not prove a hardware GPU frame.

A self-hosted qualification pass proves that the Gold Ocean City window runtime selected a non-software Vulkan adapter, configured a surface, presented frames, moved the camera, and generated a hardware-qualified receipt. It does not by itself prove Android presentation, headset output, live vehicle physics, or production deployment.

# PureRenderIR v0.11 — Native Window Presentation and Camera

This package advances the Gold Ocean City plaza from an offscreen frame into a native desktop window path.

## Desktop controls

- `W`, `A`, `S`, `D` or arrow keys: move the camera
- `Q`: zoom out
- `E`: zoom in
- `Escape`: close the window

## Cloud validation

GitHub Actions launches the executable inside an X11 virtual display, configures a Vulkan surface, presents twelve frames, follows a scripted camera path, captures the final view, and generates a presentation receipt.

A passing cloud run proves native window creation and Vulkan surface-frame presentation on the Mesa software Vulkan adapter. It does not prove a physical display, physical hardware GPU, Android presentation, VR output, or production deployment.

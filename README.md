<div align="center">

# Simple Rust 2D Game Engine

[![dependency status](https://deps.rs/repo/github/Overpeek/srs2dge/status.svg)](https://deps.rs/repo/github/Overpeek/srs2dge)
[![build status](https://github.com/Overpeek/srs2dge/actions/workflows/rust.yml/badge.svg)](https://github.com/Overpeek/srs2dge/actions)
[![crates.io](https://img.shields.io/crates/v/srs2dge.svg?label=srs2dge)](https://crates.io/crates/srs2dge)
[![docs.rs](https://docs.rs/srs2dge/badge.svg)](https://docs.rs/srs2dge/)

</div>

### Live demo at: https://gpu.ovpk.net

## Demo images

#### examples/tetris

<img src="/.github/tetris.png"/>

#### examples/platformer

<img src="/.github/platformer.png"/>

#### examples/main

<img src="/.github/main.png"/>

## Runtime env var configs:

#### `PRESENT_MODE`

- `mailbox/mail/sync/mb/m/s` for immediate without tearing
- `fifo/f` for no tearing (fallback)
- `immediate/nosync/im/i` for immediate

#### `WGPU_BACKEND` (one or more comma separated)

- `vulkan/vk` to allow Vulkan
- `dx12/d3d12` to allow DirectX 12
- `dx11/d3d11` to allow DirectX 11
- `metal/mtl` to allow Metal
- `opengl/gles/gl` to allow OpenGL
- `webgpu` to allow WebGPU

#### `WGPU_POWER_PREF`

- `low` to pick 'high efficiency low power GPU'
- `high` to pick 'low efficiency high power GPU'

##### Examples:

- `PRESENT_MODE=mailbox` to use 'vertical sync'
- `WGPU_BACKEND=vulkan,opengl` to allow vulkan and/or opengl to be used
- `WGPU_POWER_PREF=low` to prefer low power video cards

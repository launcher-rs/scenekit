---
name: Bug Report
about: Something is broken or behaving unexpectedly
title: "fix: "
labels: bug
assignees: ""
---

## Description

A clear, one-paragraph description of the bug.

## Minimal Reproduction

```rust
// The smallest possible code that demonstrates the bug.
// Remove everything unrelated.
use scenekit::*;

fn main() {
    // reproduce here
}
```

## Expected Behavior

What you expected to happen.

## Actual Behavior

What actually happened. Include the full error message, panic output, or incorrect visual result.

## Environment

| | |
|---|---|
| scenekit version | `0.x.x` |
| Rust version | `rustc --version` |
| OS | e.g. Pop OS 22.04 / macOS 14 / Windows 11 |
| GPU / Driver | e.g. NVIDIA RTX 3060 / Mesa 23.1 (lavapipe) |
| wgpu backend | e.g. Vulkan / Metal / DX12 / WebGPU |
| Active features | e.g. `default`, `renderer`, `post` |

## Additional Context

Related issues, workarounds you've tried, screenshots, or anything else relevant.

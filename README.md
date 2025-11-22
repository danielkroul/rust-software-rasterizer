An educational 3D graphics pipeline implemented from scratch in [Rust](https://www.rust-lang.org/) on the CPU, as preparation for my Computer Graphics exam.

This project uses [minifb](https://github.com/emoon/rust_minifb) (MIT/Apache 2.0) for window creation and displaying the pixel buffer.

---

## Status

⚠️ **Early-stage learning project**  
This project is primarily for my own education. It is not intended as a library for others to use, but others are welcome to explore the code, learn from it, or give suggestions.

---

## Rough Roadmap / Learning Goals

### Setup & Basics
- [ ] Create a window and framebuffer using `minifb`
- [ ] Implement `set_pixel(x, y, color)`

### 2D Primitives & Rasterization
- [ ] Implement Bresenham’s line algorithm
- [ ] Implement Scanline algorithm for filled triangles

### 3D Data Structures & Coordinates
- [ ] Define `Vector3`, `Vertex`, and `Model` structs
- [ ] Hardcode simple models (e.g., cube)
- [ ] Understand coordinate spaces: Object, World, View, Clip, Screen

### Transformations & Vertex Pipeline
- [ ] Implement MVP (Model-View-Projection) matrix transformations
- [ ] Write a CPU-side “vertex shader” function
- [ ] Perform perspective division and map to screen coordinates

### Depth Handling
- [ ] Implement a Z-Buffer for occlusion
- [ ] Interpolate depth using barycentric coordinates

### Animation & Rotations
- [ ] Apply simple time-based rotations
- [ ] Learn quaternions and SLERP for smooth rotations

### Lighting & Shading
- [ ] Implement Phong local lighting model
- [ ] Support Flat, Gouraud, and Phong shading
- [ ] Handle normals and diffuse/specular/ambient components

### Texturing
- [ ] Extend vertices with UV coordinates
- [ ] Implement texture sampling and mapping
- [ ] Combine textures with Phong shading

### Advanced Features (Optional / Bonus)
- [ ] Shadow mapping (Depth + Shading pass)
- [ ] PBR (Physically-Based Rendering) pipeline
- [ ] CPU-side simulation of GPU shader stages (vertex/fragment)
- [ ] Simple Ray Tracer (spheres, primitives)

### Additional Concepts & Enhancements
- [ ] Physics basics: gravity, collision detection (AABB)
- [ ] Stereoscopy / VR: render two slightly offset views
- [ ] Performance optimization: backface culling, parallelization

This is a work-in-progress; the code and design may change frequently.

---

## Getting Started

> ⚠️ This project uses CPU rendering, which can be extremely slow in debug mode. For a usable framerate, run in release mode:

1. Clone the repository

```bash
git clone https://github.com/danielkroul/rust-software-rasterizer.git
```

2. Build and run

```bash
cargo run --release
```

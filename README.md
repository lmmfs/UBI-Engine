
# UBI Game Engine

A rust game engine that uses opengl in a sdl2 window



## Prerequisites

Install SDL2

Ubuntu

```bash
  sudo apt-get install libsdl2-dev
```


    
## Run Locally

Clone the project

```bash
  git clone https://github.com/lmmfs/UBI-Engine.git
```

Go to the project directory

```bash
  cd UBI-Engine
```

Build the ubi engine library

```bash
  cargo build -p ubi
```

Run the sandbox

```bash
  cd sandbox
```

```bash
  cargo run
```

## Roadmap

- Logging

- Event System

- ECS style system

- Data and Resource manager

- Basic 3d renderer

- Engine scenes

## Rust crates

**gl:** [crate](https://crates.io/crates/gl) for opengl bindings

**sdl2:** [crate](https://crates.io/crates/sdl2) for sdl2 bindings 

**image:** [crate](https://crates.io/crates/image) for image loading

**gltf:** to add [crate](https://crates.io/crates/gltf) for .gltf and .glb files 

**assimp:** to add [crate](https://crates.io/crates/assimp) for assimp library bidings for 3d model loading 

**log:**

**env_logger:**

**thiserror:**


#show link: underline
#heading(outlined: false)[CS 488 Final Project]

Term: Fall 2023

Name: Devin Leamy

UW ID: 20872933

UW User ID: dleamy

#line()

#align(center)[
  #heading(outlined: false)[Eva]
  #heading(outlined: false, level: 3)[_A WebGPU Real-time Ray Tracer Written in Rust_]
]
#line(length: 100%)

#outline(indent: auto)


#set heading(numbering: "1.")

= Overview
#line(length: 100%)

Eva is a real-time ray tracer built in Rust using WebGPU, with an integrated scripting API. 

#pagebreak()
= Features
#line(length: 100%)

== Texture Mapping

Any material can be assigned a texture. Textures are sourced from: `/assets/textures`.

```py
# Load a texture.
texture_handle = Eva.add_texture("texture.png")

# Add the texture to a material.
textured_material = Material(
  1.0,
  0.0,
  (1.0, 1.0, 1.0),
  texture=texture_handle
)

# Add the material to some geometry.
box = Box()
box.set_material(textured_material)
```

#figure(
  stack(dir: ltr)[#image("./assets/texture1.png", width: 50%)][#image("./assets/texture2.png", width: 50%)],
  caption: "Textured Materials"
)



== Skyboxes

Scenes can optionally set a skybox. Skyboxes are sourced from: `/eva/skybox`.
Skyboxes are defined by six images, listed in the order: ["x", "-x", "y", "-y", "z", "-z"], defining
the six faces of a cube.

```py
Eva.add_skybox([
    "clouds/x.png",
    "clouds/-x.png",
    "clouds/y.png",
    "clouds/-y.png",
    "clouds/z.png",
    "clouds/-z.png",
])
```

#figure(
  stack(dir: ltr)[#image("./assets/sky-light.png", width: 50%)][#image("./assets/sky-dark.png", width: 50%)],
  caption: "Day and Night Skyboxes"
)

== Phong Shading

Eva can render `.obj` meshes with triangular faces. If the mesh has vertex normals, Phong Shading is applied.

#figure(
  stack(dir: ltr)[#image("./assets/cat.png", width: 50%)][#image("./assets/catN.png", width: 50%)],
  caption: "Cat Phong Shading"
)

== Real-time Ray Tracing
Eva supports two render modes `RenderStatic` and `RenderDynamic`. Implementing `RenderDynamic` makes your application real-time, and provides `update` and `handle_input` methods.

```py
class Realtime(RenderDynamic):
  def __init__(self):
    super().__init__()

    self.cube = Box()
    self.add_geometry(cube)

  def update(self):
    self.cube.rotate_x(1)
  
  def handle_input(self, key, state):
    # Move the camera left and right in response to input.
    if state == "Pressed" and key == "A":
      self.camera.translate(-1, 0, 0)
    if state == "Pressed" and key == "D":
      self.camera.translate(1, 0, 0)
```

== Reflections
Rays can reflect off of metallic surfaces. The maximum ray reflections can be configured:
```python
Eva.set_max_reflections(100)
```

#figure(
  stack(dir: ltr)[#image("./assets/reflection1.png", width: 50%)][#image("./assets/reflection2.png", width: 50%)],
  caption: "Reflective Surfaces"
)

== Python Scripting
Eva is divided into two core components: `/eva` and `/eva-py`. `/eva-py` defines a scripting 
API for the `/eva` renderer. Scripts are sourced from `/scripts`. 

Scripts can be run using the utilities `run.sh` and `debug.sh`. `debug.sh` will display build logs. 

To run a script, `my-scene.py` execute:

```bash
./debug.sh my-scene
```


== Materials

A material is defined by a `roughness`, `metallicness`, `color`, an optional `texture`, and an optional `emissiveness`. 

```python
ruby = Material(0.1, 1.0, (1, 0.1, 0.1))
blue_light = Material(0, 0, (0, 0, 1), light=(0.0, 0.0, 1.0))

earth_handle = Eva.add_texture("earth.jpeg")
earth = Material(0, 0, (1, 1, 1), texture=earth_handle)
```

#figure(
  stack(dir: ltr)[#image("./assets/mat1.png", width: 50%)][#image("./assets/mat2.png", width: 50%)],
  caption: "Different Kinds of Materials"
)

= Technical Overview
#line(length: 100%)

== Project Structure
- `/eva`: Core renderer.
- `/eva-macros`: Marcos for the core renderer.
- `/eva-py`: Python3 scripting API.
- `/eva-py/python/eva_py`: Pure Python3 wrapper on the Rust bindings.
- `/eva-py-macros`: Macros for the Python3 scripting API.
- `/scripts`: Python3 scripts.

_Note: In Rust macros must be put into a separate crate._

== Ray Tracer

The ray tracer lives in `/eva`. It's written in 100% safe Rust and WGSL (WebGPU Shading Language).

- `/eva/src`
  - `/shader`: Types that can be loaded into the WGSL shaders.
  - `/scene`: Scene definition.
  - `/renderer`: The machinery that creates the WebGPU primitives, loads GPU data, and runs the shaders.
- `/eva/shaders`
  - `display.wgsl`: Fragment and vertex shader.
  - `ray_tracer.wgsl`: Ray tracer compute shader.

Eva uses #link("https://github.com/rust-windowing/winit")[#underline[`winit`]] as the windowing API. It's cross-platform and the defacto API in the Rust ecosystem.
After creating a `winit::Window`, to use the ray tracer, you first create a `StaticRenderContext` which contains information about your scene that will not change. This includes the loaded textures, materials, and some parameters for the ray tracer. Using the `StaticRenderContext` and the `Window` you can build a `Renderer` using the `RendererBuilder`.

```rust
let window: winit::Window = todo!("create a window");
let static_context: StaticRenderContext = todo!("create a static context");
let mut renderer = RendererBuilder::new(window, static_context).build();
```

Eva uses #link("https://github.com/gfx-rs/wgpu")[#underline[`wgpu`]] to access WebGPU. The `RendererBuilder` will create the WebGPU buffers, bind group layouts, bind groups (where possible), pipelines, textures, shader modules, and create all of the required WebGPU core API components including the `Device`, `Queue`, `Surface`, and `Adapter`. `build()` then loads this state into a `Renderer`.

The `Renderer` has one function, `render`, which takes in a `DynamicRenderContext`. The `DynamicRenderContext` contains things that _will change_. This includes the positions of objects in your `Scene`, and the position and orientation of your `Camera`. 

```rust
let mut renderer: Renderer = RendererBuilder::new(window, static_context).build();
let mut dynamic: DynamicRenderContext = todo!();
loop {
  renderer.render(dynamic);
  update(dynamic);
}
```
On render, all of the data is loaded into the shaders and the render commands are encoded using a WebGPU `CommandEncoder`. There are two$\*$ render passes, a `ComputePass` and a `RenderPass`. The `ComputerPass`, which uses `ray_tracer.wgsl`, will run `ray_tracer.wgsl` once for each pixel in the screen using "working groups" (a collection of runs of a compute shader). Each run will compute the color of the pixel and store it in a texture. The `RenderPass` uses `display.wgsl` which is a simple shader that is used to compute the `UV`s for sampling from the texture written to by the `ComputePass`. The screen buffers are then swapped and the new frame is displayed. 

The `Renderer` _does not_ have any notion of a render loop. Updates can be handled in whatever way you want so long as you can provide the `Renderer` with a `DynamicRenderContext`. This made it significantly easier to enable runtime `update()`s from Python.

\*_There's a third render pass to take screenshots._

== Scripting Bindings

The scripting bindings live in `/eva-py`. Eva uses #link("https://github.com/PyO3/pyo3")[#underline[`pyo3`]] to generate bindings and #link("https://github.com/PyO3/maturin")[#underline[`maturin`]] to create the Python3 package `eva_py`. The raw `pyo3` bindings are not very easy to work with directly due to Rust's ownership rules (e.g. once I "give" an object to Rust it can no longer be updated from Python), and type restrictions (e.g. cannot build a robust class hierarchy). For those reasons, a pure Python layer - `/eva-py/python/eva_py` - was added to make the scripting API easier to work with.

The `eva_py` package can be built as follows:
```bash
cd eva-py
# create a virtual environment
python3 -m venv .env
source .env/bin/activate
# install maturin
pip install maturin
# start maturin
maturin develop
python3 "script-that-uses-eva.py"
```


== Scripting API

Not comprehensive overview of the scripting API.
```python
# import the module
from eva_py import *

# create a skybox (from /assets/skybox)
Eva.add_skybox(["x", "-x", "y", "-y", "z", "-z"])

# load a texture (from /assets/textures)
wood_handle = Eva.add_texture("wood.png")

rock_material = Material(
  0.9, # roughness
  0.3, # metalicness
  (1, 0, 0), # rgb colour
  # texture=wood_handle (optional) texture handle
  # light=(1, 1, 1) (optional) light emissiveness
)
rock_handle = Eva.add_material(rock_material)

# create a mesh (from /assets/meshes)
suzanne = Mesh("suzanne.obj").translate(1, 0, 0) \
                             .set_material(rock_handle)
```

For a more comprehensive look at how it can be used, check out the `/scripts/flappy-bird` for a dynamic example and 
`/scripts/nonhier` for a static example.

= Development Process
#line(length: 100%)

== Lighting

== Web

= Games and Images

= Post Mortem
#line(length: 100%)

The biggest trump card for this project is that I didn't know what I wanted to do until I started building it. I looked at my output images and 
considered what would make them better and I let that, more than my core objectives, drive my development process. Perhaps a little more research at the proposal
phase could have helped to avoid some of this, but I couldn't _see_ what my project would look like at that point so it was hard to look forward and know what I'd want to add.

The second biggest trump card was that virtually every feature was harder to add and debug because the routine that determines the color of each pixel runs in a compute shader on the GPU. All data structures need to be passed into the GPU, textures need to be encoded in a GPU-compatible format, and there is no such thing as a print statement. 

#pagebreak()

Three things were harder than I thought they would have been:

== Porting Eva to the Web

Yes, `wgpu` compiles to WASM but the work extends far beyond that. 

- i) Getting the Python scripting to work in the web was very tricky and something I ultimately sided against doing. To make real-time updates work, my renderer requires exclusive access to the Python3 Global Interpreter Lock (GIL) to run the Python code and fetch the updated values. On the web, this is hard because WASM is single-threaded. So, although I could have gotten the scripts to compile to WASM, I couldn't have run them in my browser without significantly modifying how I handle the updates and added a lot of `#[cfg(target = "wasm")]` annotations for conditional rendering.
- ii) Assets. Loading assets on the web requires making requests. This is fine, but it requires an asynchronous runtime, like NodeJS, to poll the `Future`s (promises in JavaScript) to see if the asset is ready. Threads cannot block. Eva and Eva-py fetch texture assets, mesh assets, and create screenshots. To make these asynchronous, I needed to either move all asset loading into Rust and add an async runtime for Rust that was WASM compatible, or add an async runtime for both Rust and Python. And for Python runtimes, I needed it to be compatible with #link("https://github.com/pyodide/pyodide")[`pyodide`] so it could run in the browser. 


For those reasons, I decided to not port the application to the web.

== GPU Compatibility

Not all GPUs have the same features. The WebGPU `Adapter` is used to ask for a `Device` and `Queue` supporting a certain set of features, if they're available. The WebGPU `Instance`, a wrapper on your native GPU, allows you to create a `Surface` (a fancy texture) given a `winit::Window` and can tell you what the capabilities of that `Surface` are. Because I work at home on my local machine, I found out rather late into this assignment that the features and capabilities of the `Device` and `Surface` of my local machine (an M1 Max Macbook Pro) are different than what are available on the school Linux machines. My project would not run. Fixing this required changing texture formats, storage types for texture, and some other shader-specific logic. This was a non-trivial diff I was not expecting. 

== Random Number Generation

The quality (i.e. randomness) of random numbers greatly impacted the quality of my images, because they are used extensively when computing how rays should reflect when hitting diffuse surfaces. Typically, you can resolve this by providing a uniform to your shader and then using that as a seed for random number generation. Compute shaders, however, are numbers hundreds of times with the same uniforms making this not a feasible solution. Each invocation, however, does provide a `GlobalInvocationID` which is a number from zero to the number of invocations. This single integer was the seed for my random number generation. It works, but it's not perfect and results in some visual artifacts.

= Resources
#line(length: 100%)

== Dependencies
- #link("https://github.com/gfx-rs/wgpu")[`wgpu`]: Rust implementation of the WebGPU specification. 
- #link("https://github.com/PyO3/pyo3")[`pyo3`]: Rust to Python3 bindings.
- #link("https://github.com/PyO3/maturin")[`maturin`]: Python3 package builder for pyo3 generated bindings.
- #link("https://github.com/dimforge/nalgebra")[`nalgebra`]: Rust linear algebra.
- #link("https://github.com/rust-windowing/winit")[`winit`]: Rust cross-platform windowing.
- #link("https://github.com/zesterer/pollster")[`pollster`]: Rust crate for statically resolving `Future`s.
- #link("https://github.com/Lokathor/bytemuck")[`bytemuck`]: Rust crate for converting data types into bytes.
- #link("https://github.com/image-rs/image")[`image`]: Rust crate for encoding and decoding images.
- #link("https://github.com/teoxoy/encase")[`encase`]: Rust crate for byte-aligning structures for use in WebGPU shaders.
- #link("https://github.com/simnalamburt/obj-rs")[`obj`]: Rust crate for loading `.obj` meshes.
- #link("https://github.com/starkat99/half-rs")[`half`]: Rust crate for handling 16-bit floating float numbers.

== Learning Resources

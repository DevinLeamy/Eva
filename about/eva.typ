#show link: underline

#align(center)[
  #heading(outlined: false)[Eva]
  #heading(outlined: false, level: 3)[_A WebGPU Real-time Ray Tracer Written in Rust_]
]
#line(length: 100%)

#outline(indent: auto)


#set heading(numbering: "1.")

= Overview
#line(length: 100%)

Eva is a real-time ray tracer built in Rust using WebGPU, with an integrated Python3 scripting API. Unlike WALL-E (A4) which took hours to render a 
nice image, Eva is hip and modern. Eva can render at 850x850 resolution with 16 samples per pixel at 60FPS on an M1 Max Macbook Pro. Eva can also go big, rendering images with
over 1000 samples and hundreds of reflections per pixel in only a couple of minutes!

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

\*_There's a third render pass for Multisample Anti Aliasing and a fourth render pass to take a screenshot. _

== Lighting

Objects are lit in a physically based way. Rays are shot into the scene, objects are hit, and the rays are reflected based on the material properties on the impacted surface. Diffuse surfaces will randomly scatter the rays, while metallic surfaces will perfectly reflect the rays. Materials can also be partially diffuse and partially metallic, in practice meaning the reflected ray is a weighted average of a perfect reflection and a random reflection. There is no ambient lighting.

Initially, Eva used Blinn-Phong lighting. Then PBR as described in #link("https://learnopengl.com/PBR/Theory")[Learn OpenGL PBR]. The lighting was then changed to how it is now. The motivation for this was that I didn't want the real-time renders to look like something you could easily achieve with a raster engine (i.e. clever approximations of real lighting). I wanted
it to be apparent that the images were ray traced. 

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

Non-comprehensive overview of the scripting API.
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

= Games and Images

Screenshots can be found in `/assets`. Eva comes with two games, "Flappy Bird" and "Pong". 

#figure(
  stack(dir: ltr)[#image("./assets/flap.png", width: 50%)][#image("./assets/pong.png", width: 50%)],
  caption: "Real-time Flappy Bird and Pong"
)

#strong[Flappy Bird]
- `[Space]` Jump

```bash
./debug.sh flappy-bird/flappy-bird
```

#strong[Pong]
- `[A]` Move bottom paddle left
- `[D]` Move bottom paddle right
- `[J]` Move top paddle left
- `[L]` Move top paddle right

```bash
./debug.sh pong
```

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

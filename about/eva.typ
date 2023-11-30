#heading(outlined: false)[CS 488 Final Project]

Term: Fall 2023

Name: Devin Leamy

UW ID: 20872933

UW UserID: dleamy

#align(center)[
  #heading(outlined: false)[Eva]
  #heading(outlined: false, level: 3)[_A WebGPU Real-time Ray Tracer_]
]
#line(length: 100%)

#outline()


#set heading(numbering: "1.")
= Overview
#line(length: 100%)

Eva is a real-time ray tracer built in Rust using WebGPU, with an integrated scripting API. 

#pagebreak()
= Features
#line(length: 100%)

#heading(outlined: false, level: 2)[Texture Mapping]

Any material can be assigned a texture. Textures are sourced from: `./eva/assets/textures`.

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


#heading(outlined: false, level: 2)[Skyboxes]

Scenes can optionally set a skybox. Skyboxes are sourced from: `./eva/assets/skybox`.
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

#heading(outlined: false, level: 2)[Phong Shading]

Eva can render `.obj` meshes with triangular faces. If the mesh has vertex normals, Phong Shading is applied.

#figure(
  image("./assets/phong.png", width: 50%),
  caption: "Suzanne Phong Shading"
)

#heading(outlined: false, level: 2)[Real-time Ray Tracing]
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

#heading(outlined: false, level: 2)[Reflections]

#heading(outlined: false, level: 2)[Python Scripting]
Eva is divided into two core components: `/eva` and `/eva-py`. `/eva-py` defines a scripting 
API for the `/eva` renderer. Scripts are sources from `/scripts`. 

Scripts can be run using the utilities `run.sh` and `debug.sh`. `debug.sh` will display build logs. 

To run a script, `my-scene.py` execute:

```bash
./debug.sh my-scene
```


#heading(outlined: false, level: 2)[TODO: Photon mapping]

#heading(outlined: false, level: 2)[TODO: PBR Materials]

= Technical Overview
#line(length: 100%)

#heading(outlined: false, level: 2)[Ray Tracer]

#heading(outlined: false, level: 2)[Scripting Bindings]

#heading(outlined: false, level: 2)[Scripting API]

= Development Process
#line(length: 100%)

#heading(outlined: false, level: 2)[Lighting]

#heading(outlined: false, level: 2)[Web]

= Post Mortem
#line(length: 100%)

#heading(outlined: false, level: 2)[Porting t]

= Resources
#line(length: 100%)

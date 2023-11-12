# Python3 Scripting Docs

Scripting is done in Python using functions and classes that are very similar, if not the same,
as the original Lua scripting API.

## Notes

-   To render bounding volumes, call `Scene::render_bounding_volumes()` before calling `ray_trace`.
-   Lights are part of the scene hierarchy, rather than being absolutely positioned.

## Classes and Methods

`Light`, `Geometry`, `Mesh`, and `Transform` are types of scene nodes. `Transform` is the type
of the root node in all scenes scripts, but they don't have to be.
Scene nodes can be rotated, scaled, translated, and have any number of children.

-   `Node::rotate(axis, degree)`: Rotate the node around a given axis, "x", "y", or "z".
-   `Node::translate(x, y, z)`: Translate the node.
-   `Node::scale(x, y, z)`: Scale the node.
-   `Node::add_child(node)`: Add a child node to node.
-   `Transform()`: Create a new transformation node.
-   `Material((d0, d1, d2), (s0, s1, s2), shininess)`: Create a new Phong material.
    -   `(d0, d1, d2)`: Diffuse parameters.
    -   `(s0, s1, s2)`: Specular parameters.
-   `Geometry(primitive)`: Construct a geometric primitive. I.e. "sphere" or "cube"
-   `Geometry::set_material(material)`: Set the material of the primitive.
-   `Mesh(obj_file)`: Construct a mesh from an object file.
    -   e.g. `Mesh("buckyball.obj")` loads `/wall-e-py/assets/meshes/buckyball.obj`.
-   `Mesh::set_material(material)`: Set the material of the mesh.
-   `Light((i0, i1, i2), (c0, c1, c2))`: Create a new light.
    -   `(i0, i1, i2)`: RGB intensity parameters.
    -   `(c0, c1, c2)`: Attenuation parameters.
-   `Scene()`: Create a new scene.
-   `Scene::render_bounding_volumes()`: Render bounding volumnes around meshes.
-   `Scene::set_root(node)`: Set the root node of the scene.
-   `Scene::set_ambient(a0, a1, a2)`: Set the ambient lighting paramets for the scene.
-   `Camera((x, y, z), (f0, f1, f2), (u0, u1, u2), fov)`: Create a camera.
    -   `(x, y, z)`: Position of the camera.
    -   `(f0, f1, f2)`: Forward vector for the camera.
    -   `(u0, u1, u2)`: Up vector for the camera.
-   `Camera::look_at(x, y, z)`: Orient the camera to look at a given point.
-   `ray_trace(scene, camera, img_width, img_height, file):`: Render the scene using the ray tracer.

## Script template

```python3
# Imports
from wall_e_py import Mesh, Scene, Transform, Geometry, Light, ray_trace, Camera, Material

# Materials
# ...

# Create a scene root.
scene_root = Transform()

# Add geometry to the scene.
# ...

# Add a light source
light = Light((0.8, 0.8, 0.8), (1, 0, 0))
light.translate(200, 202, 430)
scene_root.add_child(light)

# Create a camera.
camera = Camera((0, 0, 0), (0, 0, -1), (0, 1, 0))

# Create a scene.
scene = Scene()
scene.set_root(scene_root)
scene.set_ambient(0.2, 0.2, 0.2)
# scene.render_bounding_volumes()

# Render the scene.
ray_trace(scene, camera, 256, 256, "image.png")
```

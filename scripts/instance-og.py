from eva_py import Mesh, Scene, Transform, Geometry, Light, ray_trace, Camera, Material
from shared import copy_and_archive_image

stone = Material((0.8, 0.7, 0.7), (0.0, 0.0, 0.0), 0)
grass = Material((0.1, 0.7, 0.1), (0.0, 0.0, 0.0), 0)
scene = Scene()

n_scene = Transform()
n_scene.rotate('X', 23)

plane = Mesh('plane.obj')
plane.set_material(grass)
plane.scale(30, 30, 30)
n_scene.add_child(plane)

sphere = Mesh('buckyball.obj')
sphere.scale(1.5, 1.5, 1.5)
sphere.set_material(stone)
n_scene.add_child(sphere)

for i in range(6):
    arc = Transform()
    arc.translate(0, 0, -10)

    p1 = Geometry("cube")
    p1.set_material(stone)
    p1.scale(0.8, 4, 0.8)
    p1.translate(-2.4, 0, -0.4)
    arc.add_child(p1)

    p2 = Geometry("cube")
    p2.set_material(stone)
    p2.scale(0.8, 4, 0.8)
    p2.translate(1.6, 0, -0.4)
    arc.add_child(p2)

    s = Geometry("sphere")
    s.set_material(stone)
    s.scale(4, 0.6, 0.6)
    s.translate(0, 4, 0)
    arc.add_child(s)

    n_arc = Transform()
    n_arc.rotate('Y', i * 60)
    n_arc.add_child(arc)
    n_scene.add_child(n_arc)

light = Light((0.8, 0.8, 0.8), (1, 0, 0))
light.translate(200, 202, 430)

n_scene.add_child(light)
camera = Camera((0, 2, 30), (0, 0, -1), (0, 1, 0), 50)
scene.set_root(n_scene)
scene.set_ambient(0.4, 0.4, 0.4)

ray_trace(scene, camera, 256, 256, "image.png")

copy_and_archive_image()
# ray_trace(scene, camera, 600, 600, "image.png")
# ray_trace(scene, camera, 1000, 1000, "image.png")
# ray_trace(scene, camera, 100, 100, "image.png")

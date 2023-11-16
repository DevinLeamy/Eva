from eva_py import Mesh, Scene, Transform, Geometry, Light, ray_trace, Camera, Material
from shared import copy_and_archive_image

stone = Material((0.8, 0.7, 0.7), (0.0, 0.0, 0.0), 0)
grass = Material((0.1, 0.7, 0.1), (0.0, 0.0, 0.0), 0)
hide = Material((0.84, 0.6, 0.53), (0.3, 0.3, 0.3), 20)

scene_root = Transform()
scene_root.rotate('X', 23)

plane = Mesh('plane.obj')
plane.set_material(grass)
plane.scale(30, 30, 30)
scene_root.add_child(plane)

buckyball = Mesh('buckyball.obj')
buckyball.set_material(stone)
buckyball.scale(1.5, 1.5, 1.5)
scene_root.add_child(buckyball)

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

for i in range(6):
    arch_instance = Transform()
    arch_instance.rotate('Y', i * 60)
    arch_instance.add_child(arc)
    scene_root.add_child(arch_instance)


def create_cow():
    cow = Transform()
    parts = [
        ("body", (0, 0, 0), 1.0),
        ("head", (0.9, 0.3, 0), 0.6),
        ("tail", (-0.94, 0.34, 0), 0.2),
        # legs
        ("lfleg", (0.7, -0.7, -0.7), 0.3),
        ("lrleg", (-0.7, -0.7, -0.7), 0.3),
        ("rfleg", (0.7, -0.7, 0.7), 0.3),
        ("rrleg", (-0.7, -0.7, 0.7), 0.3),
    ]

    for name, position, scale in parts:
        part = Geometry("sphere")
        part.set_material(hide)
        part.scale(scale, scale, scale)
        part.translate(position[0], position[1], position[2])
        cow.add_child(part)

    return cow


cow_positions = [((1, 1.3, 14), 20), ((5, 1.3, -11), 180),
                 ((-5.5, 1.3, -3), -60)]
for position, rotation in cow_positions:
    cow_instance = Transform()
    cow_instance.add_child(create_cow())
    cow_instance.scale(1.4, 1.4, 1.4)
    cow_instance.rotate('Y', rotation)
    cow_instance.translate(position[0], position[1], position[2])
    scene_root.add_child(cow_instance)

light_source = Light((0.8, 0.8, 0.8), (1, 0, 0))
light_source.translate(200, 202, 430)
scene_root.add_child(light_source)

camera = Camera((0, 2, 30), (0, 0, -1), (0, 1, 0), 50)

scene = Scene()
scene.set_root(scene_root)
scene.set_ambient(0.4, 0.4, 0.4)

ray_trace(scene, camera, 256, 256, "image.png")
# ray_trace(scene, camera, 900, 900, "image.png")

copy_and_archive_image()

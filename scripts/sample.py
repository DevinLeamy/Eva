from wall_e_py import Mesh, Scene, Transform, Light, Camera, Material, Geometry, ray_trace
from shared import copy_and_archive_image
import random

wood = Material((0.8, 0.7, 0.7), (0.0, 0.0, 0.0), 0)
leaves = Material((0.8, 0.3, 0.3), (0.0, 0.0, 0.0), 0)
leaves2 = Material((0.8, 0.5, 0.5), (0.0, 0.0, 0.0), 0)
grass = Material((0.1, 0.4, 0.1), (0.0, 0.0, 0.0), 0)
tree_leaves = Material((0.4, 0.7, 0.4), (0.0, 0.0, 0.0), 0)
car_side = Material((0.84, 0.0, 0.00), (0.3, 0.3, 0.3), 20)
car_wheel = Material((0.0, 0.0, 0.0), (0.3, 0.3, 0.3), 0)
car_light = Material((8.0, 8.0, 0.6), (0.3, 0.3, 0.3), 0)
stone = Material((0.8, 0.7, 0.7), (0.0, 0.0, 0.0), 0)
water = Material((0.1, 0.1, 0.7), (0.9, 0.9, 0.9), 20)

scene_root = Transform()

plane = Mesh('plane.obj')
plane.set_material(grass)
plane.scale(90, 90, 90)
scene_root.add_child(plane)


def create_car():
    tree = Transform()
    h = 2
    s = 1.5
    l = 0.8
    parts = [
        # Body
        ("cube", (0, 2, 0), (11.0, 2, 5.0), car_side),
        ("cube", (0, 4, 1.5), (7.0, 4, 2.5), car_side),
        ("cube", (10, 4, 1.5), (0.5, 3, 2.5), car_wheel),
        # Wheels
        ("sphere", (13.0, 0, 0.0), (s, s, s), car_wheel),
        ("sphere", (13.0, 0, 7.0), (s, s, s), car_wheel),
        ("sphere", (2, 0, 0.0), (s, s, s), car_wheel),
        ("sphere", (2, 0, 7.0), (s, s, s), car_wheel),
        # lights
        ("sphere", (15, 4, 2.0), (l, l, l), car_light),
        ("sphere", (15, 4, 5.0), (l, l, l), car_light),
    ]
    for geometry, position, scale, material in parts:
        part = Geometry(geometry)
        part.set_material(material)
        part.scale(scale[0], scale[1], scale[2])
        part.translate(position[0], position[1], position[2])
        tree.add_child(part)

    return tree


def create_tree():
    tree = Transform()
    h = 6.0
    parts = [
        # Trunk
        ("cube", (0, 0, 0), (1.5, h, 1.5), wood),
        # Leaves
        ("sphere", (1.0, h + 0.3, 1), (2.0, 2.0, 2.0), tree_leaves),
        ("sphere", (1.0, h + 1.7, 1), (1.8, 1.8, 1.8), tree_leaves),
        ("sphere", (1.4, h + 2.4, 1), (1.6, 1.6, 1.6), tree_leaves),
    ]
    for geometry, position, scale, material in parts:
        part = Geometry(geometry)
        part.set_material(material)
        part.scale(scale[0], scale[1], scale[2])
        part.translate(position[0], position[1], position[2])
        tree.add_child(part)

    return tree


def create_leaves():
    n_leaves = Transform()
    h = -0.7
    parts = [
        # Leaves
        ("sphere", (-0.2, h, -0.2), (1.0, 1.0, 1.0), leaves),
        ("sphere", (0, h, 0), (1.8, 1.8, 1.8), leaves),
        ("sphere", (-0.4, h, 0.5), (1.6, 1.6, 1.6), leaves2),
    ]
    for geometry, position, scale, material in parts:
        part = Geometry(geometry)
        part.set_material(material)
        part.scale(scale[0], scale[1], scale[2])
        part.translate(position[0], position[1], position[2])
        n_leaves.add_child(part)

    return n_leaves


x = [-40, 40]
z = [20, -40]

random.seed(41)

tree_positions = []
n_trees = 10
for i in range(n_trees):
    x_pos = random.uniform(x[0], x[1])
    z_pos = random.uniform(z[0], z[1])
    tree_positions.append(((x_pos, 1.3, z_pos), 0))

random.seed(47)
leaves_positions = []
n_leaves = 6
for i in range(n_leaves):
    x_pos = random.uniform(x[0], x[1])
    z_pos = random.uniform(z[0], z[1])
    leaves_positions.append(((x_pos, 1.3, z_pos), random.uniform(0, 180)))

for position, rotation in tree_positions:
    tree_instance = Transform()
    tree_instance.add_child(create_tree())
    tree_instance.scale(1.4, 1.4, 1.4)
    tree_instance.rotate('Y', rotation)
    tree_instance.translate(position[0], position[1], position[2])
    scene_root.add_child(tree_instance)

for position, rotation in leaves_positions:
    leaves_instance = Transform()
    leaves_instance.add_child(create_leaves())
    leaves_instance.scale(1.4, 1.4, 1.4)
    leaves_instance.rotate('Y', rotation)
    leaves_instance.translate(position[0], position[1], position[2])
    scene_root.add_child(leaves_instance)

car_positions = [
    ((0, 0, 8), -60),
    ((-20, 0, -20), 30),
]

for position, rotation in car_positions:
    car_instance = Transform()
    car_instance.add_child(create_car())
    car_instance.scale(1.4, 1.4, 1.4)
    car_instance.rotate('Y', rotation)
    car_instance.translate(position[0], position[1], position[2])
    scene_root.add_child(car_instance)

rock_positions = [
    ((-40, -10, -60), 30, 12),
    ((35, -10, -60), 60, 14),
    ((10, -15, -80), 40, 18),
]

for position, rotation, scale in rock_positions:
    rock_instance = Mesh('buckyball.obj')
    rock_instance.set_material(stone)
    rock_instance.scale(scale, scale, scale)
    rock_instance.rotate('Y', rotation)
    rock_instance.translate(position[0], position[1], position[2])
    scene_root.add_child(rock_instance)

water_pool = Geometry('sphere')
water_pool.translate(-18, 0, 27)
water_pool.scale(15, 1, 15)
water_pool.set_material(water)

scene_root.add_child(water_pool)


light_source = Light((0.8, 0.8, 0.8), (1, 0, 0))
light_source.translate(50, 202, -130)

light_source2 = Light((0.4, 0.4, 0.4), (1, 0, 0))
light_source2.translate(-50, 202, 130)

light_source3 = Light((0.8, 0.8, 0.8), (1, 0.5, 0))
light_source3.translate(-18, 3, 20)

scene_root.add_child(light_source)
scene_root.add_child(light_source2)
scene_root.add_child(light_source3)

scene = Scene()
scene.set_root(scene_root)
scene.set_ambient(0.2, 0.2, 0.2)

camera = Camera((0, 30, 30), (0, 0, -1), (0, 1, 0), 80)
camera.look_at(0, 0, 0)

ray_trace(scene, camera, 500, 500, "image.png")

copy_and_archive_image()

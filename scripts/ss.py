from eva_py import Mesh, Scene, Transform, Geometry, Light, ray_trace, Camera, Material
import random

AMBIENT = 0.5


ground = Material((0, 0.3, 0.3), (0.3, 0.3, 0.3), 10)
red = Material((0.7, 0.0, 0.0), (0.3, 0.3, 0.3), 10)
green = Material((0.0, 0.7, 0.0), (0.3, 0.3, 0.3), 10)
blue = Material((0.0, 0.0, 0.7), (0.3, 0.3, 0.3), 10)

scene = Scene()
wood = scene.add_texture("wood.jpeg")
scene.set_skybox([
    "blue/x.png",
    "blue/-x.png",
    "blue/y.png",
    "blue/-y.png",
    "blue/z.png",
    "blue/-z.png",
])
# scene.set_skybox([
#     "sky/z.tga",
#     "sky/-z.tga",
#     "sky/y.tga",
#     "sky/-y.tga",
#     "sky/x.tga",
#     "sky/-x.tga",
# ])
scene.set_ambient(AMBIENT, AMBIENT, AMBIENT)
scene_root = Transform()

scene_size = 10000
plane_size = scene_size
plane = Geometry("cube")
center = [0, 0]

plane.translate(center[0], -100, center[1])
plane.scale(plane_size, 5, plane_size)
plane.set_material(ground)
plane.set_texture(wood)
scene_root.add_child(plane)

cube_size = 500


def random_color():
    return (
        random.random(),
        random.random(),
        random.random(),
    )


def random_shape():
    return random.sample(["cube", "sphere"], 1)[0]


def random_material():
    m = Material(random_color(), (0.1, 0.1, 0.1), 25)
    # m = Material(random_color(), (0.0, 0.0, 0.0), 10)
    return m


def random_offset():
    return (
        random.randint(-50, 50),
        random.randint(-250, 250),
        random.randint(-50, 50),
    )


for i in range(4):
    for j in range(4):
        x = i / 4 * scene_size
        z = j / 4 * scene_size

        cube = Geometry(random_shape())
        offset = random_offset()
        cube.translate(center[0] + x + offset[0], cube_size /
                       2 + offset[1] + 500, center[0] + z + offset[2])
        cube.scale(cube_size, cube_size, cube_size)
        cube.set_material(random_material())
        scene_root.add_child(cube)

camera = Camera(
    (0, 0, 2700),
    (0, 0, -1),
    (0, 1, 0),
    50,
)

white_light = Light((0.9, 0.9, 0.9), (1.0, 0, 0))
white_light.translate(0, 500.0, 0)
scene_root.add_child(white_light)

scene.set_root(scene_root)
ray_trace(scene, camera)

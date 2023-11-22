from eva_py import TextureBuilder, Scene, Light, Camera, Material, Eva, Box

AMBIENT = 0.3
wood = TextureBuilder.build("wood.jpeg")

scene = Scene()
scene.set_ambient(AMBIENT)
scene.set_skybox([
    "blue/x.png",
    "blue/-x.png",
    "blue/y.png",
    "blue/-y.png",
    "blue/z.png",
    "blue/-z.png",
])

ball_mat = Material(
    (0.5, 0.0, 0.0),
    (0.5, 0.5, 0.5),
    10
)
paddle_mat = Material(
    (0.5, 0.5, 0.5),
    (0.0, 0.0, 0.0),
    0
)
table_mat = Material(
    (1.0, 1.0, 1.0),
    (0.4, 0.4, 0.4),
    10
)

ball_size = 10
paddle_width = 20
paddle_height = 5

table = Box()
table.scale(100, 100, 2)
table.set_material(table_mat)
table.set_texture(wood)
scene.add(table)

white_light = Light(0.9)
white_light.translate(-100.0, 150.0, 400.0)

scene.add(white_light)

camera = Camera((0, 0, 200))

Eva.run()

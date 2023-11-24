from eva_py import TextureBuilder, Scene, Light, Camera, Material, Eva, Box, Sphere

AMBIENT = 0.3
wood = TextureBuilder.build("wood.jpeg")

scene = Scene()
scene.set_ambient(AMBIENT)
ball_mat = Material(
    (0.4, 0.0, 0.8),
    (0.0, 0.0, 0.0),
    10
)

table_mat = Material(
    (1.0, 1.0, 1.0),
    (0.4, 0.4, 0.4),
    10
)

ball_size = 8
board_size = 100
game_z = 20

table = Box()
table.scale(board_size, board_size, 2)
table.set_material(table_mat)
# table.set_texture(wood)
scene.add(table)

ball = Sphere(radius=ball_size)
ball.set_material(ball_mat)
ball.translate(0, 0, game_z)

scene.add(ball)

white_light = Light(0.9)
white_light.translate(-100.0, 150.0, 400.0)

scene.add(white_light)

camera = Camera((75, 0, 200))
camera.look_at(0, 0, 0)


def update():
    print("Start update")
    camera.translate(0.0000, 0.0000, 0.001)
    print("End update")


Eva.run(update)

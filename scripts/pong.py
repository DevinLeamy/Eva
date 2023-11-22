from eva_py import TextureBuilder, Scene, Light, Camera, Material, Eva, Box, Sphere

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
    (0.4, 0.0, 0.8),
    (0.0, 0.0, 0.0),
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

ball_size = 8
paddle_width = 40
paddle_height = 10
paddle_depth = 10
board_size = 100
game_z = 20

table = Box()
table.scale(board_size, board_size, 2)
table.set_material(table_mat)
# table.set_texture(wood)
scene.add(table)

top_paddle = Box()
top_paddle.scale(paddle_width, paddle_height, paddle_depth)
top_paddle.set_material(paddle_mat)
top_paddle.translate(0.0, -board_size / 2.0, game_z)

scene.add(top_paddle)

bottom_paddle = Box()
bottom_paddle.scale(paddle_width, paddle_height, paddle_depth)
bottom_paddle.set_material(paddle_mat)
bottom_paddle.translate(0.0, board_size / 2.0, game_z)

scene.add(bottom_paddle)

ball = Sphere(radius=ball_size)
ball.set_material(ball_mat)
ball.translate(0, 0, game_z)

scene.add(ball)

white_light = Light(0.9)
white_light.translate(-100.0, 150.0, 400.0)

scene.add(white_light)

camera = Camera((75, 0, 200))
camera.look_at(0, 0, 0)


def foo():
    print("bar")


Eva.run()

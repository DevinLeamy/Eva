from eva_py import Scene, Camera, Material, Eva, Box, Sphere
from eva_py import vec3_sub, vec3_normalize, vec3_scalar_mult, vec3_length

# Eva.add_skybox([
#     "blue/x.png",
#     "blue/-x.png",
#     "blue/y.png",
#     "blue/-y.png",
#     "blue/z.png",
#     "blue/-z.png",
# ])
Eva.add_skybox([
    "sky/z.tga",
    "sky/-z.tga",
    "sky/y.tga",
    "sky/-y.tga",
    "sky/x.tga",
    "sky/-x.tga",
])

# Bottom paddle.
MOVE_LEFT: str = "A"
MOVE_RIGHT: str = "D"
# Top paddle.
TOP_MOVE_LEFT: str = "J"
TOP_MOVE_RIGHT: str = "L"

PADDLE_SPEED: float = 3.0
BALL_SPEED: float = 3.0

# Eva.set_ambient(0.10)
Eva.set_ambient(0.03)
ball_mat = Eva.add_material(Material(
    1.0,
    0.0,
    (0.0, 0.0, 1.0),
    # (1.0, 1.0, 1.0)
))
paddle_mat = Eva.add_material(Material(
    1.0,
    1.0,
    (1.0, 0.0, 0.0),
))
table_mat = Eva.add_material(Material(
    0.0,
    1.0,
    (1.0, 1.0, 1.0),
    (0.1, 0.1, 0.1)
))

wall_mat = Eva.add_material(Material(
    1.0,
    0.0,
    (0.0, 1.0, 0.0),
))
container_mat = Eva.add_material(Material(
    1.0,
    0.0,
    (1.0, 1.0, 1.0),
    # (0.2, 0.2, 0.2)
))
light_mat = Eva.add_material(Material(
    0.0,
    0.0,
    (0.0, 0.0, 0.0),
    (0.8, 0.8, 0.8)
))

scene = Scene()

ball_size = 4
paddle_width = 40
paddle_height = 3
paddle_depth = 15
board_size = 100
game_z = 20

wall_height = board_size - 10
wall_width = 2
wall_depth = paddle_depth

table = Box()
table.scale(board_size, board_size, 2)
table.set_material(table_mat)
scene.add(table)

top_paddle = Box()
top_paddle.scale(paddle_width, paddle_height, paddle_depth)
top_paddle.set_material(paddle_mat)
top_paddle.translate(0.0, board_size / 2.0, game_z)

scene.add(top_paddle)

bottom_paddle = Box()
bottom_paddle.scale(paddle_width, paddle_height, paddle_depth)
bottom_paddle.set_material(paddle_mat)
bottom_paddle.translate(0.0, -board_size / 2.0, game_z)

scene.add(bottom_paddle)

left_wall = Box()
left_wall.scale(wall_width, wall_height, wall_depth)
left_wall.translate(-board_size / 2.0, 0.0, game_z)
left_wall.set_material(wall_mat)

scene.add(left_wall)

right_wall = Box()
right_wall.scale(wall_width, wall_height, wall_depth)
right_wall.translate(board_size / 2.0, 0.0, game_z)
right_wall.set_material(wall_mat)

scene.add(right_wall)

ball = Sphere(radius=ball_size)
ball.set_material(ball_mat)
ball.translate(0, 0, game_z)
ball_velocity = [0, -BALL_SPEED, 0]

scene.add(ball)

top_light = Box()
top_light.scale(wall_height, paddle_height, paddle_depth)
top_light.set_material(light_mat)
top_light.translate(0.0, board_size / 2.0, 90)

# scene.add(top_light)

bottom_light = Box()
bottom_light.scale(wall_height, paddle_height, paddle_depth)
bottom_light.set_material(light_mat)
bottom_light.translate(0.0, -board_size / 2.0, 90)

# scene.add(bottom_light)

orbit = Sphere(10)
orbit.set_material(table_mat)
scene.add(orbit)

camera = Camera((0, 0, 220))
camera.look_at(0, 0, 0)


def handle_input(key, state):
    paddle_delta_x = 0
    top_paddle_delta_x = 0

    if key == MOVE_LEFT:
        paddle_delta_x = -PADDLE_SPEED
    elif key == MOVE_RIGHT:
        paddle_delta_x = PADDLE_SPEED

    if key == TOP_MOVE_LEFT:
        top_paddle_delta_x = -PADDLE_SPEED
    elif key == TOP_MOVE_RIGHT:
        top_paddle_delta_x = PADDLE_SPEED

    bottom_paddle.translate(paddle_delta_x, 0.0, 0.0)
    top_paddle.translate(top_paddle_delta_x, 0.0, 0.0)


ball_escaping = False

r = 0


def position_in_orbit(t, r):
    import math
    theta = (2 * math.pi / 20) * t
    x = r * math.cos(theta)
    y = r * math.sin(theta)
    return (x, y)


def update():
    global ball_velocity, ball_escaping, r

    # Update the position of the ball.
    ball.translate(ball_velocity[0], ball_velocity[1], 0.0)

    # Check for ball-paddle intersections.
    if ball.intersects_with(bottom_paddle):
        if not ball_escaping:
            x_offset = vec3_sub(ball.translation(),
                                bottom_paddle.translation())[0]
            ball_velocity[0] = (-x_offset) / 30.0
            ball_velocity[1] = ball_velocity[1] * -1
            ball_velocity = vec3_scalar_mult(
                vec3_normalize(ball_velocity), BALL_SPEED)
        ball_escaping = True
    elif ball.intersects_with(top_paddle):
        if not ball_escaping:
            x_offset = vec3_sub(ball.translation(),
                                top_paddle.translation())[0]
            ball_velocity[0] = (-x_offset) / 30.0
            ball_velocity[1] = ball_velocity[1] * -1
            ball_velocity = vec3_scalar_mult(
                vec3_normalize(ball_velocity), BALL_SPEED)
        ball_escaping = True
    elif ball.intersects_with(left_wall) or ball.intersects_with(right_wall):
        if not ball_escaping:
            ball_velocity[0] = ball_velocity[0] * -1
        ball_escaping = True
    else:
        ball_escaping = False

    if vec3_length(ball.translation()) > board_size:
        ball.set_translation(0, 0, game_z)
        ball_velocity = [0, -BALL_SPEED, 0]

    position = position_in_orbit(r, board_size * 0.65)
    orbit.set_translation(position[0], position[1], game_z + 30)
    r += 0.03


Eva.run(update, handle_input)

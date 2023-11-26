from eva_py import Camera, Material, Eva, Box, Sphere, Render
from eva_py import vec3_sub, vec3_normalize, vec3_scalar_mult, vec3_length

# Eva.add_skybox([
#     "blue/x.png",
#     "blue/-x.png",
#     "blue/y.png",
#     "blue/-y.png",
#     "blue/z.png",
#     "blue/-z.png",
# ])
Eva.set_max_reflections(15)
Eva.set_sample_count(9)
Eva.set_ambient(0.03)
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

ball_size = 4
paddle_width = 40
paddle_height = 3
paddle_depth = 15
board_size = 100
game_z = 20

wall_height = board_size - 10
wall_width = 2
wall_depth = paddle_depth


def position_in_orbit(t, r):
    import math
    theta = (2 * math.pi / 20) * t
    x = r * math.cos(theta)
    y = r * math.sin(theta)
    return (x, y)


class Pong(Render):
    def __init__(self):
        super().__init__()

        self.ball_velocity = [0, -BALL_SPEED, 0]

        self.r = 0

        table = Box()
        table.scale(board_size, board_size, 2)
        table.set_material(table_mat)
        self.add_geometry(table)

        self.top_paddle = Box() \
            .scale(paddle_width, paddle_height, paddle_depth) \
            .set_material(paddle_mat) \
            .translate(0.0, board_size / 2.0, game_z)
        self.add_geometry(self.top_paddle)

        self.bottom_paddle = Box() \
            .scale(paddle_width, paddle_height, paddle_depth) \
            .set_material(paddle_mat) \
            .translate(0.0, -board_size / 2.0, game_z)
        self.add_geometry(self.bottom_paddle)

        self.left_wall = Box() \
            .scale(wall_width, wall_height, wall_depth) \
            .translate(-board_size / 2.0, 0.0, game_z) \
            .set_material(wall_mat)
        self.add_geometry(self.left_wall)

        self.right_wall = Box() \
            .scale(wall_width, wall_height, wall_depth) \
            .translate(board_size / 2.0, 0.0, game_z) \
            .set_material(wall_mat)
        self.add_geometry(self.right_wall)

        self.ball_escaping = False
        self.ball_velocity = [0, -BALL_SPEED, 0]
        self.ball = Sphere(radius=ball_size) \
            .set_material(ball_mat) \
            .translate(0, 0, game_z)
        self.add_geometry(self.ball)

        self.orbit = Sphere(10) \
            .set_material(table_mat)
        self.add_geometry(self.orbit)

    def handle_input(self, key, state):
        if state != "Pressed":
            return
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

        self.bottom_paddle.translate(paddle_delta_x, 0.0, 0.0)
        self.top_paddle.translate(top_paddle_delta_x, 0.0, 0.0)

    def update(self):
        # Update the position of the ball.
        self.ball.translate(self.ball_velocity[0], self.ball_velocity[1], 0.0)

        # Check for ball-paddle intersections.
        if self.ball.intersects_with(self.bottom_paddle):
            if not self.ball_escaping:
                x_offset = vec3_sub(self.ball.translation(),
                                    self.bottom_paddle.translation())[0]
                self.ball_velocity[0] = (-x_offset) / 30.0
                self.ball_velocity[1] = self.ball_velocity[1] * -1
                self.ball_velocity = vec3_scalar_mult(
                    vec3_normalize(self.ball_velocity), BALL_SPEED)
            self.ball_escaping = True
        elif self.ball.intersects_with(self.top_paddle):
            if not self.ball_escaping:
                x_offset = vec3_sub(self.ball.translation(),
                                    self.top_paddle.translation())[0]
                self.ball_velocity[0] = (-x_offset) / 30.0
                self.ball_velocity[1] = self.ball_velocity[1] * -1
                self.ball_velocity = vec3_scalar_mult(
                    vec3_normalize(self.ball_velocity), BALL_SPEED)
            self.ball_escaping = True
        elif self.ball.intersects_with(self.left_wall) or self.ball.intersects_with(self.right_wall):
            if not self.ball_escaping:
                self.ball_velocity[0] = self.ball_velocity[0] * -1
            self.ball_escaping = True
        else:
            self.ball_escaping = False

        if vec3_length(self.ball.translation()) > board_size:
            self.ball.set_translation(0, 0, game_z)
            self.ball_velocity = [0, -BALL_SPEED, 0]

        position = position_in_orbit(self.r, board_size * 0.65)
        self.orbit.set_translation(position[0], position[1], game_z + 30)
        self.r += 0.03


pong = Pong()
pong.run()

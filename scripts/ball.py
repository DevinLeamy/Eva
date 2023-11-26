from eva_py import Scene, Camera, Material, Eva, Box, Sphere
from eva_py import vec3_sub, vec3_normalize, vec3_scalar_mult, vec3_length

Eva.set_ambient(0.03)
light_mat = Eva.add_material(Material(
    0.0,
    0.0,
    (0.0, 0.0, 0.0),
    (0.8, 0.8, 0.8)
))

# Eva.set_ambient(0.03)
scene = Scene()
for i in range(7):
    for j in range(7):
        ball_size = 4
        game_z = 20

        ball_mat = Eva.add_material(Material(
            1.0 / 7.0 * i,
            1.0 / 7.0 * j,
            (1.0, 0.0, 0.0),
        ))

        ball = Sphere(radius=ball_size)
        ball.set_material(ball_mat)
        ball.translate(-50 + i * 10, -50 + j * 10, game_z)

        scene.add(ball)

top_light = Box()
top_light.scale(100, 10, 60)
top_light.set_material(light_mat)
top_light.translate(0.0, 50 / 2.0, 90)

scene.add(top_light)

camera = Camera((0, 0, 200))
camera.look_at(0, 0, 0)


def handle_input(key, state):
    pass


def update():
    pass


Eva.run(update, handle_input)

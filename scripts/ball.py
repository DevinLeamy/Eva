from eva_py import *

Eva.add_skybox([
    "sky/z.tga",
    "sky/-z.tga",
    "sky/y.tga",
    "sky/-y.tga",
    "sky/x.tga",
    "sky/-x.tga",
])


class Ball(RenderStatic):
    def __init__(self):
        super().__init__()

        for i in range(7):
            for j in range(7):
                ball_size = 8 
                game_z = 20

                ball_mat = Eva.add_material(Material(
                    1.0 / 6.0 * i,
                    1.0 / 6.0 * j,
                    (1.0, 0.0, 0.0),
                ))

                ball = Sphere(radius=ball_size)
                ball.set_material(ball_mat)
                ball.translate(-50 + i * (ball_size + 10), -
                               50 + j * (ball_size + 10), game_z)

                self.add_geometry(ball)

    def handle_input(key, state):
        pass

    def update():
        pass


ball = Ball()
ball.run()

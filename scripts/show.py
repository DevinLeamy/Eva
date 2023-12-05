from eva_py import *

Eva.add_skybox([
    "sky/z.tga",
    "sky/-z.tga",
    "sky/y.tga",
    "sky/-y.tga",
    "sky/x.tga",
    "sky/-x.tga",
])
# Eva.add_skybox([
#     "blue/x.png",
#     "blue/-x.png",
#     "blue/y.png",
#     "blue/-y.png",
#     "blue/z.png",
#     "blue/-z.png",
# ])

class Ball(RenderStatic):
    def __init__(self):
        super().__init__()
        Eva.set_sample_count(200)
        Eva.set_screenshot("./archive/show.png")

        c = Material(0.2, 1.0, (1.0, 1.0, 1.0), [0.05, 0.05, 0.05])
        c_handle = Eva.add_material(c)

        self.add_geometry(Sphere(30).set_material(c_handle))

    def handle_input(key, state):
        pass

    def update():
        pass


ball = Ball()
ball.run()

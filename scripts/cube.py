from eva_py import *
import random

random.seed(420)

# Eva.add_skybox([
#     "sky/z.tga",
#     "sky/-z.tga",
#     "sky/y.tga",
#     "sky/-y.tga",
#     "sky/x.tga",
#     "sky/-x.tga",
# ])
Eva.add_skybox([
    "blue/x.png",
    "blue/-x.png",
    "blue/y.png",
    "blue/-y.png",
    "blue/z.png",
    "blue/-z.png",
])

BOX_SIZE = 10

wood_handle = Eva.add_texture("wood.jpeg")
wall_handle = Eva.add_texture("wall.jpg")


# class Cube(RenderDynamic):
class Cube(RenderStatic):
    def __init__(self):
        super().__init__()

        # Eva.set_sample_count(4)
        Eva.set_screenshot(f"./archive/cube-{random.randint(100, 999)}.png")
        Eva.set_sample_count(500)
        Eva.set_max_reflections(4)

        # self.camera.set_translation(-3 * BOX_SIZE, 40, 35)
        self.camera.set_translation(0, 40, 35)
        self.camera.look_at(-3 * BOX_SIZE, 20, -3.5 * BOX_SIZE)

        l1 = Eva.add_material(Material(
            1.0,
            0.0,
            (1.0, 1.0, 1.0),
            [0.5, 0.5, 0.5],
            texture=wall_handle
        ))
        l2 = Eva.add_material(Material(
            1.0,
            0.0,
            (1.0, 1.0, 1.0),
            # (1.0, 1.0, 1.0),
            [0.5, 0.5, 0.5],
            texture=wall_handle
        ))
        l3 = Eva.add_material(Material(
            0.0,
            1.0,
            (1.0, 1.0, 1.0),
            [0.2, 0.2, 0.2]
        ))
        l4 = Eva.add_material(Material(
            1.0,
            0.0,
            (1.0, 1.0, 1.0),
            [0.5, 0.5, 0.5],
        ))


        w1 = Box()
        w1.scale(70, 70, 1)
        w1.translate(3 * BOX_SIZE, 3 * BOX_SIZE, -6.5 * BOX_SIZE)
        w1.set_material(l1)
        self.add_geometry(w1)

        w2 = Box()
        w2.scale(1, 70, 70)
        w2.translate(-BOX_SIZE / 2.0, 3 * BOX_SIZE, -3 * BOX_SIZE)
        w2.set_material(l2)
        self.add_geometry(w2)

        w3 = Box()
        w3.scale(1, 70, 70)
        w3.translate(6.5 * BOX_SIZE, 3 * BOX_SIZE, -3 * BOX_SIZE)
        w3.set_material(l3)
        self.add_geometry(w3)

        w4 = Box()
        w4.scale(70, 70, 1)
        w4.translate(3 * BOX_SIZE, 3 * BOX_SIZE, 40)
        w4.set_material(l4)
        self.add_geometry(w4)



        def box_center(i, j):
            return [
                i * BOX_SIZE,
                -j * BOX_SIZE
            ]

        for i in range(7):
            for j in range(7):
                ball_mat = Eva.add_material(Material(
                    1.0,
                    0.3,
                    (1.0, 1.0, 1.0),
                    # [0.4, 0.4, 0.4]
                    texture=wood_handle
                ))

                ball = Box()
                height = random.randint(1, 10)
                ball.scale(BOX_SIZE, height, BOX_SIZE)
                ball.set_material(ball_mat)
                ball.translate(
                    box_center(i, j)[0],
                    height - height / 2,
                    box_center(i, j)[1],
                )

                self.add_geometry(ball)
        
        middle = box_center(3, 3)
        sz = 2.5
        for i in range(3):
            for j in range(3):
                for k in range(3):
                    c1 = Sphere()
                    c1.scale(sz, sz, sz)
                    center_mat = Eva.add_material(Material(
                        1.0 / 2.0 * i,
                        1.0 / 2.0 * j,
                        (1.0, 1.0, 1.0),
                    ))
                    c1.set_material(center_mat)
                    c1.translate(
                        middle[0] + 2 * i * (sz + 1) - 10,
                        30 + 2 * k * (sz + 1) - 10,
                        middle[1] + 2 * j * (sz + 1) - 25,
                    )
                    self.add_geometry(c1)

        middle = box_center(3, 3)
        sz = 2.5
        for i in range(3):
            for j in range(3):
                c1 = Mesh("pyramid.obj")
                c1.scale(sz, sz, sz * 2)
                c1.rotate_x(-90)
                center_mat = Eva.add_material(Material(
                    1.0,
                    0.0,
                    (1.0 / 2 * i, 1.0 / 2 * j, 1.0),
                ))
                c1.set_material(center_mat)
                c1.translate(
                    middle[0] + i * (sz + 1) - 1,
                    8,
                    middle[1] + j * (sz + 1) + BOX_SIZE + BOX_SIZE / 2 + 2,
                )
                self.add_geometry(c1)

        middle = box_center(1, 5)
        sz = 2.5
        for i in range(3):
            for j in range(3):
                c1 = Mesh("pyramid.obj")
                c1.scale(sz, sz, sz * 2)
                c1.rotate_x(-90)
                center_mat = Eva.add_material(Material(
                    1.0,
                    0.0,
                    (1.0 / 2 * i, 1.0, 1.0 / 2 * j),
                ))
                c1.set_material(center_mat)
                c1.translate(
                    middle[0] + i * (sz + 1) - 2,
                    10,
                    middle[1] + j * (sz + 1) + BOX_SIZE + BOX_SIZE / 2 + 2,
                )
                self.add_geometry(c1)



        middle = box_center(5, 5)
        sz = 2.5
        for i in range(3):
            for j in range(3):
                c1 = Mesh("pyramid.obj")
                c1.scale(sz, sz, sz * 2)
                c1.rotate_x(-90)
                center_mat = Eva.add_material(Material(
                    1.0,
                    0.0,
                    (1.0 / 2 * i, 1.0, 1.0 / 2 * j),
                ))
                c1.set_material(center_mat)
                c1.translate(
                    middle[0] + i * (sz + 1) - 2,
                    10,
                    middle[1] + j * (sz + 1) + BOX_SIZE + BOX_SIZE / 2 + 2,
                )
                self.add_geometry(c1)


    def handle_input(self, key, state):
        if state == "Pressed":
            if key == "W":
                self.camera.translate(0, 1.0, 0)
            if key == "A":
                self.camera.translate(1.0, 0.0, 0)
            if key == "D":
                self.camera.translate(-1.0, 0.0, 0)
            if key == "S":
                self.camera.translate(0, -1.0, 0)
            if key == "Q":
                self.camera.translate(0, 0.0, 1.0)
            if key == "E":
                self.camera.translate(0, 0.0, -1.0)

    def update(self):
        pass


ball = Cube()
ball.run()

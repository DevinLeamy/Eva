from eva_py import *

# class Cube(RenderDynamic):
class Cube(RenderStatic):
    def __init__(self):
        super().__init__()
        Eva.set_screenshot("./archive/cornell.png")

        self.camera = Camera((0, 0, 0), fov=42)
        self.camera.set_translation(-278, 273, -575)
        self.camera.look_at(-278, 273, -200)

        red = Eva.add_material(Material(1.0, 0.0, (0.65, 0.05, 0.05)))
        green = Eva.add_material(Material(1.0, 0.0, (0.12, 0.45, 0.15)))
        white = Eva.add_material(Material(1.0, 0.0, (0.73, 0.73, 0.73)))
        light = Eva.add_material(Material(1.0, 0.0, (1.0, 1.0, 1.0), [10.0, 10.0, 10.0]))


        box1 = Box() \
            .scale(165, 330, 165) \
            .translate(265 + 165 / 2, 330 / 2, 295 + 165 / 2) \
            .rotate_y(-15) \
            .set_material(white)
        box2 = Box() \
            .scale(165, 165, 165) \
            .rotate_y(18) \
            .translate(130 + 165 / 2, 165 / 2, 65 + 165 / 2) \
            .set_material(white)
        
        self.add_geometry(Box().scale(0.01, 555, 555).translate(555, 277.5, 277.5).set_material(green))
        self.add_geometry(Box().scale(0.01, 555, 555).translate(0, 277.5, 277.5).set_material(red))
        self.add_geometry(Box().scale(130, 0.01, 105).translate(343 -130/2, 554, 332 - 105/2).set_material(light))
        self.add_geometry(Box().scale(555, 0.01, 555).translate(277.5, 0, 277.5).set_material(white))
        self.add_geometry(Box().scale(555, 0.01, 555).translate(277.5, 555, 277.5).set_material(white))
        self.add_geometry(Box().scale(555, 555, 0.01).translate(277.5, 277.5, 555).set_material(white))

        self.add_geometry(box1)
        self.add_geometry(box2)

    def handle_input(self, key, state):
        speed = 10
        if state == "Pressed":
            if key == "W":
                self.camera.translate(0, speed, 0)
            if key == "A":
                self.camera.translate(speed, 0.0, 0)
            if key == "D":
                self.camera.translate(-speed, 0.0, 0)
            if key == "S":
                self.camera.translate(0, -speed, 0)
            if key == "Q":
                self.camera.translate(0, 0.0, speed)
            if key == "E":
                self.camera.translate(0, 0.0, -speed)

    def update(self):
        pass


ball = Cube()
ball.run()

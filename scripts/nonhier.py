from eva_py import *
import random

earth_handle = Eva.add_texture("earth.jpg")
wood_handle = Eva.add_texture("wood.jpeg")


Eva.add_skybox([
    "sky/z.tga",
    "sky/-z.tga",
    "sky/y.tga",
    "sky/-y.tga",
    "sky/x.tga",
    "sky/-x.tga",
])

mat1 = Material(1.0, 0.0, (0.3, 1.0, 0.3))
mat1.set_texture(earth_handle)
mat1 = Eva.add_material(mat1)

mat2 = Material(0.0, 1.0, (1.0, 1.0, 1.0))
mat2 = Eva.add_material(mat2)
mat3 = Eva.add_material(Material(1.0, 0.0, (1.0, 0.6, 0.1)))

mat4 = Material(1.0, 0.0, (0.7, 0.6, 1.0))
mat4.set_texture(wood_handle)
mat4 = Eva.add_material(mat4)

mat5 = Material(1.0, 0.0, (1.0, 1.0, 1.0))
mat5.set_texture(earth_handle)
mat5 = Eva.add_material(mat5)
light_mat = Eva.add_material(
    Material(0.0, 0.0, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)))
mat6 = Material(0.3, 1.0, (1.0, 1.0, 1.0))
mat6 = Eva.add_material(mat6)


class NonHier(RenderStatic):
    def __init__(self):
        super().__init__()
        # Eva.set_sample_count(100)
        # Eva.set_max_reflections(200)
        Eva.set_sample_count(20)
        Eva.set_max_reflections(100)
        Eva.set_screenshot(f"./archive/nonhier.png")

        self.camera.set_translation(0, 0, 800)

        s1 = Mesh('buckyball.obj')
        s1.set_material(mat1)
        s1.scale(45)
        s1.translate(-60, 0, -400)
        self.add_geometry(s1)

        s2 = Sphere(150)
        s2.set_material(mat5)
        s2.translate(200, 50, -100)
        self.add_geometry(s2)

        s3 = Sphere(1000)
        s3.set_material(mat2)
        s3.translate(0, -1200, -500)
        self.add_geometry(s3)

        b1 = Box()
        b1.set_material(mat4)
        b1.scale(120, 120, 120)
        b1.translate(-200, -125, 40)
        self.add_geometry(b1)

        s4 = Mesh("smstdodeca.obj")
        s4.set_material(mat3)
        s4.translate(-100, 25, -270)
        self.add_geometry(s4)

        steldodec = Mesh('catN.obj')
        steldodec.scale(10)
        steldodec.translate(-200, 200, -100)
        steldodec.rotate_x(-90)
        steldodec.rotate_y(90)
        steldodec.set_material(mat6)
        self.add_geometry(steldodec)


nonhier = NonHier()
nonhier.run()

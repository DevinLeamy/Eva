from eva_py import Mesh, Scene, Transform, Light, Camera, Material, Geometry, ray_trace

mat1 = Material((0.7, 1.0, 0.7), (0.5, 0.7, 0.5), 25)
mat2 = Material((0.5, 0.5, 0.5), (0.5, 0.7, 0.5), 25)
mat3 = Material((1.0, 0.6, 0.1), (0.5, 0.7, 0.5), 25)
mat4 = Material((0.7, 0.6, 1.0), (0.5, 0.4, 0.8), 25)
mat5 = Material((0.7, 1.0, 0.7), (0.0, 0.0, 0.0), 25)

scene = Scene()
earth = scene.add_texture("earth.jpg")
scene_root = Transform()

s1 = Mesh('buckyball.obj')
s1.set_material(mat1)
s1.scale(45, 45, 45)
s1.translate(0, 0, -400)
scene_root.add_child(s1)

s2 = Geometry('sphere')
s2.set_material(mat5)
s2.scale(150, 150, 150)
s2.translate(200, 50, -100)
s2.set_texture(earth)
scene_root.add_child(s2)

s3 = Geometry('sphere')
s3.set_material(mat2)
s3.scale(1000, 1000, 1000)
s3.translate(0, -1200, -500)
scene_root.add_child(s3)


b1 = Mesh('cube.obj')
b1.set_material(mat4)
b1.scale(90, 90, 90)
b1.translate(-200, -125, 0)
scene_root.add_child(b1)

s4 = Geometry('sphere')
s4.set_material(mat3)
s4.scale(50, 50, 50)
s4.translate(-100, 25, -300)
scene_root.add_child(s4)

s5 = Geometry('sphere')
s5.set_material(mat1)
s5.scale(25, 25, 25)
s5.translate(0, 100, -250)
scene_root.add_child(s5)

# steldodec = Mesh('smstdodeca.obj')
steldodec = Mesh('suzanne.obj')
steldodec.scale(100, 100, 100)
steldodec.translate(-200, 200, -100)
steldodec.set_material(mat3)
scene_root.add_child(steldodec)

white_light = Light((0.9, 0.9, 0.9), (1.0, 0, 0))
white_light.translate(-100.0, 150.0, 400.0)

magenta_light = Light((0.7, 0, 0.7), (1, 0, 0))
magenta_light.translate(400.0, 100.0, 150.0)

scene_root.add_child(white_light)
# scene_root.add_child(magenta_light)
camera = Camera((0, 0, 800), (0, 0, -1), (0, 1, 0), 50)

scene.set_root(scene_root)
scene.set_ambient(0.3, 0.3, 0.3)
scene.set_skybox([
    "blue/x.png",
    "blue/-x.png",
    "blue/y.png",
    "blue/-y.png",
    "blue/z.png",
    "blue/-z.png",
])

ray_trace(scene, camera)

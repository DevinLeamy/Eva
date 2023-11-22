from eva_py import TextureBuilder, Scene, Light, Camera, Material, Geometry, Eva, Mesh, Box, Sphere

mat1 = Material((0.7, 1.0, 0.7), (0.5, 0.7, 0.5), 25)
mat2 = Material((0.5, 0.5, 0.5), (0.5, 0.7, 0.5), 25)
mat3 = Material((1.0, 0.6, 0.1), (0.5, 0.7, 0.5), 25)
mat4 = Material((0.7, 0.6, 1.0), (0.5, 0.4, 0.8), 25)
mat5 = Material((0.7, 0.7, 0.5), (0.0, 0.0, 0.0), 25)

earth = TextureBuilder.build("earth.jpg")
wood = TextureBuilder.build("wood.jpeg")

scene = Scene()
scene.set_ambient(0.4)
scene.set_skybox([
    "blue/x.png",
    "blue/-x.png",
    "blue/y.png",
    "blue/-y.png",
    "blue/z.png",
    "blue/-z.png",
])

s1 = Mesh('buckyball.obj')
s1.set_material(mat1)
s1.scale(45)
s1.translate(0, 0, -400)
scene.add(s1)

s2 = Sphere(150)
s2.set_material(mat5)
s2.translate(200, 50, -100)
s2.set_texture(earth)
scene.add(s2)

s3 = Sphere(1000)
s3.set_material(mat2)
s3.translate(0, -1200, -500)
scene.add(s3)

b1 = Box()
b1.set_material(mat4)
b1.scale(120, 120, 120)
b1.translate(-200, -125, 40)
b1.set_texture(wood)
scene.add(b1)

s4 = Sphere(50)
s4.set_material(mat3)
s4.translate(-100, 25, -270)
scene.add(s4)

s5 = Sphere(25)
s5.set_material(mat1)
s5.translate(0, 100, -250)
scene.add(s5)

# steldodec = Geometry('smstdodeca.obj')
steldodec = Mesh('suzanne.obj')
steldodec.scale(100)
steldodec.translate(-200, 200, -100)
steldodec.set_material(mat5)
scene.add(steldodec)

white_light = Light(0.9)
white_light.translate(-100.0, 150.0, 400.0)

magenta_light = Light(0.4, 0.4, 0.7)
magenta_light.translate(400.0, 400.0, -200.0)

scene.add(white_light)
scene.add(magenta_light)

camera = Camera((0, 0, 800))

Eva.run()

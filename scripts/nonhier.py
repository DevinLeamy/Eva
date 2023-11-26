from eva_py import Scene, Camera, Material, Eva, Box, Sphere, Mesh

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

mat2 = Eva.add_material(Material(0.0, 1.0, (1.0, 1.0, 1.0)))
mat3 = Eva.add_material(Material(1.0, 0.0, (1.0, 0.6, 0.1)))

mat4 = Material(1.0, 0.0, (0.7, 0.6, 1.0))
mat4.set_texture(wood_handle)
mat4 = Eva.add_material(mat4)

mat5 = Eva.add_material(Material(1.0, 1.0, (1.0, 1.0, 1.0)))
light_mat = Eva.add_material(
    Material(0.0, 0.0, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)))

# Eva.add_skybox([
#     "blue/x.png",
#     "blue/-x.png",
#     "blue/y.png",
#     "blue/-y.png",
#     "blue/z.png",
#     "blue/-z.png",
# ])
Eva.set_ambient(0.4)

scene = Scene()

s1 = Mesh('buckyball.obj')
s1.set_material(mat1)
s1.scale(45)
s1.translate(0, 0, -400)
scene.add(s1)

s2 = Sphere(150)
s2.set_material(mat5)
s2.translate(200, 50, -100)
scene.add(s2)

s3 = Sphere(1000)
s3.set_material(mat2)
s3.translate(0, -1200, -500)
scene.add(s3)

b1 = Box()
b1.set_material(mat4)
b1.scale(120, 120, 120)
b1.translate(-200, -125, 40)
scene.add(b1)

s4 = Sphere(50)
s4.set_material(mat3)
s4.translate(-100, 25, -270)
scene.add(s4)

s5 = Sphere(25)
s5.set_material(mat1)
s5.translate(0, 100, -250)
scene.add(s5)

steldodec = Mesh('suzanne.obj')
steldodec.scale(100)
steldodec.translate(-200, 200, -100)
steldodec.set_material(mat5)
scene.add(steldodec)

# l1 = Sphere(50)
# l1.set_material(light_mat)
# l1.translate(-100.0, 150.0, 400.0)

# l2 = Sphere(50)
# l2.set_material(light_mat)
# l2.translate(400.0, 400.0, -400.0)

# scene.add(l1)
# scene.add(l2)

camera = Camera((0, 0, 800))


def update():
    pass


def handle_input(key, state):
    pass


Eva.run(update, handle_input)

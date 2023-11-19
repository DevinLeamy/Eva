from eva_py import Mesh, Scene, Transform, Light, Camera, Material, Geometry, ray_trace

scene = Scene()
spheres = scene.add_texture("spheres.png")
earth = scene.add_texture("earth.jpg")

mat1 = Material((0.7, 0.7, 0.7), (0.0, 0.0, 0.0), 25)
# mat1 = Material((0.7, 0.7, 0.7), (0.5, 0.5, 0.5), 25)
mat2 = Material((0.5, 0.5, 0.5), (0.5, 0.5, 0.5), 25)

scene_root = Transform()

s1 = Geometry('sphere')
s1.set_material(mat1)
s1.set_texture(earth)

s1.scale(100, 100, 100)
s1.translate(0, 0, -400)
scene_root.add_child(s1)

s2 = Geometry('sphere')
s2.set_material(mat1)
s2.set_texture(spheres)
s2.scale(150, 150, 150)
s2.translate(200, 50, -100)
scene_root.add_child(s2)

s3 = Geometry('sphere')
s3.set_material(mat2)
# s3.set_texture(earth)
s3.scale(1000, 1000, 1000)
s3.translate(0, -1200, -500)
scene_root.add_child(s3)

white_light = Light((0.9, 0.9, 0.9), (1.0, 0, 0))
white_light.translate(-100.0, 150.0, 400.0)

scene_root.add_child(white_light)
camera = Camera((0, 0, 800), (0, 0, -1), (0, 1, 0), 50)

scene.set_root(scene_root)
scene.set_ambient(0.3, 0.3, 0.3)

ray_trace(scene, camera, 256, 256, "image.png")

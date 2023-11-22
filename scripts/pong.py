from eva_py import TextureBuilder, Scene, Transform, Geometry, Light, ray_trace, Camera, Material, Eva

AMBIENT = 0.5
wood = TextureBuilder.build("wood.jpeg")

ball_mat = Material(
    (0.5, 0.0, 0.0),
    (0.5, 0.5, 0.5),
    10
)
paddle_mat = Material(
    (0.5, 0.5, 0.5),
    (0.0, 0.0, 0.0),
    0
)
background_mat = Material(
    (1.0, 1.0, 1.0),
    (1.0, 1.0, 1.0),
    10
)

ball_size = 10
paddle_width = 20
paddle_height = 5

Eva.run()

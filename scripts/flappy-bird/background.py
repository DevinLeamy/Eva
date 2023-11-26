from eva_py import Material, Box, Eva

class Background:
    def __init__(self, size: float):
        texture_handle = Eva.add_texture("flap.png")
        material = Material(
            roughness=1.0,
            metallic=0.0,
            albedo=(0.0, 0.0, 0.0)
        )
        material.set_texture(texture_handle)
        self.material_handle = Eva.add_material(material)
        self.geometry = Box() \
            .scale(size, size, 1) \
            .translate(0.0, 0.0, -10) \
            .set_material(self.material_handle)


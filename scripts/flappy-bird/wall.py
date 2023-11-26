from eva_py import Box, Eva, Material

WALL_DEPTH = 120


class Wall:
    def __init__(self, width, height, y):
        material = Material(
            roughness=1.0,
            metallic=1.0,
            albedo=(0.5, 0.7, 1.0)
        )
        self.material_handle = Eva.add_material(material)
        self.geometry = Box() \
            .scale(width, height, WALL_DEPTH) \
            .translate(0.0, y, 0.0) \
            .set_material(self.material_handle)

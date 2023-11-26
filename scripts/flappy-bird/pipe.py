from eva_py import Box, Material, Eva, Mesh
import random

SPEED = 0.75
PIPE_DEPTH = 30


class PipePair:
    def __init__(self, width: float, height: float, x: float, window: (float, float), gap_size: float, max_x: float):
        self.max_x = max_x
        self.gap_size = gap_size
        self.window = window

        material = Material(
            roughness=1.0,
            metallic=1.0,
            albedo=(1.0, 1.0, 1.0)
        )
        self.material_handle = Eva.add_material(material)
        # self.top_geometry = Mesh("cylinder.obj") \
        self.top_geometry = Box() \
            .scale(width, height, PIPE_DEPTH) \
            .set_material(self.material_handle)
        # self.bottom_geometry = Mesh("cylinder.obj") \
        self.bottom_geometry = Box() \
            .scale(width, height, PIPE_DEPTH) \
            .set_material(self.material_handle)

        shift = random.randint(-self.gap_size, self.gap_size)
        self.top_geometry.translate(x, window[0] + shift, 0.0)
        self.bottom_geometry.translate(x, window[1] + shift, 0.0)

    def update(self):
        self.top_geometry.translate(-SPEED, 0.0, 0.0)
        self.bottom_geometry.translate(-SPEED, 0.0, 0.0)

        if self.should_reposition():
            self.reposition()

    def should_reposition(self) -> bool:
        x = self.top_geometry.translation()[0]
        return x > self.max_x

    def reposition(self):
        shift = random.randint(-self.gap_size, self.gap_size)

        self.top_geometry.set_translation(-self.max_x, self.window[0] + shift, 0.0)
        self.bottom_geometry.set_translation(-self.max_x, self.window[1] + shift, 0.0)

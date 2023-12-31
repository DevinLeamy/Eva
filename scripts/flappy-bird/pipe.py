from eva_py import Box, Material, Eva, Mesh
import random

SPEED = 1.25
PIPE_DEPTH = 120


class PipePair:
    def __init__(self, width: float, height: float, x: float, window: (float, float), gap_size: float, max_x: float, light):
        self.max_x = max_x
        self.starting_x = x
        self.gap_size = gap_size
        self.window = window

        material = Material(
            roughness=0.0,
            metallic=1.0,
            albedo=(0.8, 1.0, 0.8),
            light=(light, light, light)
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

        self.position_pipes(self.starting_x)

    def update(self):
        self.top_geometry.translate(-SPEED, 0.0, 0.0)
        self.bottom_geometry.translate(-SPEED, 0.0, 0.0)

        if self.should_reposition():
            self.reposition()

    def should_reposition(self) -> bool:
        x = self.top_geometry.translation()[0]
        return x > self.max_x
    
    def reset(self):
        self.position_pipes(self.starting_x)

    def position_pipes(self, x):
        shift = random.randint(-self.gap_size, self.gap_size)
        self.top_geometry.set_translation(-x, shift + self.window[0] + shift, 0.0)
        self.bottom_geometry.set_translation(-x, shift + self.window[1] + shift, 0.0)


    def reposition(self):
        self.position_pipes(self.max_x)

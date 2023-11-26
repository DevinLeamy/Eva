from eva_py import Box, Sphere, Material, Eva

FALL_SPEED = 0.6
JUMP = "Space"
JUMP_SPEED = 8.0
TERMINAL_FALL_SPEED = 4.0
# One second.
JUMP_COOLDOWN = 0.2 

BIRD_Z = 30


class Bird:
    # Y-velocity of the bird.
    velocity: float
    # Time (in seconds) until the bird can jump.
    jump_cooldown: float

    def __init__(self):
        material = Material(
            roughness=1.0,
            metallic=0.0,
            albedo=(0.0, 1.0, 1.0)
        )
        self.material_handle = Eva.add_material(material)
        self.geometry = Sphere().scale(10, 10, 10).translate(0.0, 0.0, BIRD_Z).set_material(self.material_handle)
        self.velocity = 0
        self.jump_cooldown = 0

    def reset(self):
        self.velocity = 0
        self.geometry.set_translation(0, 0, BIRD_Z)

    def handle_input(self, key: str, state: str):
        if key == JUMP and self.jump_cooldown == 0:
            self.velocity = min(JUMP_SPEED, self.velocity + JUMP_SPEED)
            self.jump_cooldown = JUMP_COOLDOWN

    def update(self, dt: float):
        self.geometry.translate(0.0, self.velocity, 0.0)
        self.velocity = max(-TERMINAL_FALL_SPEED, self.velocity - FALL_SPEED)
        self.jump_cooldown = max(0.0, self.jump_cooldown - dt)

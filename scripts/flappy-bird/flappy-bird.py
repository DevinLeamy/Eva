from eva_py import *
from bird import Bird
from wall import Wall
from background import Background
from pipe import PipePair
import datetime

Eva.add_skybox([
    "sky/z.tga",
    "sky/-z.tga",
    "sky/y.tga",
    "sky/-y.tga",
    "sky/x.tga",
    "sky/-x.tga",
])
# Eva.add_skybox([
#     "flap/flap.png",
#     "flap/flap.png",
#     "flap/flap.png",
#     "flap/flap.png",
#     "flap/flap.png",
#     "flap/flap.png",
# ])
# Eva.add_skybox([
#     "blue/x.png",
#     "blue/-x.png",
#     "blue/y.png",
#     "blue/-y.png",
#     "blue/z.png",
#     "blue/-z.png",
# ])


GAME_WIDTH = 220
GAME_HEIGHT = 175
WALL_HEIGHT = 50

PIPE_WIDTH = 20
PIPE_HEIGHT = 110


class FlappyBird(RenderDynamic):
    camera: Camera
    bird: Bird
    pipes: [PipePair]

    def __init__(self):
        super().__init__()
        self.bird = Bird()
        self.background = Background(GAME_WIDTH + 150, GAME_HEIGHT)
        self.top_wall = Wall(GAME_WIDTH + 70, WALL_HEIGHT, GAME_HEIGHT * 0.75)
        self.bottom_wall = Wall(
            GAME_WIDTH + 70, WALL_HEIGHT, -GAME_HEIGHT * 0.5)
        self.camera.translate(100, 0, -5)
        self.camera.look_at(0, 0, 0)

        self.pipes = []
        for i in range(3):
            light = 0.0
            if i % 2 == 1:
                light = 0.0

            self.pipes.append(PipePair(
                PIPE_WIDTH,
                PIPE_HEIGHT,
                (i + 1) * 80,
                (2 * GAME_HEIGHT / 3, -GAME_HEIGHT / 2),
                30,
                GAME_WIDTH - 80,
                light
            ))
            self.add_geometry(self.pipes[-1].top_geometry)
            self.add_geometry(self.pipes[-1].bottom_geometry)

        self.add_geometry(self.bird.geometry)
        self.add_geometry(self.top_wall.geometry)
        self.add_geometry(self.bottom_wall.geometry)
        self.add_geometry(self.background.geometry)

        self.last_time = datetime.datetime.now()

    def reset_game(self):
        self.bird.reset()
        for pipe in self.pipes:
            pipe.reset()

    def handle_input(self, key: str, state: str):
        self.bird.handle_input(key, state)

    def update(self):
        now = datetime.datetime.now()
        dt = (now - self.last_time).total_seconds()
        self.bird.update(dt)

        for pipe in self.pipes:
            pipe.update()

        if self.is_game_over():
            self.reset_game()

        self.last_time = now

    def is_game_over(self):
        # Check if the bird intersected with either of the walls.
        if (self.bird.hitbox.intersects_with(self.top_wall.geometry)):
            return True
        if (self.bird.hitbox.intersects_with(self.bottom_wall.geometry)):
            return True

        for pipe in self.pipes:
            if self.bird.hitbox.intersects_with(pipe.top_geometry):
                return True
            if self.bird.hitbox.intersects_with(pipe.bottom_geometry):
                return True

        return False


flappy_bird = FlappyBird()
flappy_bird.run()

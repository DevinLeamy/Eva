from eva_py import Render, Camera, Eva
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

GAME_WIDTH = 200
GAME_HEIGHT = 175
WALL_HEIGHT = 10

PIPE_WIDTH = 20
PIPE_HEIGHT = 110


class FlappyBird(Render):
    camera: Camera
    bird: Bird
    pipes: [PipePair]

    def __init__(self):
        super().__init__()
        self.bird = Bird()
        self.background = Background(GAME_WIDTH)
        self.top_wall = Wall(GAME_WIDTH, WALL_HEIGHT, GAME_HEIGHT / 2)
        self.bottom_wall = Wall(GAME_WIDTH, WALL_HEIGHT, -GAME_HEIGHT / 2)

        self.pipes = []
        for i in range(3):
            self.pipes.append(PipePair(
                PIPE_WIDTH,
                PIPE_HEIGHT,
                (i + 1) * 80,
                (GAME_HEIGHT / 2, -GAME_HEIGHT / 2),
                30,
                GAME_WIDTH - 80
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
        if (self.bird.geometry.intersects_with(self.top_wall.geometry)):
            return True
        if (self.bird.geometry.intersects_with(self.bottom_wall.geometry)):
            return True

        return False


flappy_bird = FlappyBird()
flappy_bird.run()

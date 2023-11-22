from .eva_py import ray_trace
from eva_py.camera import Camera
from eva_py.scene import Scene


class Eva:
    @staticmethod
    def run():
        scene = Scene()
        camera = Camera()

        ray_trace(scene.build(), camera.inner)

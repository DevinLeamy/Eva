from .eva_py import eva_main, EvaGlobal
from eva_py.camera import Camera
from eva_py.scene import Scene
from eva_py.utils import Singleton
from eva_py.material import Material


class Eva(Singleton):
    inner: EvaGlobal

    def init(self):
        self.inner = EvaGlobal()

    @staticmethod
    def run(update, handle_input):
        eva_main(Eva().inner, Scene(), Camera(), update, handle_input)

    @staticmethod
    def add_texture(name: str) -> int:
        return Eva().inner.add_texture(name)

    @staticmethod
    def add_skybox(images: [str]):
        Eva().inner.add_skybox(images)

    @staticmethod
    def set_ambient(strength: float):
        Eva().inner.set_ambient(strength, strength, strength)

    @staticmethod
    def add_material(material: Material) -> int:
        return Eva().inner.add_material(material.inner)

from .eva_py import eva_main, EvaGlobal
from eva_py.camera import Camera
from eva_py.scene import Scene
from eva_py.utils import Singleton
from eva_py.material import Material
from eva_py.sphere import Sphere


class Eva(Singleton):
    inner: EvaGlobal

    def init(self):
        self.inner = EvaGlobal()

    @staticmethod
    def run(render):
        eva_main(Eva().inner, Scene(), Camera(), render)

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

    @staticmethod
    def add_light(colour: (float, float, float), position: (float, float, float)):
        light_material = Material(0, 0, (0, 0, 0))
        light_material.set_light((colour[0], colour[1], colour[2]))
        material_id = Eva.add_material(light_material)
        light_node = Sphere(10)
        light_node.translate(position[0], position[1], position[2])
        light_node.set_material(material_id)

        Scene().add(light_node)

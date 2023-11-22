from eva_py.utils import Singleton
from .eva_py import EvaScene, Transform
from eva_py.node import Node


class Scene(Singleton):
    inner: EvaScene

    def init(self):
        self.inner = EvaScene()
        self.root = Transform()

    def add(self, node: Node):
        self.root.add_child(node.inner)

    def set_ambient(self, strength):
        self.inner.set_ambient(strength, strength, strength)

    def set_skybox(self, images: [str]):
        self.inner.set_skybox(images)

    def build(self):
        self.inner.set_root(self.root)
        return self.inner

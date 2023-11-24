from eva_py.utils import Singleton
from .eva_py import EvaScene, Transform
from eva_py.node import Node


class Scene(Singleton):
    _inner: EvaScene

    children: [Node]

    def init(self):
        self._inner = EvaScene()
        self.children = []

    def add(self, node: Node):
        self.children.append(node)

    def set_ambient(self, strength):
        self._inner.set_ambient(strength, strength, strength)

    def inner(self) -> EvaScene:
        # Construct the entire scene hierarchy.
        root = Transform()
        for node in self.children:
            root.add_child(node.inner())

        self._inner.set_root(root)
        return self._inner

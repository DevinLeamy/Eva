from .eva_py import Geometry
from eva_py.geometry_node import GeometryNode


class Sphere(GeometryNode):
    def __init__(self, radius=1):
        super().__init__(Geometry("sphere"))
        self.scale(radius)

from .eva_py import Geometry
from eva_py.geometry_node import GeometryNode


class Mesh(GeometryNode):
    def __init__(self, name: str):
        super().__init__(Geometry(name))

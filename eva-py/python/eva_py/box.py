from .eva_py import Geometry
from eva_py.geometry_node import GeometryNode


class Box(GeometryNode):
    def __init__(self):
        super().__init__(Geometry("cube"))

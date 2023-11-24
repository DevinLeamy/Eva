from .eva_py import Geometry
from eva_py.box import Box
from eva_py.geometry_node import GeometryNode


class Sphere(GeometryNode):
    def __init__(self, radius=1):
        super().__init__(Geometry("sphere"))
        self.scale(radius)
    
    # Check if the Sphere intersects with the Box.
    def intersects_with(self, box: Box) -> bool:
        pass

from .eva_py import Geometry
from eva_py.box import Box
from eva_py.geometry_node import GeometryNode


class Sphere(GeometryNode):
    def __init__(self, radius=1):
        super().__init__(Geometry("sphere"))
        self.scale(radius)

    # Check if the Sphere intersects with the Box.
    def intersects_with(self, box: Box) -> bool:
        radius = self.radius()
        center = self.translation()

        box_size = box.size()
        box_center = box.translation()

        box_min = [box_center[i] - box_size[i] / 2 for i in range(3)]
        box_max = [box_center[i] + box_size[i] / 2 for i in range(3)]

        for i in range(3):
            if center[i] < box_min[i] - radius or center[i] > box_max[i] + radius:
                return False

        return True

    def radius(self) -> float:
        return self.inner().get_scale()[0]

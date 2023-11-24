from .eva_py import EvaLight
from eva_py.node import Node


class Light(Node):
    def __init__(self, r: float, g = None, b = None):
        light = None
        if g != None and b != None:
            light = EvaLight((r, g, b), (1.0, 0.0, 0.0))
        else:
            light = EvaLight((r, r, r), (1.0, 0.0, 0.0))
        super().__init__(light)

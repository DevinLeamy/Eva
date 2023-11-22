class Node:
    inner = None

    def __init__(self, inner):
        self.inner = inner

    def translate(self, x: float, y: float, z: float):
        self.inner.translate(x, y, z)

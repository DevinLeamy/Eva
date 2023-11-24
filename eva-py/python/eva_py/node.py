class Node:
    _inner = None

    def __init__(self, inner):
        self._inner = inner

    def inner(self) -> any:
        return self._inner

    def translate(self, x: float, y: float, z: float):
        self.inner().translate(-x, y, z)

    def rotate_y(self, degrees: float):
        self.inner().rotate("y", degrees)

    def translation(self) -> [float]:
        return self.inner().translation()

    def set_translation(self, x: float, y: float, z: float):
        self.inner().set_translation(x, y, z)

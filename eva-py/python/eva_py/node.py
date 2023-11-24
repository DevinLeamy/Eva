class Node:
    _inner = None

    def __init__(self, inner):
        self._inner = inner

    def translate(self, x: float, y: float, z: float):
        self._inner.translate(x, y, z)

    def rotate_y(self, degrees: float):
        self._inner.rotate("y", degrees)

    def inner(self) -> any:
        return self._inner

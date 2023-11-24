class Node:
    _inner = None

    def __init__(self, inner):
        self._inner = inner

    def translate(self, x: float, y: float, z: float):
        self._inner.translate(x, y, z)

    def inner(self) -> any:
        return self._inner

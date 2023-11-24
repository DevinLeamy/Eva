from eva_py.utils import Singleton
from .eva_py import EvaCamera


class Camera(Singleton):
    inner: EvaCamera

    def init(self, translation):
        self.inner = EvaCamera(translation, (0, 0, -1), (0, 1, 0), 50)

    def look_at(self, x, y, z):
        self.inner.look_at(x, y, z)

    def translate(self, x, y, z):
        print("Translate Camera", x, y, z)
        self.inner.translate(x, y, z)

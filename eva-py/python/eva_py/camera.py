from eva_py.utils import Singleton
from .eva_py import EvaCamera


class Camera(Singleton):
    inner: EvaCamera

    def init(self, translation, fov=None):
        if fov == None:
            fov = 50
        self.inner = EvaCamera(translation, (0, 0, -1), (0, 1, 0), fov)

    def look_at(self, x, y, z):
        self.inner.look_at(x, y, z)

    def translate(self, x, y, z):
        self.inner.translate(x, y, z)
    
    def set_translation(self, x, y, z):
        self.inner.set_translation(x, y, z)

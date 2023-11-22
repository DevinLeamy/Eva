from eva_py.utils import Singleton
from .eva_py import EvaCamera


class Camera(Singleton):
    inner: EvaCamera

    def init(self, translation):
        self.inner = EvaCamera(translation, (0, 0, -1), (0, 1, 0), 50)

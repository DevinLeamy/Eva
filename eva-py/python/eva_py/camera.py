from eva_py.utils import Singleton
from .eva_py import EvaCamera


class Camera(Singleton):
    inner: EvaCamera

    def init(self):
        self.inner = EvaCamera((0, 0, 800), (0, 0, -1), (0, 1, 0), 50)

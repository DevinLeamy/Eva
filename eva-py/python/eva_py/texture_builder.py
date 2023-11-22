from eva_py.utils import Singleton
from eva_py.scene import Scene


class TextureBuilder:
    @staticmethod
    def build(name: str) -> int:
        return Scene().inner.add_texture(name)

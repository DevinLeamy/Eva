from .eva_py import EvaMaterial


class Material():
    def __init__(self, roughness, metallic, albedo):
        self.inner = EvaMaterial(roughness, metallic, albedo)

    def set_texture(self, texture_id: int):
        self.inner.set_texture(texture_id)

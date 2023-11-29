from .eva_py import EvaMaterial


class Material():
    def __init__(self, roughness, metallic, albedo, light=None, texture=None):
        self.inner = EvaMaterial(roughness, metallic, albedo)
        if (light != None):
            self.set_light(light)
        if texture != None:
            self.inner.set_texture(texture)

    def set_texture(self, texture_id: int):
        self.inner.set_texture(texture_id)

    def set_light(self, light: [float]):
        self.inner.set_light((light[0], light[1], light[2]))

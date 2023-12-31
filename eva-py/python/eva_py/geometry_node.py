from eva_py.node import Node


class GeometryNode(Node):
    def __init__(self, inner):
        super().__init__(inner)

    def scale(self, x, y=None, z=None):
        if y != None and z != None:
            self.inner().scale(x, y, z)
        else:
            self.inner().scale(x, x, x)
        
        return self

    def set_material(self, material):
        self.inner().set_material(material)
        return self

    def set_texture(self, texture: int):
        self.inner().set_texture(texture)
        return self

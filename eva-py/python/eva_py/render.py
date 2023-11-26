from eva_py import Eva, Scene

class Render:
    def __init__(self):
        self.scene = Scene()

    def update(self):
        print("Updating")

    def handle_input(self, key: str, state: str):
        print("key")
    
    # Add geometry to the scene.
    def add_geometry(self, geometry):
        self.scene.add(geometry)

    # Generate a material handle.
    def add_material(self, material) -> int:
        pass
    
    # Generate a texture handle.
    def add_texture(self, texture) -> int:
        pass

    def run(self):
        Eva.run(self)

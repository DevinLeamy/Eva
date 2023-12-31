from eva_py import Eva, Scene, Camera


class RenderDynamic:
    scene: Scene
    camera: Camera

    def __init__(self):
        self.scene = Scene()
        self.camera = Camera((0, 0, 200))
        self.camera.look_at(0, 0, 0)
        Eva.set_max_reflections(10)
        Eva.set_sample_count(9)

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
        Eva.run_dynamic(self)


class RenderStatic:
    scene: Scene
    camera: Camera

    def __init__(self):
        self.scene = Scene()
        self.camera = Camera((0, 0, 200))
        self.camera.look_at(0, 0, 0)
        Eva.set_max_reflections(45)
        Eva.set_sample_count(81)

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
        Eva.run_static()

# Eva

> Realtime raytracing that will make your computer sweat :)

<p align="center">
  <img src="./about/assets/reflection1.png" width="350" height="350">
</p>

### Structure

-   `/eva`: Core renderer.
-   `/eva-macros`: Marcos for the core renderer.
-   `/eva-py`: Python3 scripting API.
-   `/eva-py/python/eva_py`: Pure Python3 wrapper on the Rust bindings.
-   `/eva-py-macros`: Macros for the Python3 scripting API.
-   `/scripts`: Python3 scripts.

### Setup

> _Note: You will need to have Rust and Python3 installed on your system._

Create a virtual environment and install `maturin`.

```bash
cd eva-py
python3 -m venv .env
source .env/bin/activate
pip install maturin
deactivate
```

### Usage

Scripts, which are python3 files, are put in `/scripts`.

To run a script `./scripts/custom-script.py`, from the root folder, call

```bash
# No debug output.
./run.sh custom-script
# With debug output.
./debug.sh custom-script
```

Or, alternatively:

```bash
cd eva-py
source .env/bin/activate
maturin develop
cd ..
python3 ./scripts/custom-script.py
```

### Dependencies

-   [`wgpu`](https://github.com/gfx-rs/wgpu): Rust implementation of the WebGPU specification.
-   [`pyo3`](https://github.com/PyO3/pyo3): Rust to Python3 bindings.
-   [`maturin`](https://github.com/PyO3/maturin): Python3 package builder for pyo3 generated bindings.
-   [`nalgebra`](https://github.com/dimforge/nalgebra): Rust linear algebra.
-   [`winit`](https://github.com/rust-windowing/winit): Rust cross-platform windowing.
-   [`pollster`](https://github.com/zesterer/pollster): Rust crate for statically resolving `Future`s.
-   [`bytemuck`](https://github.com/Lokathor/bytemuck): Rust crate for converting data types into bytes.
-   [`image`](https://github.com/image-rs/image): Rust crate for encoding and decoding images.
-   [`encase`](https://github.com/teoxoy/encase): Rust crate for byte-aligning structures for use in WebGPU shaders.
-   [`obj`](https://github.com/simnalamburt/obj-rs): Rust crate for loading `.obj` meshes.
-   [`half`](https://github.com/starkat99/half-rs): Rust crate for handling 16-bit floating float numbers.

### Resources

-   https://github.com/amengede/webgpu-for-beginners/tree/main
-   https://cohost.org/mcc/post/1406157-i-want-to-talk-about
-   https://surma.dev/things/webgpu/
-   https://sotrh.github.io/learn-wgpu/#what-is-wgpu
-   https://gpuweb.github.io/gpuweb/
-   https://github.com/gfx-rs/wgpu

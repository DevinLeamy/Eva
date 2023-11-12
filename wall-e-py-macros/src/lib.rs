extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PyNode)]
pub fn node_methods_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        impl #name {
            fn add_child(&mut self, py: Python, child: PyObject) {
                if let Ok(child) = child.extract::<PyRef<PyGeometry>>(py) {
                    self.inner.add_child(child.inner.clone().into());
                } else if let Ok(child) = child.extract::<PyRef<PyTransform>>(py) {
                    self.inner.add_child(child.inner.clone().into());
                } else if let Ok(child) = child.extract::<PyRef<PyLight>>(py) {
                    self.inner.add_child(child.inner.clone().into());
                } else if let Ok(child) = child.extract::<PyRef<PyMesh>>(py) {
                    self.inner.add_child(child.inner.clone().into());
                } else {
                    panic!("add_child only accepts PyGeometry, PyTransform, PyLight, or PyMesh");
                }
            }

            fn scale(&mut self, x: f32, y: f32, z: f32) {
                self.inner
                    .transform_mut()
                    .scale_nonuniform(Vector3::new(x, y, z));
            }

            fn translate(&mut self, x: f32, y: f32, z: f32) {
                self.inner.transform_mut().translate(Vector3::new(x, y, z));
            }

            fn rotate(&mut self, axis: &str, degrees: f32) {
                let rad = degrees.to_radians();
                match axis {
                    "x" | "X" => self.inner.transform_mut().rotate_x(rad),
                    "y" | "Y" => self.inner.transform_mut().rotate_y(rad),
                    "z" | "Z" => self.inner.transform_mut().rotate_z(rad),
                    _ => panic!("Invalid axis: {axis}")
                }
            }
        }
    };

    TokenStream::from(expanded)
}

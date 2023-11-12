use std::sync::Mutex;

pub static RENDER_BOUNDING_VOLUMES: Mutex<bool> = Mutex::new(false);

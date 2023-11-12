pub trait Buffer: Clone {
    type Value;

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn set(&mut self, x: u32, y: u32, value: Self::Value);

    fn get(&self, x: u32, y: u32) -> Self::Value;

    fn aspect(&self) -> f32 {
        self.width() as f32 / self.height() as f32
    }
}

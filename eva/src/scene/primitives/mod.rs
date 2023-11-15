mod cube;
mod sphere;
mod triangle;

pub use cube::*;
pub use sphere::*;
pub use triangle::*;

pub trait Collidable: CollidableClone + Send + std::fmt::Debug + Sync {
    fn foo(&self) {}
}

pub trait CollidableClone {
    fn clone_collidable(&self) -> Box<dyn Collidable>;
}

impl<T> CollidableClone for T
where
    T: 'static + Collidable + Clone,
{
    fn clone_collidable(&self) -> Box<dyn Collidable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Collidable> {
    fn clone(&self) -> Box<dyn Collidable> {
        self.clone_collidable()
    }
}

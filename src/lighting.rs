use crate::geometry::{Operations, Point3};
use crate::object::Object;

pub struct PointLight {
    pub position: Point3
}

pub struct ParallelLight{
    pub angle: f64
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>
}

pub trait Camera {
    fn render (&self, object: &impl Object, screen: &mut Screen);
}

impl Camera for PointLight {
    fn render(&self, _object: &impl Object, mut _screen: &mut Screen) {
        todo!("Implement this function");
    }
}

impl Camera for ParallelLight {
    fn render(&self, object: &impl Object, screen: &mut Screen) {
        for (i, color) in  screen.buffer.iter_mut().enumerate() {
            let x = i as f64 / screen.width as f64;
            let y = i as f64 % screen.width as f64;

            
            if (*object).inside([x, y]) {
                let points = (*object).surface([x, y]);
                let normal = (*object).surface_normal(points[0]);
                let cos = normal.dot([0., 0., 1.]).abs();
                println!("{:?}", cos);
                *color = (200.*(1. - cos*cos).sqrt()) as u32;
            }
        }
    }

}

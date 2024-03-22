use crate::geometry::*;

pub struct Sphere {
    centre: Point3, 
    radius: f64
}

pub trait Object {
    fn inside(&self, coord: Point2) -> bool;
    fn surface_normal(&self, coord: Point3) -> Vector3;
    fn surface(&self, coord: Point2) -> Vec<Vector3>;
}

impl Sphere {
    pub fn new (centre: Point3, radius: f64) -> Sphere{
        Sphere { centre, radius}
    }

    fn _render(self, _buffer: Vec<u32>){
        ()
    }
}

impl Object for Sphere {
    fn inside(&self, coord: Point2) -> bool {
        let x_offset = coord[0] - self.centre[0];
        let y_offset = coord[1] - self.centre[1];
        self.radius*self.radius  - x_offset*x_offset - y_offset*y_offset >= 0. 
    }

    fn surface_normal(&self, coord: Point3) -> Vector3 {
        [coord[0] - self.centre[0], coord[1] - self.centre[1], coord[2] - self.centre[2]].normalize()
    }

    fn surface(&self, coord: Point2) -> Vec<Vector3> {
        assert!(self.inside(coord));
        let x_offset = coord[0] - self.centre[0];
        let y_offset = coord[1] - self.centre[1];
        let z_offset = (self.radius*self.radius - x_offset*x_offset - y_offset*y_offset).sqrt();
        let normal = [coord[0], coord[1], z_offset];
        vec![normal]
    }
}
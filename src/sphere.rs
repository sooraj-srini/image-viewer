pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Point2 {
    x: f64,
    y: f64,
}

pub struct Sphere {
    centre: Point3, 
    radius: f64
}

impl Sphere {
    pub fn new (centre: Point3, radius: f64) -> Sphere{
        Sphere { centre, radius}
    }

    fn render(self, buffer: Vec<u32>){
        ()
    }
}
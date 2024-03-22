pub type Point3 = [f64; 3];

pub type Point2  = [f64; 2];

pub type Vector3  = [f64; 3];

pub trait Operations {
    fn normalize(self) -> Self;
    fn magnitude(self) -> f64;
    fn dot(self, other: Self) -> f64;
    fn add(self, other: Self) -> Self;

}

impl Operations for Vector3 {
    fn magnitude(self) -> f64 {
        (self[0]*self[0] + self[1]*self[1] + self[2]*self[2]).sqrt()
    }
    fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        [self[0]/magnitude, self[1]/magnitude, self[2]/magnitude]
    }

    fn dot(self, other: Self) -> f64 {
        self[0]*other[0] + self[1]*other[1] + self[2]*other[2]    
    }

    fn add(self, other: Self) -> Self {
        [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
    }
}

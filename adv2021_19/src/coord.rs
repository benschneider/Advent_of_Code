use nalgebra::Vector3;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coord {
    pub fn distance(&self, other: &Coord) -> i64 {
        // returns the distance squared
        let x_dis = self.x - other.x;
        let y_dis = self.y - other.y;
        let z_dis = self.z - other.z;
        x_dis * x_dis + y_dis * y_dis + z_dis * z_dis
    }

    pub fn distance_manhattan(&self, other: &Coord) -> i64 {
        // returns the distance squared
        let x_dis = self.x - other.x;
        let y_dis = self.y - other.y;
        let z_dis = self.z - other.z;
        x_dis.abs() + y_dis.abs() + z_dis.abs()
    }

    pub fn offset_sub(&self, offset: &Coord) -> Coord {
        Coord {
            x: self.x - offset.x,
            y: self.y - offset.y,
            z: self.z - offset.z,
        }
    }

    pub fn offset_add(&self, offset: &Coord) -> Coord {
        Coord {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }

    pub fn new(x: i64, y: i64, z: i64) -> Coord {
        Coord { x, y, z }
    }

    pub fn into_vector(&self) -> Vector3<f64> {
        // convert to vector
        // using f64 as nalgebra uses f64 for most of its operations
        Vector3::new(self.x as f64, self.y as f64, self.z as f64)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

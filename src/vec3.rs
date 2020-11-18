#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point = Vec3;
pub type Color = Vec3;


#[allow(dead_code)]
impl Vec3 {
    pub fn zeros() -> Vec3 {
        Vec3{x: 0., y: 0., z: 0.}
    }
    
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x, y,  z}
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3{
            x: self[1]*rhs[2] - self[2]*rhs[1],
            y: self[2]*rhs[0] - self[0]*rhs[2],
            z: self[1]*rhs[0] - self[0]*rhs[2],
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn unit_vec(&self) -> Vec3 {
        self / self.length()
    }

    pub fn clamped(self, min: f64, max: f64) -> Vec3 {
        Vec3::new(clamp(self.x, min, max), clamp(self.y, min, max), clamp(self.z, min, max))
    }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min }
    else if x > max { max }
    else { x }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });

impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl_op_ex!(* |a: &Vec3, b: &f64| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b) });

impl_op_ex!(* |a: &f64, b: &Vec3| -> Vec3 { b * a });

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z) });

impl_op_ex!(/ |a: &Vec3, b: &f64| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b) });

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::ops::Index<u8> for Vec3 {
    type Output = f64;
    fn index(&self, index: u8) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index for Vec3: {}", index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(Vec3::new(1., 2., 3.), Vec3::new(1., 2., 3.));
    }

    #[test]
    fn test_zeros() {
        assert_eq!(Vec3::zeros(), Vec3::new(0., 0., 0.));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1., 2., 3.), Vec3::new(-1., -2., -3.));
    }

    #[test]
    fn test_add() {
        let mut a = Vec3::new(0., 1., 2.);
        let b = Vec3::new(0., 1., 2.);
        assert_eq!(a + b, Vec3::new(0., 2., 4.));
        a += b;
        assert_eq!(a, Vec3::new(0., 2., 4.));
    }

    #[test]
    fn test_mul_scalar() {
        assert_eq!(Vec3::new(0., 1., 2.) * 2., Vec3::new(0., 2., 4.));
        let mut a = Vec3::new(0., 1., 2.);
        a *= 2.;
        assert_eq!(a, Vec3::new(0., 2., 4.));
    }

    #[test]
    fn test_mul_vec() {
        let a = Vec3::new(0., 1., 2.);
        assert_eq!(a * a, Vec3::new(0., 1., 4.));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(2., 4., 6.) / 2., Vec3::new(1., 2., 3.));
        let mut a = Vec3::new(2., 4., 6.);
        a /= 2.;
        assert_eq!(a, Vec3::new(1., 2., 3.));
    }

    #[test]
    fn test_index() {
        let a = Vec3::new(0., 1., 2.);
        assert_eq!(a[0], 0.);
        assert_eq!(a[1], 1.);
        assert_eq!(a[2], 2.);
    }

    #[test]
    #[should_panic]
    fn test_invalid_index() {
        let a = Vec3::new(0., 1., 2.);
        assert_eq!(a[3], 3.);
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(1., 2., 3.).length(), (14. as f64).sqrt());
    }

    #[test]
    fn test_dot() {
        let a = Vec3::new(0., 1., 2.);
        assert_eq!(a.dot(&a), 5.);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(0., 1., 2.);
        let b = Vec3::new(2., 1., 0.);
        assert_eq!(a.cross(&b), Vec3::new(-2., 4., 2.));
    }
}

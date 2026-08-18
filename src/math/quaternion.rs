use crate::math::{Matrix4, Vec3, math};


#[derive(Clone, Copy)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl std::ops::Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, q: Quaternion) -> Quaternion {
        Quaternion::new(q.w * self, q.x * self, q.y * self, q.z * self)
    }
}

impl std::ops::Add<Quaternion> for Quaternion {
    type Output = Self;

    fn add(self, q: Self) -> Self {
        Self::new(self.w + q.w, self.x + q.x, self.y + q.y, self.z + q.z)
    }
}

impl std::ops::Div<f32> for Quaternion {
    type Output = Self;

    fn div(self, v: f32) -> Self {
        Self::new(self.w / v, self.x / v, self.y / v, self.z / v)
    }
}

impl Quaternion {
    pub const ZERO: Self = { Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 } };

    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn to_mat4(&self) -> Matrix4 {
        let qxx = self.x * self.x;
		let qyy = self.y * self.y;
		let qzz = self.z * self.z;
		let qxz = self.x * self.z;
		let qxy = self.x * self.y;
		let qyz = self.y * self.z;
		let qwx = self.w * self.x;
		let qwy = self.w * self.y;
		let qwz = self.w * self.z;

		let mut result = Matrix4::IDENTITY;

		result.values[0].x = 1.0 - 2.0 * (qyy +  qzz);
		result.values[0].y = 2.0 * (qxy + qwz);
		result.values[0].z = 2.0 * (qxz - qwy);

		result.values[1].x = 2.0 * (qxy - qwz);
		result.values[1].y = 1.0 - 2.0 * (qxx +  qzz);
		result.values[1].z = 2.0 * (qyz + qwx);

		result.values[2].x = 2.0 * (qxz + qwy);
		result.values[2].y = 2.0 * (qyz - qwx);
		result.values[2].z = 1.0 - 2.0 * (qxx +  qyy);

		return result;
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();

		if len <= 0.0 {
			return Self::new(1.0, 0.0, 0.0, 0.0);
		}

		let one_over_len = 1.0 / len;

		return Self::new(self.w * one_over_len, self.x * one_over_len, self.y * one_over_len, self.z * one_over_len);
    }

    pub fn length(&self) -> f32 {
        Self::dot(*self, *self).sqrt()
    }

    pub fn from_eulerv(rotation: Vec3) -> Self {
        Self::from_euler(rotation.x, rotation.y, rotation.z)
    }

    pub fn from_euler(x: f32, y: f32, z: f32) -> Self {
        let c = Vec3::new((x * 0.5).cos(), (y * 0.5).cos(), (z * 0.5).cos()) ;
        let s = Vec3::new((x * 0.5).sin(), (y * 0.5).sin(), (z * 0.5).sin()) ;

        return Self {
            w: c.x * c.y * c.z + s.x * s.y * s.z,
            x: s.x * c.y * c.z - c.x * s.y * s.z,
            y: c.x * s.y * c.z + s.x * c.y * s.z,
            z: c.x * c.y * s.z - s.x * s.y * c.z
        };
    }

    pub fn slerp(a: Self, b: Self, factor: f32) -> Self {
        let mut z = b;

        let mut cos_theta = Self::dot(a, b);

        if cos_theta < 0.0 {
            z = Quaternion::new(-b.w, -b.x, -b.y, -b.z);
            cos_theta = -cos_theta;
        }

        if cos_theta > 1.0 - f32::EPSILON {
			return Quaternion::new(
				math::lerp(a.w, z.w, factor),
				math::lerp(a.x, z.x, factor),
				math::lerp(a.y, z.y, factor),
				math::lerp(a.z, z.z, factor)
			);
		}

		let angle = cos_theta.acos();

		return (((1.0 - factor) * angle).sin() * a + (factor * angle).sin() * z) / angle.sin();
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z) + (a.w * b.w)
    }
}

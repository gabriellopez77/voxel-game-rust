use super::vec4::Vec4;
use super::vec3::Vec3;
use std::ops::Mul;

pub struct Matrix4 {
    pub values: [Vec4; 4]
}

impl Matrix4 {
    pub const ZERO: Matrix4 = Matrix4 { values: [Vec4::ZERO; 4] };
    pub const IDENTITY: Matrix4 = Matrix4 {
        values: [
            Vec4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0 },
            Vec4{ x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
            Vec4{ x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
            Vec4{ x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        ]
    };


    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let mut result = Matrix4::IDENTITY;

        result.values[0].x = 2.0 / (right - left);
        result.values[1].y = 2.0 / (top - bottom);
        result.values[2].z = -1.0;
        result.values[3].x = -(right + left) / (right - left);
        result.values[3].y = -(top + bottom) / (top - bottom);

        return result;
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let tan_half_fov = (fov / 2.0).tan();

        let mut result = Matrix4::ZERO;

        result.values[0].x = 1.0 / (aspect * tan_half_fov);
        result.values[1].y = 1.0 / tan_half_fov;
        result.values[2].z = - (far + near) / (far - near);
        result.values[2].w = - 1.0;
        result.values[3].z = - (2.0 * far * near) / (far - near);

        return result;
    }

    pub fn look_at(eye: Vec3, target: Vec3) -> Self {
        let f = (target - eye).normalized();
        let s = f.cross(Vec3::UP).normalized();
        let u = s.cross(f);
        
        let mut result = Matrix4::IDENTITY;
        
        result.values[0].x =  s.x;
        result.values[1].x =  s.y;
        result.values[2].x =  s.z;
        result.values[0].y =  u.x;
        result.values[1].y =  u.y;
        result.values[2].y =  u.z;
        result.values[0].z = -f.x;
        result.values[1].z = -f.y;
        result.values[2].z = -f.z;
        result.values[3].x = -s.dot(eye);
        result.values[3].y = -u.dot(eye);
        result.values[3].z =  f.dot(eye);
        
        return result;
    }
    
    pub fn as_ptr(&self) -> *const f32 { self.values[0].as_ptr() }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, other: Matrix4) -> Self {
        let src_a0 = self.values[0];
        let src_a1 = self.values[1];
        let src_a2 = self.values[2];
        let src_a3 = self.values[3];

        let src_b0 = &other.values[0];
        let src_b1 = &other.values[1];
        let src_b2 = &other.values[2];
        let src_b3 = &other.values[3];

        return Matrix4{ values: [
            src_a3 * src_b0.w + src_a2 * src_b0.z + src_a1 * src_b0.y + src_a0 * src_b0.x,
            src_a3 * src_b1.w + src_a2 * src_b1.z + src_a1 * src_b1.y + src_a0 * src_b1.x,
            src_a3 * src_b2.w + src_a2 * src_b2.z + src_a1 * src_b2.y + src_a0 * src_b2.x,
            src_a3 * src_b3.w + src_a2 * src_b3.z + src_a1 * src_b3.y + src_a0 * src_b3.x
        ]
        };
    }
}
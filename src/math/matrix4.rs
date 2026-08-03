use super::Vec4;
use super::Vec3;
use std::ops::Mul;


#[repr(C)]
#[derive(Copy, Clone, Default)]
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

    pub fn from(v1: Vec4, v2: Vec4, v3: Vec4, v4: Vec4) -> Self {
        Self { values: [v1, v2, v3, v4] }
    }


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
        let tan_half_fov = (fov.to_radians() / 2.0).tan();

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

    pub fn get_row0(&self) -> Vec4 { Vec4::new(self.values[0].x, self.values[1].x, self.values[2].x, self.values[3].x) }
    pub fn get_row1(&self) -> Vec4 { Vec4::new(self.values[0].y, self.values[1].y, self.values[2].y, self.values[3].y) }
    pub fn get_row2(&self) -> Vec4 { Vec4::new(self.values[0].z, self.values[1].z, self.values[2].z, self.values[3].z) }
    pub fn get_row3(&self) -> Vec4 { Vec4::new(self.values[0].w, self.values[1].w, self.values[2].w, self.values[3].w) }

    // pub fn inverse(&self) -> Self {
    //     let Coef00 = self.values[2].z * self.values[3].w - self.values[3].z * self.values[2].w;
    //     let Coef02 = self.values[1].z * self.values[3].w - self.values[3].z * self.values[1].w;
    //     let Coef03 = self.values[1].z * self.values[2].w - self.values[2].z * self.values[1].w;
    //
    //     let Coef04 = self.values[2].y * self.values[3].w - self.values[3].y * self.values[2].w;
    //     let Coef06 = self.values[1].y * self.values[3].w - self.values[3].y * self.values[1].w;
    //     let Coef07 = self.values[1].y * self.values[2].w - self.values[2].y * self.values[1].w;
    //
    //     let Coef08 = self.values[2].y * self.values[3].z - self.values[3].y * self.values[2].z;
    //     let Coef10 = self.values[1].y * self.values[3].z - self.values[3].y * self.values[1].z;
    //     let Coef11 = self.values[1].y * self.values[2].z - self.values[2].y * self.values[1].z;
    //
    //     let Coef12 = self.values[2].x * self.values[3].w - self.values[3].x * self.values[2].w;
    //     let Coef14 = self.values[1].x * self.values[3].w - self.values[3].x * self.values[1].w;
    //     let Coef15 = self.values[1].x * self.values[2].w - self.values[2].x * self.values[1].w;
    //
    //     let Coef16 = self.values[2].x * self.values[3].z - self.values[3].x * self.values[2].z;
    //     let Coef18 = self.values[1].x * self.values[3].z - self.values[3].x * self.values[1].z;
    //     let Coef19 = self.values[1].x * self.values[2].z - self.values[2].x * self.values[1].z;
    //
    //     let Coef20 = self.values[2].x * self.values[3].y - self.values[3].x * self.values[2].y;
    //     let Coef22 = self.values[1].x * self.values[3].y - self.values[3].x * self.values[1].y;
    //     let Coef23 = self.values[1].x * self.values[2].y - self.values[2].x * self.values[1].y;
    //
    //     let Fac0 = Vec4::new(Coef00, Coef00, Coef02, Coef03);
    //     let Fac1 = Vec4::new(Coef04, Coef04, Coef06, Coef07);
    //     let Fac2 = Vec4::new(Coef08, Coef08, Coef10, Coef11);
    //     let Fac3 = Vec4::new(Coef12, Coef12, Coef14, Coef15);
    //     let Fac4 = Vec4::new(Coef16, Coef16, Coef18, Coef19);
    //     let Fac5 = Vec4::new(Coef20, Coef20, Coef22, Coef23);
    //
    //     let Vec0 = Vec4::new(self.values[1].x, self.values[0].x, self.values[0].x, self.values[0].x);
    //     let Vec1 = Vec4::new(self.values[1].y, self.values[0].y, self.values[0].y, self.values[0].y);
    //     let Vec2 = Vec4::new(self.values[1].z, self.values[0].z, self.values[0].z, self.values[0].z);
    //     let Vec3 = Vec4::new(self.values[1].w, self.values[0].w, self.values[0].w, self.values[0].w);
    //
    //     let Inv0 = Vec1 * Fac0 - Vec2 * Fac1 + Vec3 * Fac2;
    //     let Inv1 = Vec0 * Fac0 - Vec2 * Fac3 + Vec3 * Fac4;
    //     let Inv2 = Vec0 * Fac1 - Vec1 * Fac3 + Vec3 * Fac5;
    //     let Inv3 = Vec0 * Fac2 - Vec1 * Fac4 + Vec2 * Fac5;
    //
    //     let SignA = Vec4::new(1.0, -1.0, 1.0, -1.0);
    //     let SignB = Vec4::new(-1.0, 1.0, -1.0, 1.0);
    //     let Inverse = Matrix4::from(Inv0 * SignA, Inv1 * SignB, Inv2 * SignA, Inv3 * SignB);
    //
    //     let Row0 = Vec4::new(Inverse.values[0].x, Inverse.values[1].x, Inverse.values[2].x, Inverse.values[3].x);
    //
    //     let Dot0 = self.values[0] * Row0;
    //     let Dot1 = (Dot0.x + Dot0.y) + (Dot0.z + Dot0.w);
    //
    //     let OneOverDeterminant = 1.0 / Dot1;
    //
    //     return Inverse * OneOverDeterminant;
    // }

    pub fn remove_translation(&self) -> Self {
        Self {
            values: [self.values[0], self.values[1], self.values[2], Vec4{x: 0.0, y: 0.0, z: 0.0, w: 1.0}],
        }
    }


    pub fn translatev(&mut self, value: Vec3) { self.translate(value.x, value.y, value.z); }
    pub fn scalev(&mut self, value: Vec3) { self.scale(value.x, value.y, value.z); }
    pub fn rotatev(&mut self, angle: f32, value: Vec3) { self.rotate(angle, value.x, value.y, value.z); }
    
    pub fn rotate_x(&mut self, angle: f32) { self.rotate(angle, 1.0, 0.0, 0.0); }
    pub fn rotate_y(&mut self, angle: f32) { self.rotate(angle, 0.0, 1.0, 0.0); }
    pub fn rotate_z(&mut self, angle: f32) { self.rotate(angle, 0.0, 0.0, 1.0); }
    
    pub fn rotatev_xyz(&mut self, rotation: Vec3) {
        self.rotate(rotation.x, 1.0, 0.0, 0.0);
        self.rotate(rotation.y, 0.0, 1.0, 0.0);
        self.rotate(rotation.z, 0.0, 0.0, 1.0);
    }

    pub fn rotate_xyz(&mut self, x: f32, y: f32, z: f32) {
        self.rotate(x, 1.0, 0.0, 0.0);
        self.rotate(y, 0.0, 1.0, 0.0);
        self.rotate(z, 0.0, 0.0, 1.0);
    }
    
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.values[3] = self.values[0] * x +
                         self.values[1] * y +
                         self.values[2] * z +
                         self.values[3];
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        let mut result = Matrix4::ZERO;

        result.values[0] = self.values[0] * x;
        result.values[1] = self.values[1] * y;
        result.values[2] = self.values[2] * z;
        result.values[3] = self.values[3];

        *self = result;
    }

    pub fn rotate(&mut self, angle: f32, dx: f32, dy: f32, dz: f32) {
        let a = angle.to_radians();
        let c = a.cos();
        let s = a.sin();

        let axis = Vec3::new(dx, dy, dz).normalized();
        let temp = axis * (1.0 - c);

        let mut rot = Matrix4::ZERO;

        rot.values[0].x = c + temp.x * axis.x;
        rot.values[0].y = temp.x * axis.y + s * axis.z;
        rot.values[0].z = temp.x * axis.z - s * axis.y;

        rot.values[1].x = temp.y * axis.x - s * axis.z;
        rot.values[1].y = c + temp.y * axis.y;
        rot.values[1].z = temp.y * axis.z + s * axis.x;

        rot.values[2].x = temp.z * axis.x + s * axis.y;
        rot.values[2].y = temp.z * axis.y - s * axis.x;
        rot.values[2].z = c + temp.z * axis.z;

        let mut result = Matrix4::ZERO;

        result.values[0] = self.values[0] * rot.values[0].x + self.values[1] * rot.values[0].y + self.values[2] * rot.values[0].z;
        result.values[1] = self.values[0] * rot.values[1].x + self.values[1] * rot.values[1].y + self.values[2] * rot.values[1].z;
        result.values[2] = self.values[0] * rot.values[2].x + self.values[1] * rot.values[2].y + self.values[2] * rot.values[2].z;
        result.values[3] = self.values[3];

        *self = result;
    }
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

impl Mul<Vec4> for Matrix4 {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Vec4 {
        let mul0 = self.values[0] * v.x;
        let mul1 = self.values[1] * v.y;
        let add0 = mul0 + mul1;
        let mul2 = self.values[2] * v.z;
        let mul3 = self.values[3] * v.w;
        let add1 = mul2 + mul3;
        let add2 = add0 + add1;

        return add2;
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Self;

    fn mul(self, v: f32) -> Self {
        Matrix4::from(
            self.values[0] * v,
        self.values[1] * v,
        self.values[2] * v,
        self.values[3] * v,

        )
    }
}

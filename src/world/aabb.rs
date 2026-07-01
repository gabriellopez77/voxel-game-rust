use crate::math::Vec3;


#[derive(Clone, Copy)]
pub struct Aabb {
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32
}

impl Aabb {
    pub const CUBE: Self = Self {
        x0: 0.0,
        y0: 0.0,
        z0: 0.0,
        x1: 1.0,
        y1: 1.0,
        z1: 1.0,
    };

    pub const EMPTY: Self = Self {
        x0: 0.0,
        y0: 0.0,
        z0: 0.0,
        x1: 0.0,
        y1: 0.0,
        z1: 0.0,
    };

    pub fn new(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
        Self {
            x0,
            y0,
            z0,
            x1,
            y1,
            z1
        }
    }

    pub fn expand(&self, xa: f32, ya: f32, za: f32) -> Self {
        let mut x0 = self.x0;
        let mut y0 = self.y0;
        let mut z0 = self.z0;
        let mut x1 = self.x1;
        let mut y1 = self.y1;
        let mut z1 = self.z1;

        if xa < 0.0 { x0 += xa };
        if xa > 0.0 { x1 += xa };

        if ya < 0.0 { y0 += ya };
        if ya > 0.0 { y1 += ya };

        if za < 0.0 { z0 += za };
        if za > 0.0 { z1 += za };

        return Aabb::new(x0, y0, z0, x1, y1, z1);
    }

    pub fn grow(&mut self, xa: f32, ya: f32, za: f32) {
        self.x1 += xa;
        self.y1 += ya;
        self.z1 += za;
    }

    pub fn clip_x_collide(&self, other: &Self, mut xa: f32) -> f32 {
        if other.y1 <= self.y0 || other.y0 >= self.y1 { return xa }
        if other.z1 <= self.z0 || other.z0 >= self.z1 { return xa }

        if xa > 0.0 && other.x1 <= self.x0 {
            let max = self.x0 - other.x1;
            if max < xa { xa = max }
        }
        if xa < 0.0 && other.x0 >= self.x1 {
            let max = self.x1 - other.x0;
            if max > xa { xa = max }
        }

        return xa;
    }

    pub fn clip_y_collide(&self, other: &Self, mut ya: f32) -> f32 {
        if other.x1 <= self.x0 || other.x0 >= self.x1 { return ya }
        if other.z1 <= self.z0 || other.z0 >= self.z1 { return ya }

        if ya > 0.0 && other.y1 <= self.y0 {
            let max = self.y0 - other.y1;
            if max < ya { ya = max }
        }
        if ya < 0.0 && other.y0 >= self.y1 {
            let max = self.y1 - other.y0;
            if max > ya { ya = max }
        }

        return ya;
    }

    pub fn clip_z_collide(&self, other: &Self, mut za: f32) -> f32 {
        if other.x1 <= self.x0 || other.x0 >= self.x1 { return za }
        if other.y1 <= self.y0 || other.y0 >= self.y1 { return za }

        if za > 0.0 && other.z1 <= self.z0 {
            let max = self.z0 - other.z1;
            if max < za { za = max }
        }
        if za < 0.0 && other.z0 >= self.z1 {
            let max = self.z1 - other.z0;
            if max > za { za = max }
        }

        return za;
    }

    pub fn move_at(&mut self, xa: f32, ya: f32, za: f32) {
        self.x0 += xa;
        self.y0 += ya;
        self.z0 += za;
        self.x1 += xa;
        self.y1 += ya;
        self.z1 += za;
    }

    /// clone this aabb and move the clone
    pub fn clone_move(&self, xa: f32, ya: f32, za: f32) -> Self {
        Self::new(
            self.x0 + xa,
            self.y0 + ya,
            self.z0 + za,
            self.x1 + xa,
            self.y1 + ya,
            self.z1 + za
        )
    }

    pub fn set_position(&mut self, xa: f32, ya: f32, za: f32) {
        let size = self.get_size();

        self.x0 = xa;
        self.y0 = ya;
        self.z0 = za;
        self.x1 = self.x0 + size.x;
        self.y1 = self.y0 + size.y;
        self.z1 = self.z0 + size.z;
    }

    pub fn set(&mut self, other: &Self) {
        *self = *other;
    }

    pub fn intersects(&self, other: &Self) -> bool {
        if other.x1 <= self.x0 || other.x0 >= self.x1 { return false }
        if other.y1 <= self.y0 || other.y0 >= self.y1 { return false }
        if other.z1 <= self.z0 || other.z0 >= self.z1 { return false }

        return true;
    }

    pub fn get_size(&self) -> Vec3 {
        Vec3::new(
            self.x1 - self.x0,
            self.y1 - self.y0,
            self.z1 - self.z0
        )
    }

    pub fn get_pos(&self) -> Vec3 {
        Vec3::new(
            self.x0,
            self.y0,
            self.z0
        )
    }
}

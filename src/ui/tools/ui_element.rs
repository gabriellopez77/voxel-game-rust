use crate::math::Vec2;


pub trait UiElement {
    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;

    
    fn set_pos(&mut self, x: f32, y: f32);
    fn set_posx(&mut self, x: f32) { self.set_pos(x, self.get_pos().y) }
    fn set_posy(&mut self, y: f32) { self.set_pos(self.get_pos().x, y) }
    fn set_posv(&mut self, pos: Vec2) { self.set_pos(pos.x, pos.y);}
    
    fn set_size(&mut self, x: f32, y: f32);
    fn set_sizex(&mut self, x: f32) { self.set_size(x, self.get_size().y) }
    fn set_sizey(&mut self, y: f32) { self.set_size(self.get_size().x, y) }
    fn set_sizev(&mut self, size: Vec2) { self.set_size(size.x, size.y);}

    fn mouse_hover(&self, mouse_pos: Vec2) -> bool {
        return  mouse_pos.x > self.get_pos().x &&
                mouse_pos.x < self.get_pos().x + self.get_size().x &&
                mouse_pos.y > self.get_pos().y &&
                mouse_pos.y < self.get_pos().y + self.get_size().y;
    }

    fn get_center(&self, other: &dyn UiElement) -> Vec2 { other.get_pos() + other.get_size() / 2.0 - self.get_size() / 2.0 }
    fn get_centerx(&self, other: &dyn UiElement) -> f32 { other.get_pos().x + other.get_size().x / 2.0 - self.get_size().x / 2.0 }
    fn get_centery(&self, other: &dyn UiElement) -> f32 { other.get_pos().y + other.get_size().y / 2.0 - self.get_size().y / 2.0 }

    fn set_center(&mut self, other: &dyn UiElement) { self.set_posv(other.get_pos() + other.get_size() / 2.0 - self.get_size() / 2.0); }
    fn set_centerx(&mut self, other: &dyn UiElement) { self.set_posx(other.get_pos().x + other.get_size().x / 2.0 - self.get_size().x / 2.0); }
    fn set_centery(&mut self, other: &dyn UiElement) { self.set_posy(other.get_pos().y + other.get_size().y / 2.0 - self.get_size().y / 2.0); }


    fn get_final(&self) -> Vec2 { self.get_pos() + self.get_size() }
    fn get_finalx(&self) -> f32 { self.get_pos().x + self.get_size().x }
    fn get_finaly(&self) -> f32 { self.get_pos().y + self.get_size().y }

    fn set_final(&mut self, other: &dyn UiElement) {self.set_posv(other.get_pos() + other.get_size()); }
    fn set_finalx(&mut self, other: &dyn UiElement) { self.set_posx(other.get_pos().x + other.get_size().x) }
    fn set_finaly(&mut self, other: &dyn UiElement) { self.set_posy(other.get_pos().y + other.get_size().y) }
}
use crate::{
    math::{Color4b, Vec2, Vec2i16}, render::{GlobalRenderer, SpritesVertices, Texture, UiRenderer}, resources::TexCoords, ui::tools::UiElement
};


pub struct Slice {
    position: Vec2,
    size: Vec2,
    texture_idx: u8,

    pub color: Color4b,

    corner: u8,

    slice_tex: [TexCoords; 9],
    slice_size: [Vec2; 9],
    slice_position: [Vec2; 9],
}

impl UiElement for Slice {
    fn get_pos(&self) -> Vec2 { self.position }
    fn set_pos(&mut self, x: f32, y: f32) {
        self.position = Vec2{ x, y };

        self.update_position();
    }

    fn get_size(&self) -> Vec2 { self.size }
    fn set_size(&mut self, x: f32, y: f32) {
        self.size = Vec2{ x, y };

        self.update_size();
        self.update_position();
    }
}

impl Slice {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            texture_idx: 0,

            color: Color4b::WHITE,

            corner: 0,

            slice_tex: [TexCoords::ZERO; 9],
            slice_size: [Vec2::ZERO; 9],
            slice_position: [Vec2::ZERO; 9],
        }
    }

    pub fn draw(&self, renderer: &mut UiRenderer) {
        if self.size.x == 0.0 || self.size.y == 0.0 { return }

        if renderer.get_sprites_count() >= UiRenderer::MAX_SPRITES_COUNT - 9 { return }

        for i in 0..9 {
            let pos = self.slice_position[i];
            let size = self.slice_size[i];

            renderer.add_sprite(SpritesVertices{
                position: Vec2i16::new(pos.x as i16, pos.y as i16),
                size: Vec2i16::new(size.x as i16, size.y as i16),
                uv: self.slice_tex[i],
                color: self.color,
                texture_idx: self.texture_idx,
            });
        }
    }

    pub fn set_texture_from_coords(&mut self, coords: TexCoords, corner: u8, corner_norm: f32) {
        self.corner = corner;
        self.texture_idx = GlobalRenderer::UI_SPRITES_TEXTURE_IDX;

        let cx = corner_norm;
        let cy = corner_norm;

        let top = coords.miny;
        let bottom = coords.maxy;
        let left = coords.minx;
        let right = coords.maxx;

        self.slice_tex[0] = TexCoords::new(left, top, left + cx, top + cy);
        self.slice_tex[1] = TexCoords::new(left + cx, top, right - cx, top + cy);
        self.slice_tex[2] = TexCoords::new(right - cx, top, right, top + cy);

        self.slice_tex[3] = TexCoords::new(left, top + cy, left + cx, bottom - cy);
        self.slice_tex[4] = TexCoords::new(left + cx, top + cy, right - cx, bottom - cy);
        self.slice_tex[5] = TexCoords::new(right - cx, top + cy, right, bottom - cy);

        self.slice_tex[6] = TexCoords::new(left, bottom - cy, left + cx, bottom);
        self.slice_tex[7] = TexCoords::new( left + cx, bottom - cy, right - cx, bottom);
        self.slice_tex[8] = TexCoords::new(right - cx, bottom - cy, right, bottom);

        self.update_size();
        self.update_position();
    }

    pub fn set_texture(&mut self, tex: &Texture, name: &'static str, corner: u8) {
        self.corner = corner;

        let coords = tex.get_coords(name);
        let cx = corner as f32 / tex.get_size().x;

        self.set_texture_from_coords(coords, corner, cx);
    }

    fn update_size(&mut self) {
        let slice_size = &mut self.slice_size;
        let corner = self.corner as f32;
        let size = self.size;

        if size.x == 0.0|| size.y == 0.0 { return }

        let corner_multiplied = corner * 2.0;

        slice_size[0].x = (corner).ceil();
        slice_size[0].y = (corner).ceil();

        slice_size[1].x = (size.x - corner_multiplied).ceil();
        slice_size[1].y = (corner).ceil();

        slice_size[2].x = (corner).ceil();
        slice_size[2].y = (corner).ceil();

        slice_size[3].x = (corner).ceil();
        slice_size[3].y = (size.y - corner_multiplied).ceil();

        slice_size[4].x = (size.x - corner_multiplied).ceil();
        slice_size[4].y = (size.y - corner_multiplied).ceil();

        slice_size[5].x = (corner).ceil();
        slice_size[5].y = (size.y - corner_multiplied).ceil();

        slice_size[6].x = (corner).ceil();
        slice_size[6].y = (corner).ceil();

        slice_size[7].x = (size.x - corner_multiplied).ceil();
        slice_size[7].y = (corner).ceil();

        slice_size[8].x = (corner).ceil();
        slice_size[8].y = (corner).ceil();
    }

    fn update_position(&mut self) {
        let slice_pos = &mut self.slice_position;
        let corner = self.corner as f32;
        let pos = self.position;
        let size = self.size;


        slice_pos[0].x = (pos.x).floor();
        slice_pos[0].y = (pos.y).floor();

        slice_pos[1].x = (pos.x + corner).floor();
        slice_pos[1].y = (pos.y).floor();

        slice_pos[2].x = (pos.x + size.x - corner).floor();
        slice_pos[2].y = (pos.y).floor();

        slice_pos[3].x = (pos.x).floor();
        slice_pos[3].y = (pos.y + corner).floor();

        slice_pos[4].x = (pos.x + corner).floor();
        slice_pos[4].y = (pos.y + corner).floor();

        slice_pos[5].x = (pos.x + size.x - corner).floor();
        slice_pos[5].y = (pos.y + corner).floor();

        slice_pos[6].x = (pos.x).floor();
        slice_pos[6].y = (pos.y + size.y - corner).floor();

        slice_pos[7].x = (pos.x + corner).floor();
        slice_pos[7].y = (pos.y + size.y - corner).floor();

        slice_pos[8].x = (pos.x + size.x - corner).floor();
        slice_pos[8].y = (pos.y + size.y - corner).floor();
    }
}

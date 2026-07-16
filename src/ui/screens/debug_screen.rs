use std::collections::VecDeque;
use std::fmt::Write;

use crate::math::{self, Color3b, Color4b, KeyFrame};
use crate::render::UiRenderer;
use crate::ui::tools::{Slice, Sprite, Text, UiElement};
use crate::ui::{ScreenBase, ScreenResizeArgs, ScreenStartArgs, ScreenUpdateArgs};


pub struct DebugScreen {
    fps_text: Text,
    player_block_pos_text: Text,

    // ms graph
    min_ms_text: Text,
    avg_ms_text: Text,
    max_ms_text: Text,

    background: Sprite,
    corner: Slice,
    avg_line: Sprite,

    fps_graphic_gradient: KeyFrame<Color3b>,
    fps_graphic_lines: VecDeque<(f32, Sprite)>,


    in_world: bool,
}

impl ScreenBase for DebugScreen {
    fn start(&mut self, args: &ScreenStartArgs) {
        self.fps_text.set_font(args.resources.get_font("default"));
        self.fps_text.set_pos(10.0, 10.0);

        self.player_block_pos_text.set_font(args.resources.get_font("default"));
        self.player_block_pos_text.set_pos(10.0, 19.0);


        self.min_ms_text.set_font(args.resources.get_font("default"));
        self.avg_ms_text.set_font(args.resources.get_font("default"));
        self.max_ms_text.set_font(args.resources.get_font("default"));

        self.min_ms_text.enable_shadow();
        self.avg_ms_text.enable_shadow();
        self.max_ms_text.enable_shadow();

        self.corner.set_texture(&args.resources.ui_sprites_texture, "debug_background", 2);
        self.corner.set_size(240.0, 60.0);

        self.background.color = Color4b::new(9, 13, 22, 192);
        self.background.set_size(240.0, 60.0);

        self.avg_line.set_size(240.0, 1.0);
        self.avg_line.color = Color4b::from1(255);

        self.fps_graphic_gradient.frames = vec![
            (10.0 / 100.0, Color3b::new(0, 255, 0)),
            (40.0 / 100.0, Color3b::new(255, 255, 0)),
            (100.0 / 100.0, Color3b::new(255, 0, 0)),
        ];
    }

    fn update(&mut self, args: &mut ScreenUpdateArgs) {
        self.in_world = args.game.is_in_world();

        if self.in_world {
            self.player_block_pos_text.set_text_delayed(args.dt, 0.5, |text| {
                let block_pos = math::get_global_block(args.game.world.player.get_pos());
                write!(text, "Block Pos: {}, {}, {}", block_pos.x, block_pos.y, block_pos.z)
            });
        }

        self.fps_text.set_text_delayed(args.dt, 0.5, |text| { write!(text, "Fps: {}", (1.0 / args.dt) as i32) });


        if self.fps_graphic_lines.len() > 240 {
            self.fps_graphic_lines.pop_front();
        }

        let ms_count = args.dt * 1000.0;


        let height = (ms_count * 1.75).ceil();
        let color = self.fps_graphic_gradient.get(height / 100.0);

        let mut line = Sprite::new();
        line.color = Color4b::from3(color, 255);
        line.set_size(1.0,  height);
        line.set_pos(240.0, 10.0);

        self.fps_graphic_lines.push_back((ms_count, line));

        let background_end = self.background.get_finaly();

        let mut avg_ms = 0.0;
        let mut max_ms = f32::MIN;
        let mut min_ms = f32::MAX;

        for (ms, line) in &mut self.fps_graphic_lines {
            avg_ms += *ms;
            max_ms = max_ms.max(*ms);
            min_ms = min_ms.min(*ms);

            line.set_pos(
                line.get_pos().x - 1.0,
                background_end - line.get_size().y - 1.0
            );
        }

        avg_ms /= self.fps_graphic_lines.len() as f32;

        self.min_ms_text.set_text_delayed(args.dt, 1.0, |text| { write!(text, "{:.1} ms min", min_ms) });
        self.avg_ms_text.set_text_delayed(args.dt, 1.0, |text| { write!(text, "{:.1} ms avg", avg_ms) });
        self.max_ms_text.set_text_delayed(args.dt, 1.0, |text| { write!(text, "{:.1} ms max", max_ms) });


        self.min_ms_text.set_pos(
            self.background.get_pos().x,
            self.background.get_pos().y - self.min_ms_text.get_size().y - 2.0
        );

        self.avg_ms_text.set_pos(
            self.avg_ms_text.get_centerx(&self.background),
            self.background.get_pos().y - self.avg_ms_text.get_size().y - 2.0
        );

        self.max_ms_text.set_pos(
            self.background.get_finalx() - self.max_ms_text.get_size().x,
            self.background.get_pos().y - self.max_ms_text.get_size().y - 2.0
        );
    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        if self.in_world {
            self.player_block_pos_text.draw(renderer);
        }

        self.fps_text.draw(renderer);


        self.background.draw(renderer);

        for (_, line) in &mut self.fps_graphic_lines {
            line.draw(renderer);
        }

        self.corner.draw(renderer);
        self.avg_line.draw(renderer);

        self.min_ms_text.draw(renderer);
        self.avg_ms_text.draw(renderer);
        self.max_ms_text.draw(renderer);
    }

    fn resize(&mut self, args: &ScreenResizeArgs) {
        self.background.set_pos(0.0, args.screen_size.y - self.background.get_size().y);
        self.corner.set_posv(self.background.get_pos());
        self.avg_line.set_pos(0.0, self.avg_line.get_centery(&self.background));
    }
}

impl DebugScreen {
    pub fn new() -> Self {
        Self {
            fps_text: Text::new(),
            player_block_pos_text: Text::new(),


            min_ms_text: Text::new(),
            avg_ms_text: Text::new(),
            max_ms_text: Text::new(),

            background: Sprite::new(),
            corner: Slice::new(),
            avg_line: Sprite::new(),

            fps_graphic_gradient: KeyFrame::new(|factor, current, next| {
                let r = current.r as f32 + (next.r as f32 - current.r as f32) * factor;
                let g = current.g as f32 + (next.g as f32 - current.g as f32) * factor;
                let b = current.b as f32 + (next.b as f32 - current.b as f32) * factor;

                return Color3b::new(r as u8, g as u8, b as u8);
            }),

            fps_graphic_lines: VecDeque::with_capacity(240),

            in_world: false,
        }
    }
}

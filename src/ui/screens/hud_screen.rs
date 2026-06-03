use std::array;
use crate::{math::Vec2, resources::ResourceManager, ui::{ScreenBase, tools::{Sprite, UiElement}}};
use crate::render::UiRenderer;
use crate::ui::ScreenUpdateArgs;
use crate::ui::tools::elements_grid::Alignment;
use crate::ui::tools::{ElementsGrid, inventory::ItemSlot, Text};
use crate::world::player::entitiy_inventory::PLAYER_HOTBAR_SLOTS_COUNT;
use crate::world::player::EntityInventory;

pub struct HudScreen {
    crosshair: Sprite,

    hotbar_selected_slot: Sprite,
    hotbar_slots: [ItemSlot; PLAYER_HOTBAR_SLOTS_COUNT],
    hotbar_grid: ElementsGrid,

    fps_text: Text,

    delay: f32,
}

impl ScreenBase for HudScreen {
    fn start(&mut self, resources: &ResourceManager, args: &ScreenUpdateArgs) {
        self.crosshair.set_texture_from_atlas(resources,"crosshair");
        self.crosshair.set_size(16.0, 16.0);
        
        self.hotbar_selected_slot.set_texture_from_atlas(resources, "hotbar_selected_slot");
        self.hotbar_selected_slot.set_size(24.0, 24.0);

        for i in 0..self.hotbar_slots.len() {
            let slot = &mut self.hotbar_slots[i];

            slot.start(i as i32, resources.get_texture("ui").unwrap().get_coords("hotbar_slot"), resources.get_font("default").unwrap());
            slot.set_size(20.0, 20.0);

            self.hotbar_grid.add(slot);
        }

        self.fps_text.set_font(resources.get_font("default").unwrap());
        self.fps_text.set_pos(10.0, 10.0);
        self.hotbar_grid.update();
    }

    fn update(&mut self, dt: f32, args: &ScreenUpdateArgs) {
        // update selected hotbar slot position
        let selected_hotbar_index = args.game.player.get_selected_hotbar_index();
        self.hotbar_selected_slot.set_center(&self.hotbar_slots[selected_hotbar_index as usize]);

        // update hotbar item slot
        for slot in &mut self.hotbar_slots {
            slot.update(&args.game.player);
        }

        self.delay += dt;

        if self.delay > 1.0 {
            self.fps_text.set_text_i32((1.0 / dt) as i32);
            self.delay = 0.0;
        }

        self.hotbar_grid.update();
    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        self.crosshair.draw(&mut renderer.sprites);

        self.hotbar_selected_slot.draw(&mut renderer.sprites);
        for slot in &mut self.hotbar_slots {
            slot.draw(renderer);
        }


        self.fps_text.draw(&mut renderer.text)
    }

    fn resize(&mut self, args: &ScreenUpdateArgs) {
        self.crosshair.set_posv(args.screen_center - self.crosshair.get_size() / 2.0);

        let hotbar_grid_size = self.hotbar_grid.get_size();
        self.hotbar_grid.set_pos(
            args.screen_center.x - hotbar_grid_size.x / 2.0,
            args.screen_size.y - hotbar_grid_size.y - 6.0
        );
    }
}

impl HudScreen {
    pub fn new() -> Self {
        Self {
            crosshair: Sprite::new(),

            hotbar_selected_slot: Sprite::new(),
            hotbar_slots: array::from_fn(|_| ItemSlot::new()),
            hotbar_grid: ElementsGrid::new(PLAYER_HOTBAR_SLOTS_COUNT, Alignment::Horizontal, 9, 2.0),

            fps_text: Text::new(),

            delay: 0.0,
        }
    }
}

use std::array;
use crate::{math::Vec2, resources::ResourceManager, ui::{ScreenBase, tools::{Sprite, UiElement}}};
use crate::render::UiRenderer;
use crate::ui::screen_base::ScreenInfo;
use crate::ui::tools::elements_grid::Alignment;
use crate::ui::tools::{ElementsGrid, inventory::ItemSlot};
use crate::world::player::entitiy_inventory::PLAYER_HOTBAR_SLOTS_COUNT;


pub struct HudScreen {
    crosshair: Sprite,

    hotbar_slots: [ItemSlot; PLAYER_HOTBAR_SLOTS_COUNT],
    hotbar_grid: ElementsGrid,
}

impl ScreenBase for HudScreen {
    fn start(&mut self, resource_manager: &ResourceManager, args: &ScreenInfo) {
        self.crosshair.set_texture(resource_manager.get_texture("ui").unwrap().get_coords("crosshair"));
        self.crosshair.set_size(16.0, 16.0);

        for i in 0..self.hotbar_slots.len() {
            let slot = &mut self.hotbar_slots[i];

            slot.start(i as i32, resource_manager.get_texture("ui").unwrap().get_coords("hotbar_slot"));
            slot.set_size(20.0, 20.0);


            self.hotbar_grid.add(slot);
        }
    }

    fn update(&mut self, dt: f32, args: &ScreenInfo) {
        self.hotbar_grid.update();
    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        self.crosshair.draw(&mut renderer.sprites);

        for slot in &mut self.hotbar_slots {
            slot.draw(renderer);
        }
    }

    fn resize(&mut self, args: &ScreenInfo) {
        self.crosshair.set_posv(args.screen_center - self.crosshair.get_size());

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

            hotbar_slots: array::from_fn(|_| ItemSlot::new()),
            hotbar_grid: ElementsGrid::new(PLAYER_HOTBAR_SLOTS_COUNT, Alignment::Horizontal, 9, 2.0),
        }
    }
}

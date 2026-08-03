use std::array;

use crate::{ui::{ScreenBase, ScreenResizeArgs, ScreenStartArgs, tools::{Slice, Sprite, UiElement, inventory::SlotData}}, world::player::{PlayerInventory, player_inventory::SlotType}};
use crate::render::UiRenderer;
use crate::ui::ScreenUpdateArgs;
use crate::ui::tools::elements_grid::Alignment;
use crate::ui::tools::{ElementsGrid, Text};


pub struct HudScreen {
    crosshair: Sprite,

    item_name_text: Text,
    hotbar_selected_slot: Sprite,
    hotbar_slots: [SlotData; PlayerInventory::HOTBAR_SLOTS_COUNT],
    hotbar_slots_background: [Slice; PlayerInventory::HOTBAR_SLOTS_COUNT],
    hotbar_grid: ElementsGrid,
}

impl ScreenBase for HudScreen {
    fn start(&mut self, args: &ScreenStartArgs) {
        self.crosshair.set_texture(&args.resources.ui_sprites_texture, "crosshair");
        self.crosshair.set_size(16.0, 16.0);

        self.item_name_text.set_font(args.resources.get_font("default"));
        self.item_name_text.enable_shadow();

        self.hotbar_selected_slot.set_texture(&args.resources.ui_sprites_texture, "hotbar_selected_slot");
        self.hotbar_selected_slot.set_size(24.0, 24.0);

        for i in 0..PlayerInventory::HOTBAR_SLOTS_COUNT {
            let slot = &mut self.hotbar_slots[i];

            slot.start(SlotType::Inventory, i as i32, args.resources.get_font("default"));
            slot.set_size(20.0, 20.0);
        }

        for i in 0..PlayerInventory::HOTBAR_SLOTS_COUNT {
            let slot = &mut self.hotbar_slots_background[i];

            slot.set_texture(&args.resources.ui_sprites_texture, "hotbar_slot", 2);
            slot.set_size(20.0, 20.0);

            self.hotbar_grid.add(slot);
        }


        self.hotbar_grid.update();
    }

    fn update(&mut self, args: &mut ScreenUpdateArgs) {
        let player_inventory = &args.game.world.player.inventory;

        // update selected hotbar slot position
        let selected_hotbar_index = player_inventory.get_selected_hotbar_index();
        self.hotbar_selected_slot.set_center(&self.hotbar_slots[selected_hotbar_index as usize]);

        if let Some(item) = player_inventory.get_hand_slot().get_item() {
            self.item_name_text.set_text(item.name);

            self.item_name_text.set_pos(
                self.hotbar_grid.get_pos().x,
                self.hotbar_grid.get_pos().y - self.item_name_text.get_size().y - 4.0
            );

        }
        else {
            self.item_name_text.set_text("");
        }

        // update hotbar item slot
        for slot in &mut self.hotbar_slots {
            slot.update(player_inventory);
        }
    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        self.crosshair.draw(renderer);

        self.item_name_text.draw(renderer);

        self.hotbar_selected_slot.draw(renderer);
        for slot in &mut self.hotbar_slots_background {
            slot.draw(renderer);
        }

        for slot in &mut self.hotbar_slots {
            slot.draw(renderer);
        }
    }

    fn resize(&mut self, args: &ScreenResizeArgs) {
        self.crosshair.set_posv(args.screen_center - self.crosshair.get_size() / 2.0);

        let hotbar_grid_size = self.hotbar_grid.get_size();
        self.hotbar_grid.set_pos(
            args.screen_center.x - hotbar_grid_size.x / 2.0,
            args.screen_size.y - hotbar_grid_size.y - 6.0
        );
        self.hotbar_grid.update();


        for i in 0..PlayerInventory::HOTBAR_SLOTS_COUNT {
            self.hotbar_slots[i].set_center(&self.hotbar_slots_background[i]);
        }
    }
}

impl HudScreen {
    pub fn new() -> Self {
        Self {
            crosshair: Sprite::new(),

            item_name_text: Text::new(),
            hotbar_selected_slot: Sprite::new(),
            hotbar_slots: array::from_fn(|_| SlotData::new()),
            hotbar_slots_background: array::from_fn(|_| Slice::new()),
            hotbar_grid: ElementsGrid::new(Alignment::Horizontal, 9, 2.0),
        }
    }
}

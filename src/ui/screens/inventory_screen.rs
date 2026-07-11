use std::array;

use crate::render::UiRenderer;
use crate::ui::tools::elements_grid::Alignment;
use crate::ui::tools::inventory::{ItemSlot,};
use crate::ui::tools::{ElementsGrid, Slice, Sprite, Text, UiElement};
use crate::ui::{ScreenBase, ScreenResizeArgs, ScreenStartArgs, ScreenUpdateArgs};
use crate::world::player::player_inventory::SlotType;
use crate::world::player::PlayerInventory;


pub struct InventoryScreen {
    creative_background: Slice,

    creative_text: Text,
    creative_slots: Vec<ItemSlot>,
    creative_slots_grid: ElementsGrid,


    background: Slice,

    hotbar_text: Text,
    hotbar_slots_grid: ElementsGrid,
    hotbar_slots: [ItemSlot; PlayerInventory::HOTBAR_SLOTS_COUNT],
    hotbar_selected_slot: Sprite,

    inventory_text: Text,
    inventory_slots: [ItemSlot; PlayerInventory::INVENTORY_SLOTS_COUNT],
    inventory_slots_grid: ElementsGrid,
}

impl ScreenBase for InventoryScreen {
    fn start(&mut self, args: &ScreenStartArgs) {
        self.creative_background.set_texture(&args.resources.ui_sprites_texture, "inventory_background", 2);
        self.creative_background.set_size(220.0, 160.0);

        self.creative_text.set_font(args.resources.get_font("default"));
        self.creative_text.set_text("Creative Inventory");

        self.creative_slots.reserve(args.game.world.player.inventory.get_creative_items_count());
        for i in 0..self.creative_slots.capacity() {
            let mut slot = ItemSlot::new();

            slot.start(SlotType::Creative, i,
                &args.resources.ui_sprites_texture, "inventory_creative_item_slot",
                args.resources.get_font("default")
            );
            slot.set_size(20.0, 20.0);

            self.creative_slots.push(slot);
        }

        for slot in &mut self.creative_slots {
            self.creative_slots_grid.add(slot);
        }



        self.background.set_texture(&args.resources.ui_sprites_texture, "inventory_background", 2);
        self.background.set_size(220.0, 125.0);

        self.hotbar_text.set_font(args.resources.get_font("default"));
        self.hotbar_text.set_text("Hotbar");

        self.inventory_text.set_font(args.resources.get_font("default"));
        self.inventory_text.set_text("Inventory");

        for i in 0..PlayerInventory::HOTBAR_SLOTS_COUNT {
            let slot = &mut self.hotbar_slots[i];

            slot.start(SlotType::Inventory, i,
                &args.resources.ui_sprites_texture, "inventory_item_slot",
                args.resources.get_font("default")
            );
            slot.set_size(20.0, 20.0);

            self.hotbar_slots_grid.add(slot);
        }

        self.hotbar_selected_slot.set_texture(&args.resources.ui_sprites_texture, "hotbar_selected_slot");
        self.hotbar_selected_slot.set_size(24.0, 24.0);

        for i in 0..PlayerInventory::INVENTORY_SLOTS_COUNT {
            let slot = &mut self.inventory_slots[i];

            slot.start(SlotType::Inventory, i + PlayerInventory::HOTBAR_SLOTS_COUNT,
                &args.resources.ui_sprites_texture, "inventory_item_slot",
                args.resources.get_font("default")
            );
            slot.set_size(20.0, 20.0);

            self.inventory_slots_grid.add(slot);
        }

        self.inventory_slots_grid.update();
        self.hotbar_slots_grid.update();
        self.creative_slots_grid.update();
    }

    fn update(&mut self, args: &mut ScreenUpdateArgs) {
        self.hotbar_slots_grid.update();
        self.inventory_slots_grid.update();
        self.creative_slots_grid.update();

        let player_inventory = &mut args.game.world.player.inventory;

        for slot in &mut self.creative_slots {
            slot.update(player_inventory, args.mouse_pos, args.inputs, args.ui_common);
        }

        for slot in &mut self.hotbar_slots {
            slot.update(player_inventory, args.mouse_pos, args.inputs, args.ui_common);
        }

        for slot in &mut self.inventory_slots {
            slot.update(player_inventory, args.mouse_pos, args.inputs, args.ui_common);
        }

        // update selected hotbar slot position
        let selected_hotbar_index = player_inventory.get_selected_hotbar_index();
        self.hotbar_selected_slot.set_center(&self.hotbar_slots[selected_hotbar_index as usize]);
    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        self.creative_background.draw(renderer);

        self.creative_text.draw(renderer);
        for slot in &mut self.creative_slots {
            slot.draw(renderer);
        }


        self.background.draw(renderer);

        self.hotbar_text.draw(renderer);
        for slot in &mut self.hotbar_slots {
            slot.draw(renderer);
        }

        self.hotbar_selected_slot.draw(renderer);

        self.inventory_text.draw(renderer);
        for slot in &mut self.inventory_slots {
            slot.draw(renderer);
        }
    }

    fn resize(&mut self, args: &ScreenResizeArgs) {
        self.background.set_pos(
            args.screen_center.x - self.background.get_size().x / 2.0,
            args.screen_size.y - self.background.get_size().y - 20.0
        );

        self.hotbar_slots_grid.set_pos(
            self.hotbar_slots_grid.get_centerx(&self.background),
            self.background.get_finaly() - self.hotbar_slots_grid.get_size().y - 6.0
        );

        self.hotbar_text.set_pos(
            self.hotbar_slots_grid.get_pos().x,
            self.hotbar_slots_grid.get_pos().y - self.hotbar_text.get_size().y - 3.0
        );

        self.inventory_slots_grid.set_pos(
            self.inventory_slots_grid.get_centerx(&self.background),
            self.hotbar_text.get_pos().y - self.inventory_slots_grid.get_size().y - 6.0
        );

        self.inventory_text.set_pos(
            self.inventory_slots_grid.get_pos().x,
            self.inventory_slots_grid.get_pos().y - self.inventory_text.get_size().y - 3.0
        );



        self.creative_background.set_pos(
            self.background.get_pos().x,
            self.background.get_pos().y - self.creative_background.get_size().y - 6.0
        );

        self.creative_text.set_pos(
            self.creative_slots_grid.get_centerx(&self.creative_background),
            self.creative_background.get_pos().y + 6.0
        );

        self.creative_slots_grid.set_pos(
            self.creative_text.get_pos().x,
            self.creative_text.get_finaly() + 3.0
        );
    }
}

impl InventoryScreen {
    pub fn new() -> Self {
        Self {
            creative_background: Slice::new(),

            creative_slots: Vec::new(),
            creative_slots_grid: ElementsGrid::new(Alignment::Horizontal, PlayerInventory::HOTBAR_SLOTS_COUNT as i32, 3.0),
            creative_text: Text::new(),

            background: Slice::new(),

            hotbar_text: Text::new(),
            hotbar_slots_grid: ElementsGrid::new(Alignment::Horizontal, PlayerInventory::HOTBAR_SLOTS_COUNT as i32, 3.0),
            hotbar_slots: array::from_fn(|_| ItemSlot::new()),
            hotbar_selected_slot: Sprite::new(),

            inventory_text: Text::new(),
            inventory_slots_grid: ElementsGrid::new(Alignment::Horizontal, PlayerInventory::HOTBAR_SLOTS_COUNT as i32, 3.0),
            inventory_slots: array::from_fn(|_| ItemSlot::new()),
        }
    }
}

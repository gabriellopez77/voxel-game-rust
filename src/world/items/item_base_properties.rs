use crate::resources::TextureCoords;


pub struct ItemBaseProperties {
    pub id: u16,
    pub name: &'static str,
    pub internal_name: &'static str,
    pub icon: TextureCoords,
    
    pub block_index: Option<u32>,
    pub item_index: Option<u32>,
}

impl ItemBaseProperties {
    pub fn new(internal_name: &'static str, name: &'static str, icon: TextureCoords, block_index: Option<u32>, item_index: Option<u32>) -> Self {
        static mut CURRENT_ID: u16 = 0;

        let new_id;

        unsafe {
            new_id = CURRENT_ID;
            CURRENT_ID += 1;
        }

        Self {
            id: new_id,
            name,
            internal_name,
            icon,
            block_index,
            item_index,
        }
    }
}
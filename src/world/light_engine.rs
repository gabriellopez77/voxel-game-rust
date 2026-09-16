use std::{collections::{HashMap, VecDeque}, sync::{Arc, Mutex, RwLock}};

use crate::{
    math::Vec3i,
    game::Directions,
    world::{
        chunk::{
            Chunk,
            NeighborsChunksData,
            chunk_data::{
                ChunkData,
                ChunkDataFlags,
                ChunkDataReadBehavior,
                ChunkDataReadGuard,
            }
        },
        blocks::BlockProperties
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Sky,
    Block,
    Both,
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LightSectionLevel {
    Zero,
    One,
    Two
}

pub const MAX_LEVEL: u8 = 15;
pub const MIN_LEVEL: u8 = 0;
pub const BLOCK_MASK: u8 = 0b11110000;
pub const SKY_MASK: u8 = 0b00001111;

pub const MAX_SKY_LEVEL: u8 = SKY_MASK;
pub const MAX_BLOCK_LEVEL: u8 = BLOCK_MASK;
pub const MAX_BOTH_LEVEL: u8 = SKY_MASK | BLOCK_MASK;

pub const fn get_level(value: u8, light_type: LightType) -> u8 {
    if matches!(light_type, LightType::Sky) {
        return value & SKY_MASK;
    }
    else if matches!(light_type, LightType::Block) {
        return value >> 4;
    }

    return value;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum UpdateType {
    Add,
    Remove
}

struct LightQueueData{
    chunk_data: Arc<ChunkData>,
    chunk_block: Vec3i,
    light_level: u8,
    old_value: u8,
}

impl LightQueueData {
    pub fn new(
        chunk_data: Arc<ChunkData>,
        chunk_block: Vec3i,
        light_level: u8,
        old_value: u8
    ) -> Self {
        Self {
            chunk_data,
            chunk_block,
            light_level,
            old_value,
        }
    }
}

static QUEUE_DATA_POOL: Mutex<Vec<VecDeque<LightQueueData>>> = Mutex::new(Vec::new());

fn get_queue() -> VecDeque<LightQueueData> {
    if let Some(mut queue) = QUEUE_DATA_POOL.lock().unwrap().pop() {
        queue.clear();

        return queue;
    }

    return VecDeque::new();
}

fn restore_queue(queue: VecDeque<LightQueueData>) { QUEUE_DATA_POOL.lock().unwrap().push(queue); }

pub fn update_light(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>,
    chunk_data: Arc<ChunkData>,
    chunk_block: Vec3i,
    old_block: &BlockProperties,
    new_block: &BlockProperties
) {
    // update the light is redundant
    if old_block.light_filter == 0 && old_block.light_emission == 0 &&
        new_block.light_filter == 0 && new_block.light_emission == 0 {
        return
    }

    // remove around light sources
    if old_block.light_filter > MIN_LEVEL || old_block.light_emission > MIN_LEVEL ||
        new_block.light_filter > MIN_LEVEL {
        remove_block_light_source(chunks_map.clone(), chunk_data.clone(), chunk_block);
    }

    if new_block.light_emission > MIN_LEVEL {
        add_block_light_source(chunks_map.clone(), chunk_data.clone(), chunk_block, new_block);
    }

    let mut add_sky_queue = get_queue();
    let mut remove_queue = get_queue();

    let mut new_value = MIN_LEVEL;

    if chunk_block.y == Chunk::CHUNK_SIZE_MINUS_ONE.y && new_block.light_filter < MAX_LEVEL {
        add_sky_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, MAX_LEVEL, 0));

        new_value = MAX_LEVEL - new_block.light_filter;
    }

    // uses remove light logic to update skylight around
    remove_queue.push_back(LightQueueData::new(
        chunk_data.clone(),
        chunk_block,
        MIN_LEVEL,
        chunk_data.get_light(chunk_block, LightType::Sky)
    ));

    chunk_data.set_light(chunk_block, new_value, LightType::Sky);

    let mut neighbors = NeighborsChunksData::new_from_map(chunks_map.clone(), chunk_data.get_position(), false);

    process_remove_queue(Some(&chunks_map), LightType::Sky, &mut add_sky_queue, &mut remove_queue, &mut neighbors);
    process_add_queue(Some(&chunks_map), LightType::Sky, &mut add_sky_queue, &mut neighbors);

    restore_queue(add_sky_queue);
    restore_queue(remove_queue);
}

pub fn update_light_in_border_neighbors(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>,
    chunk_data: Arc<ChunkData>,
    neighbors_chunks_data: NeighborsChunksData
) {
    let update_light_in_border = |chunk_data: Arc<ChunkData>, light_type: LightType, face: Directions| {
        let mut add_queue = get_queue();
        let chunk_pos: Vec3i;

        let mut add_light_border_in_queue = |data: &ChunkDataReadGuard, chunk_block: Vec3i| {
            //let sub_chunk = (chunk_block.y as f32 / Chunk::SUB_CHUNK_SIZE.y as f32).floor() as usize;
            //if data.light_sections[sub_chunk] == LightSectionLevel::Two {
            //    return;
            //}

            let light_value = data.get_light(chunk_block, light_type);

            if light_value == MIN_LEVEL { return }

            add_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, light_value, MIN_LEVEL));
        };



        let data = chunk_data.read_guard();
        chunk_pos = data.get_position();

        if light_type == LightType::Block && !data.flag_contains(ChunkDataFlags::CONTAINS_EMISSIVE_BLOCKS_FLAG) {
            restore_queue(add_queue);

            return;
        }

        if face == Directions::Nothing {
            for y in 0..Chunk::CHUNK_SIZE.y {
                for x in 0..Chunk::CHUNK_SIZE.x {
                    add_light_border_in_queue(&data, Vec3i::new(x, y, 0));
                    add_light_border_in_queue(&data, Vec3i::new(x, y, Chunk::CHUNK_SIZE_MINUS_ONE.z));
                }

                for z in 0..Chunk::CHUNK_SIZE.z {
                    add_light_border_in_queue(&data, Vec3i::new(0, y, z));
                    add_light_border_in_queue(&data, Vec3i::new(Chunk::CHUNK_SIZE_MINUS_ONE.x, y, z));
                }
            }
        }
        else {
            for i in 0..Chunk::CHUNK_SIZE.x {
                for y in 0..Chunk::CHUNK_SIZE.y {
                    if face == Directions::North {
                        add_light_border_in_queue(&data, Vec3i::new(i, y, 0));
                    }
                    else if face == Directions::South {
                        add_light_border_in_queue(&data, Vec3i::new(i, y, Chunk::CHUNK_SIZE_MINUS_ONE.z));
                    }
                    else if face == Directions::West {
                        add_light_border_in_queue(&data, Vec3i::new(0, y, i));
                    }
                    else if face == Directions::East {
                        add_light_border_in_queue(&data, Vec3i::new(Chunk::CHUNK_SIZE_MINUS_ONE.x, y, i));
                    }
                }
            }
        }

        std::mem::drop(data);

        let mut neighbors_data = NeighborsChunksData::new_from_map(chunks_map.clone(), chunk_pos, false);

        process_add_queue(Some(&chunks_map), light_type, &mut add_queue, &mut neighbors_data);
        restore_queue(add_queue);
    };

    update_light_in_border(chunk_data.clone(), LightType::Block, Directions::Nothing);
    update_light_in_border(chunk_data.clone(), LightType::Sky, Directions::Nothing);

    if let Some(north) = neighbors_chunks_data.north {
        north.turn_on_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG);
        update_light_in_border(north.clone(), LightType::Block, Directions::South);
        update_light_in_border(north.clone(), LightType::Sky, Directions::South);
        north.turn_off_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG)
    }
    if let Some(south) = neighbors_chunks_data.south {
        south.turn_on_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG);
        update_light_in_border(south.clone(), LightType::Block, Directions::North);
        update_light_in_border(south.clone(), LightType::Sky, Directions::North);
        south.turn_off_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG)
    }
    if let Some(west) = neighbors_chunks_data.west {
        west.turn_on_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG);
        update_light_in_border(west.clone(), LightType::Block, Directions::East);
        update_light_in_border(west.clone(), LightType::Sky, Directions::East);
        west.turn_off_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG)
    }
    if let Some(east) = neighbors_chunks_data.east {
        east.turn_on_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG);
        update_light_in_border(east.clone(), LightType::Block, Directions::West);
        update_light_in_border(east.clone(), LightType::Sky, Directions::West);
        east.turn_off_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG)
    }

    chunk_data.turn_off_flag(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG);
}

pub fn compute_light_value(chunk_data: Arc<ChunkData>) {
    let mut add_sky_queue = get_queue();
    let mut add_block_queue = get_queue();


    let mut data = chunk_data.write_guard();
    //compute_sections(&mut data);

    for x in 0..Chunk::CHUNK_SIZE.x {
    for z in 0..Chunk::CHUNK_SIZE.z {
        let mut current_level = MAX_LEVEL;

        for y in (0..Chunk::CHUNK_SIZE.y).rev() {
            //let sub_chunk = (y as f32 / Chunk::SUB_CHUNK_SIZE.y as f32).floor() as usize;
            //if data.light_sections[sub_chunk] == LightSectionLevel::Two {
            //    continue;
            //}

            let chunk_block = Vec3i::new(x, y, z);
            let block = data.get_block_properties(chunk_block);

            if block.light_emission > MIN_LEVEL {
                data.turn_on_flag(ChunkDataFlags::CONTAINS_EMISSIVE_BLOCKS_FLAG);

                data.set_light(chunk_block, block.light_emission, LightType::Block);
                add_block_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, block.light_emission, MIN_LEVEL));
            }

            current_level = current_level.wrapping_sub(block.light_filter);

            // a underflow occurred, then, current_level does have a value > MAX_LEVEL
            if current_level > MAX_LEVEL {
                current_level = MIN_LEVEL;
            }

            if current_level > MIN_LEVEL {
                if block.light_filter < MAX_LEVEL {
                    add_sky_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, current_level, MIN_LEVEL));
                }

                data.set_light(chunk_block, current_level, LightType::Sky);
            }
        }
    }
    }

    std::mem::drop(data);

    process_add_queue(None, LightType::Sky, &mut add_sky_queue, &mut NeighborsChunksData::EMPTY);
    process_add_queue(None, LightType::Block, &mut add_block_queue, &mut NeighborsChunksData::EMPTY);
    restore_queue(add_sky_queue);
    restore_queue(add_block_queue);
}

fn compute_sections(chunk_data: &mut ChunkData) {
    // checks if sub chunks contains only air blocks
    //for sub_chunk in 0..Chunk::SUB_CHUNK_COUNT {
    //    for i in 0..Chunk::SUB_CHUNK_DATA_SIZE {
    //        // block != air founded, then mark that light section to 0
    //        if chunk_data.blocks_id[sub_chunk * Chunk::SUB_CHUNK_DATA_SIZE + i] != 0 {
    //            chunk_data.light_sections[sub_chunk] = LightSectionLevel::Zero;

    //            if sub_chunk > 0 && chunk_data.light_sections[sub_chunk - 1] == LightSectionLevel::Two {
    //                chunk_data.light_sections[sub_chunk - 1] = LightSectionLevel::One;
    //            }
    //            else if sub_chunk < Chunk::SUB_CHUNK_COUNT - 1 && chunk_data.light_sections[sub_chunk + 1] == LightSectionLevel::Two {
    //                chunk_data.light_sections[sub_chunk + 1] = LightSectionLevel::One;
    //            }

    //            break;
    //        }
    //    }
    //}

    //for sub_chunk in (0..Chunk::SUB_CHUNK_COUNT).rev() {
    //    if chunk_data.light_sections[sub_chunk] != LightSectionLevel::Two {
    //        break;
    //    }

    //    for i in 0..Chunk::SUB_CHUNK_DATA_SIZE {
    //        chunk_data.light_levels[sub_chunk * Chunk::SUB_CHUNK_DATA_SIZE + i] = MAX_LEVEL;
    //    }
    //}
}

fn add_block_light_source(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>,
    chunk_data: Arc<ChunkData>,
    chunk_block: Vec3i,
    block: &BlockProperties
) {
    let new_value = chunk_data.get_light(chunk_block, LightType::Block).max(block.light_emission);
    chunk_data.set_light(chunk_block, new_value, LightType::Block);

    let mut add_queue = get_queue();

    add_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, new_value, MIN_LEVEL));

    let mut neighbors = NeighborsChunksData::new_from_map(chunks_map.clone(), chunk_data.get_position(), false);

    process_add_queue(Some(&chunks_map), LightType::Block, &mut add_queue, &mut neighbors);
    restore_queue(add_queue);
}

fn remove_block_light_source(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>,
    chunk_data: Arc<ChunkData>,
    chunk_block: Vec3i
) {
    let mut add_queue = get_queue();
    let mut remove_queue = get_queue();

    let current_value = chunk_data.get_light(chunk_block, LightType::Block);
    remove_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, MIN_LEVEL, current_value));

    chunk_data.set_light(chunk_block, MIN_LEVEL, LightType::Block);

    let mut neighbors = NeighborsChunksData::new_from_map(chunks_map.clone(), chunk_data.get_position(), false);

    process_remove_queue(Some(&chunks_map), LightType::Block, &mut add_queue, &mut remove_queue, &mut neighbors);

    // add block light if necessary
    process_add_queue(Some(&chunks_map), LightType::Block, &mut add_queue, &mut neighbors);
    restore_queue(add_queue);
    restore_queue(remove_queue);
}

fn process_add_queue(
    chunks_map: Option<&Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>>,
    light_type: LightType,
    add_queue: &mut VecDeque<LightQueueData>,
    neighbor_chunks: &mut NeighborsChunksData,
) {
    while let Some(data) = add_queue.pop_front() {
        process_light_logic(
            chunks_map,
            &data,
            light_type,
            UpdateType::Add,
            add_queue,
            &mut None,
            neighbor_chunks,
        );
    }
}

fn process_remove_queue(
    chunks_map: Option<&Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>>,
    light_type: LightType,
    add_queue: &mut VecDeque<LightQueueData>,
    remove_queue: &mut VecDeque<LightQueueData>,
    neighbor_chunks: &mut NeighborsChunksData,
) {
    while let Some(data) = remove_queue.pop_front() {
        process_light_logic(
            chunks_map,
            &data,
            light_type,
            UpdateType::Remove,
            add_queue,
            &mut Some(remove_queue),
            neighbor_chunks,
        );
    }
}

fn process_light_logic(
    chunks_map: Option<&Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>>,
    light_data: &LightQueueData,
    light_type: LightType,
    update_type: UpdateType,
    add_queue: &mut VecDeque<LightQueueData>,
    remove_queue: &mut Option<&mut VecDeque<LightQueueData>>,
    neighbor_chunks: &mut NeighborsChunksData,
) {
    let new_value = (light_data.light_level as i8 - 1).max(0) as u8;
    let old_value = light_data.old_value;

    if let Some(chunks_map) = chunks_map {
        neighbor_chunks.change_from_map(chunks_map.clone(), light_data.chunk_data.get_position(), false);
    }

    let x = light_data.chunk_block.x;
    let y = light_data.chunk_block.y;
    let z = light_data.chunk_block.z;

    if y < Chunk::CHUNK_SIZE_MINUS_ONE.y {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::Up,
            Vec3i::new(x, y + 1, z), light_data.chunk_data.clone(), add_queue, remove_queue);
    }

    if y > 0 {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::Down,
            Vec3i::new(x, y - 1, z), light_data.chunk_data.clone(), add_queue, remove_queue);
    }

    if z < Chunk::CHUNK_SIZE_MINUS_ONE.z {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::South,
            Vec3i::new(x, y, z + 1), light_data.chunk_data.clone(), add_queue, remove_queue);
    }
    else if let Some(ref south) = neighbor_chunks.south {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::South,
            Vec3i::new(x, y, 0), south.clone(), add_queue, remove_queue);
    }

    if z > 0 {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::North,
            Vec3i::new(x, y, z - 1), light_data.chunk_data.clone(), add_queue, remove_queue);
    }
    else if let Some(ref north) = neighbor_chunks.north {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::North,
            Vec3i::new(x, y, Chunk::CHUNK_SIZE_MINUS_ONE.z), north.clone(), add_queue, remove_queue);
    }

    if x > 0 {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::West,
            Vec3i::new(x - 1, y, z), light_data.chunk_data.clone(), add_queue, remove_queue);
    }
    else if let Some(ref west) = neighbor_chunks.west {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::West,
            Vec3i::new(Chunk::CHUNK_SIZE_MINUS_ONE.x, y, z), west.clone(), add_queue, remove_queue);
    }

    if x < Chunk::CHUNK_SIZE_MINUS_ONE.x {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::East,
            Vec3i::new(x + 1, y, z), light_data.chunk_data.clone(), add_queue, remove_queue);
    }
    else if let Some(ref east) = neighbor_chunks.east {
        update_light_logic(old_value, new_value, light_type, update_type, Directions::East,
            Vec3i::new(0, y, z), east.clone(), add_queue, remove_queue);
    }
}

fn update_light_logic(
    old_value: u8,
    mut new_value: u8,
    light_type: LightType,
    update_type: UpdateType,
    dir: Directions,
    mut chunk_block: Vec3i,
    chunk_data: Arc<ChunkData>,
    add_queue: &mut VecDeque<LightQueueData>,
    remove_queue: &mut Option<&mut VecDeque<LightQueueData>>
) {
    let mut ch_data = chunk_data.write_guard();

    let mut current_value = ch_data.get_light(chunk_block, light_type);
    let mut current_block = ch_data.get_block_properties(chunk_block);

    if update_type == UpdateType::Add {
        if current_block.light_filter == MAX_LEVEL { return }

        // if up block light level is 15 and type is sky sets current level to 15
        if dir == Directions::Up && light_type == LightType::Sky && current_value == MAX_LEVEL {
            chunk_block.y -= 1;
            current_block = ch_data.get_block_properties(chunk_block);
            current_value = ch_data.get_light(chunk_block, light_type);
            new_value = MAX_LEVEL - current_block.light_filter;
        }

        if current_value < new_value {
            ch_data.set_light(chunk_block, new_value, light_type);
            add_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, new_value, MIN_LEVEL));
        }
    }
    else {
        if current_value == MIN_LEVEL { return }

        // just remove if neighbor block not emit light
        let remove_condition = if light_type == LightType::Block {
            current_value < old_value && current_block.light_emission == MIN_LEVEL
        }
        else { // if down light value is 15 then remove down light
            current_value < old_value || (dir == Directions::Down && current_value == MAX_LEVEL)
        };


        // set neighborLight to 0 and add to remove_queue
        if remove_condition {
            ch_data.set_light(chunk_block, MIN_LEVEL, light_type);
            remove_queue.as_mut().unwrap().push_back(LightQueueData::new(chunk_data.clone(), chunk_block, MIN_LEVEL, current_value));
        }
        else { // if neighbor value is > that old value, then add neighbor block to add light queue
            add_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, current_value, MIN_LEVEL));
        }
    }
}

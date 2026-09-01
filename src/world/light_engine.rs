use std::{collections::{HashMap, VecDeque}, sync::{Arc, Mutex, RwLock}};

use crate::{math::Vec3i, world::{Chunk, blocks::BlockProperties, chunk::{ChunkData, NeighborsChunksData, chunk::Directions}}};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Sky,
    Block,
    Both,
}

pub const MAX_LEVEL: u8 = 15;
pub const MIN_LEVEL: u8 = 0;
pub const BLOCK_MASK: u8 = 0b11110000;
pub const SKY_MASK: u8 = 0b00001111;

#[derive(Clone, Copy, PartialEq, Eq)]
enum UpdateType {
    Add,
    Remove
}

struct LightQueueData{
    chunk_data: Arc<RwLock<ChunkData>>,
    chunk_block: Vec3i,
    light_level: u8,
    old_value: u8,
}

impl LightQueueData {
    pub fn new(
        chunk_data: Arc<RwLock<ChunkData>>,
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
    let mut pool = QUEUE_DATA_POOL.lock().unwrap();

    if let Some(mut queue) = pool.pop() {
        queue.clear();

        return queue;
    }

    return VecDeque::new();
}

fn restore_queue(queue: VecDeque<LightQueueData>) {
    QUEUE_DATA_POOL.lock().unwrap().push(queue);
}


pub fn init_chunk_light(chunk_data: Arc<RwLock<ChunkData>>) {
    init_sky_light(chunk_data.clone());
    init_block_light(chunk_data);

    //let s = chunk_data.read().unwrap();
    //std::thread::sleep(std::time::Duration::from_millis(20));
    //chunkData.loadLightStage = false;
}

pub fn update_light(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>,
    chunk_data: Arc<RwLock<ChunkData>>,
    chunk_block: Vec3i,
    old_block: &BlockProperties,
    new_block: &BlockProperties
) {
    // remove around light sources
    if old_block.light_filter != MIN_LEVEL ||
        old_block.light_emission != MIN_LEVEL ||
        new_block.light_filter != MIN_LEVEL
    {
        remove_block_light_source(chunks_map.clone(), chunk_data.clone(), chunk_block);
    }

    if new_block.light_emission > MIN_LEVEL {
        add_block_light_source(chunks_map.clone(), chunk_data.clone(), chunk_block, new_block);
    }

    let mut add_sky_queue = get_queue();
    let mut remove_queue = get_queue();

    // uses remove light logic to update skylight around
    remove_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, MIN_LEVEL, chunk_data.read().unwrap().get_light(chunk_block, LightType::Sky)));

    chunk_data.write().unwrap().set_light(chunk_block, MIN_LEVEL, LightType::Sky);

    let mut neighbors = NeighborsChunksData::new_from_map(chunks_map.clone(), chunk_data.read().unwrap().position, false);

    while let Some(data) = remove_queue.pop_front()
    {
        process_light_logic(&Some(chunks_map.clone()), &data, LightType::Sky, UpdateType::Remove, &mut add_sky_queue, &mut Some(&mut remove_queue), &mut neighbors);
    }

    process_queue(Some(chunks_map.clone()), LightType::Sky, UpdateType::Add, &mut add_sky_queue, &mut None, &mut neighbors);

    restore_queue(add_sky_queue);
    restore_queue(remove_queue);
}

pub fn update_light_in_border_neighbors(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>,
    chunk_data: Arc<RwLock<ChunkData>>,
    neighbors_chunks_data: NeighborsChunksData
) {
    update_light_in_border(chunks_map.clone(), chunk_data.clone(), LightType::Block);
    update_light_in_border(chunks_map.clone(), chunk_data.clone(), LightType::Sky);

    if let Some(north) = neighbors_chunks_data.north {
        north.write().unwrap().light_gen_stage = true;
        update_light_in_border(chunks_map.clone(), north.clone(), LightType::Block);
        update_light_in_border(chunks_map.clone(), north.clone(), LightType::Sky);
        north.write().unwrap().light_gen_stage = false;
    }
    if let Some(south) = neighbors_chunks_data.south {
        south.write().unwrap().light_gen_stage = true;
        update_light_in_border(chunks_map.clone(), south.clone(), LightType::Block);
        update_light_in_border(chunks_map.clone(), south.clone(), LightType::Sky);
        south.write().unwrap().light_gen_stage = false;
    }
    if let Some(west) = neighbors_chunks_data.west {
        west.write().unwrap().light_gen_stage = true;
        update_light_in_border(chunks_map.clone(), west.clone(), LightType::Block);
        update_light_in_border(chunks_map.clone(), west.clone(), LightType::Sky);
        west.write().unwrap().light_gen_stage = false;
    }
    if let Some(east) = neighbors_chunks_data.east {
        east.write().unwrap().light_gen_stage = true;
        update_light_in_border(chunks_map.clone(), east.clone(), LightType::Block);
        update_light_in_border(chunks_map.clone(), east.clone(), LightType::Sky);
        east.write().unwrap().light_gen_stage = false;
    }

    chunk_data.write().unwrap().light_gen_stage = false;
}

fn update_light_in_border(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>,
    chunk_data: Arc<RwLock<ChunkData>>,
    light_type: LightType
) {
    if light_type == LightType::Block && !chunk_data.read().unwrap().contains_emissive_blocks {
        return;
    }

    let add_light_border_in_queue = |data: &ChunkData, queue: &mut VecDeque<LightQueueData>, chunk_block: Vec3i| {
        let light_value = data.get_light(chunk_block, light_type);

        if light_value == MIN_LEVEL { return }

        queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, light_value, MIN_LEVEL));
    };

    let mut add_queue = get_queue();
    let chunk_pos: Vec3i;

    {
        let data = chunk_data.read().unwrap();
        chunk_pos = data.position;

        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
            add_light_border_in_queue(&data, &mut add_queue, Vec3i::new(x, y, 0));
            add_light_border_in_queue(&data, &mut add_queue, Vec3i::new(x, y, Chunk::CHUNK_SIZE_MINUS_ONE.z));
        }
        }

        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            add_light_border_in_queue(&data, &mut add_queue, Vec3i::new(0, y, z));
            add_light_border_in_queue(&data, &mut add_queue, Vec3i::new(Chunk::CHUNK_SIZE_MINUS_ONE.x, y, z));
        }
        }
    }

    let mut neighbors_data = NeighborsChunksData::new_from_map(chunks_map.clone(), chunk_pos, false);

    process_queue(Some(chunks_map), light_type, UpdateType::Add, &mut add_queue, &mut None, &mut neighbors_data);
    restore_queue(add_queue);
}

fn add_block_light_source(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>,
    chunk_data: Arc<RwLock<ChunkData>>,
    chunk_block: Vec3i,
    block: &BlockProperties
) {
    let new_value = chunk_data.read().unwrap().get_light(chunk_block, LightType::Block).max(block.light_emission);
    chunk_data.write().unwrap().set_light(chunk_block, new_value, LightType::Block);

    let mut add_queue = get_queue();

    add_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, new_value, MIN_LEVEL));

    let mut neighbors = NeighborsChunksData::new_from_map(chunks_map.clone(), chunk_data.read().unwrap().position, false);

    process_queue(Some(chunks_map), LightType::Block, UpdateType::Add, &mut add_queue, &mut None, &mut neighbors);
    restore_queue(add_queue);
}

fn remove_block_light_source(
    chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>,
    chunk_data: Arc<RwLock<ChunkData>>,
    chunk_block: Vec3i
) {
    let mut add_queue = get_queue();
    let mut remove_queue = get_queue();

    let current_value = chunk_data.read().unwrap().get_light(chunk_block, LightType::Block);
    remove_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, MIN_LEVEL, current_value));

    chunk_data.write().unwrap().set_light(chunk_block, MIN_LEVEL, LightType::Block);

    let mut neighbors = NeighborsChunksData::new_from_map(chunks_map.clone(), chunk_data.read().unwrap().position, false);

    while let Some(data) = remove_queue.pop_front()
    {
        process_light_logic(&Some(chunks_map.clone()), &data, LightType::Block, UpdateType::Remove, &mut add_queue, &mut Some(&mut remove_queue), &mut neighbors);
    }

    // add block light if necessary
    process_queue(Some(chunks_map.clone()), LightType::Block, UpdateType::Add, &mut add_queue, &mut Some(&mut remove_queue), &mut neighbors);
    restore_queue(add_queue);
    restore_queue(remove_queue);
}

fn init_sky_light(chunk_data: Arc<RwLock<ChunkData>>) {
    let mut add_queue = get_queue();

    {
        let mut data = chunk_data.write().unwrap();

        for x in 0..Chunk::CHUNK_SIZE.x {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let mut current_level = MAX_LEVEL;

            for y in (0..=Chunk::CHUNK_SIZE_MINUS_ONE.y).rev() {
                let chunk_block = Vec3i::new(x, y, z);
                let current_block = data.get_block_properties(chunk_block);

                current_level = current_level.wrapping_sub(current_block.light_filter);

                // a underflow occurred, then, current_level does have a value > MAX_LEVEL
                if current_level > MAX_LEVEL {
                    current_level = MIN_LEVEL;
                }

                if current_level > MIN_LEVEL && current_block.light_filter < MAX_LEVEL {
                    add_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, current_level, MIN_LEVEL));
                }

                if current_level > MIN_LEVEL {
                    data.set_light(chunk_block, current_level, LightType::Sky);
                }
            }
        }
        }
    }

    process_queue(None, LightType::Sky, UpdateType::Add, &mut add_queue, &mut None, &mut NeighborsChunksData::EMPTY);
    restore_queue(add_queue);
}

fn init_block_light(chunk_data: Arc<RwLock<ChunkData>>) {
    let mut add_queue = get_queue();

    {
        let mut data = chunk_data.write().unwrap();

        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let chunk_block = Vec3i::new(x, y, z);

            let block = data.get_block_properties(chunk_block);

            if block.light_emission == MIN_LEVEL {
                continue;
            }

            data.contains_emissive_blocks = true;
            data.set_light(chunk_block, block.light_emission, LightType::Block);
            add_queue.push_back(LightQueueData::new(chunk_data.clone(), chunk_block, block.light_emission, MIN_LEVEL));
        }
        }
        }

    }

    process_queue(None, LightType::Block, UpdateType::Add, &mut add_queue, &mut None, &mut NeighborsChunksData::EMPTY);
    restore_queue(add_queue);
}

fn process_queue(
    chunks_map: Option<Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>>,
    light_type: LightType,
    update_type: UpdateType,
    add_queue: &mut VecDeque<LightQueueData>,
    remove_queue: &mut Option<&mut VecDeque<LightQueueData>>,
    neighbor_chunks: &mut NeighborsChunksData,
) {
    while let Some(data) = add_queue.pop_front() {
        process_light_logic(
            &chunks_map,
            &data,
            light_type,
            update_type,
            add_queue,
            remove_queue,
            neighbor_chunks,
        );
    }
}

fn process_light_logic(
    chunks_map: &Option<Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>>,
    light_data: &LightQueueData,
    light_type: LightType,
    update_type: UpdateType,
    add_queue: &mut VecDeque<LightQueueData>,
    remove_queue: &mut Option<&mut VecDeque<LightQueueData>>,
    neighbor_chunks: &mut NeighborsChunksData,
) {
    let new_value = (light_data.light_level as i8 - 1 as i8).max(0) as u8;
    let old_value = light_data.old_value;

    if let Some(chunks_map) = chunks_map {
        neighbor_chunks.change_from_map(chunks_map.clone(), light_data.chunk_data.read().unwrap().position, false);
    }

    let x = light_data.chunk_block.x;
    let y = light_data.chunk_block.y;
    let z = light_data.chunk_block.z;

    // up
    if y < Chunk::CHUNK_SIZE_MINUS_ONE.y {
        let around_block = Vec3i::new(x, y + 1, z);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::Up, around_block, light_data.chunk_data.clone(),
            add_queue, remove_queue);
    }

    // down
    if y > 0 {
        let around_block = Vec3i::new(x, y - 1, z);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::Down, around_block, light_data.chunk_data.clone(),
            add_queue, remove_queue);
    }

    // south
    if z < Chunk::CHUNK_SIZE_MINUS_ONE.z {
        let around_block = Vec3i::new(x, y, z + 1);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::South, around_block, light_data.chunk_data.clone(),
            add_queue, remove_queue);
    }
    else if let Some(ref south) = neighbor_chunks.south {
        let around_block = Vec3i::new(x, y, 0);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::South, around_block, south.clone(),
            add_queue, remove_queue);
    }

    // north
    if z > 0 {
        let around_block = Vec3i::new(x, y, z - 1);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::North, around_block, light_data.chunk_data.clone(),
            add_queue, remove_queue);
    }
    else if let Some(ref north) = neighbor_chunks.north {
        let around_block = Vec3i::new(x, y, Chunk::CHUNK_SIZE_MINUS_ONE.z);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::North, around_block, north.clone(),
            add_queue, remove_queue);
    }

    // west
    if x > 0 {
        let around_block = Vec3i::new(x - 1, y, z);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::West, around_block, light_data.chunk_data.clone(),
            add_queue, remove_queue);
    }
    else if let Some(ref west) = neighbor_chunks.west {
        let around_block = Vec3i::new(Chunk::CHUNK_SIZE_MINUS_ONE.x, y, z);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::West, around_block, west.clone(),
            add_queue, remove_queue);
    }

    // east
    if x < Chunk::CHUNK_SIZE_MINUS_ONE.x {
        let around_block = Vec3i::new(x + 1, y, z);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::East, around_block, light_data.chunk_data.clone(),
            add_queue, remove_queue);
    }
    else if let Some(ref east) = neighbor_chunks.east {
        let around_block = Vec3i::new(0, y, z);

        update_light_logic(old_value, new_value, light_type, update_type, Directions::East, around_block, east.clone(),
            add_queue, remove_queue);
    }
}

fn update_light_logic(
    old_value: u8,
    mut new_value: u8,
    light_type: LightType,
    update_type: UpdateType,
    dir: Directions,
    mut chunk_block: Vec3i,
    chunk_data: Arc<RwLock<ChunkData>>,
    add_queue: &mut VecDeque<LightQueueData>,
    remove_queue: &mut Option<&mut VecDeque<LightQueueData>>
) {
    let mut ch_data = chunk_data.write().unwrap();

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

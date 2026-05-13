use std::{rc::Rc, cell::RefCell};
use std::collections::HashMap;
use crate::resources::ResourceManager;
use crate::ui::screens_manager::ScreenManager;
use crate::world::Planet;
use crate::world::Player;
use crate::world::blocks::BlocksManager;

use serde::Deserialize;
use serde::Serialize;

pub struct Game {
    player: Player,
    planet: Planet,
    
    screen_manager: ScreenManager,
    resource_manager: Rc<RefCell<ResourceManager>>,
    blocks_manager: BlocksManager,

    characters_info: [Character; 95],
}

#[derive(Serialize, Copy, Clone)]
struct Character
{
    pub uv: [i32; 4],
    pub advance: [i32; 2],
}

#[derive(Serialize)]
struct JsonTest {
    pub chars: HashMap<char, Character>,
}


impl Game {
    pub fn new() -> Self { 
        Self { 
            player: Player::new(),
            planet: Planet::new(),

            screen_manager: ScreenManager::new(),
            resource_manager: Rc::new(RefCell::new(ResourceManager::new())),
            blocks_manager: BlocksManager::new(),

            characters_info: [Character{uv: [0; 4], advance: [0; 2]}; 95],
        } 
    }

    pub fn start(&mut self) {
        self.resource_manager.borrow_mut().start();
        self.blocks_manager.start();
        self.screen_manager.start(self.resource_manager.clone());
        
        self.player.start();
        self.planet.start(self.resource_manager.clone());
        //let now = std::time::Instant::now();

        //println!("{}", now.elapsed().as_micros());

        let mut test = JsonTest{chars: HashMap::new()};
        test.chars.reserve(self.characters_info.len());

        self.start_text();
        for i in 32..127 {
            test.chars.insert(i as u8 as char, self.characters_info[i - 32]);
        }

        let json_str = serde_json::ser::to_string(&test);

        if let Ok(json_str) = json_str {
            println!("{json_str}");
        }
        else {
            println!("error to serialize json");
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.player.update(dt);

        
        self.screen_manager.update(dt);
    }

    pub fn render(&mut self) {
        
        self.screen_manager.draw();
        self.planet.draw();
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.screen_manager.resize(width as f32, height as f32);
        self.player.camera.resize(width as f32, height as f32)
    }

    pub fn start_text(&mut self) {
        self.characters_info[' ' as usize - 32].advance = [5, 8];
        self.characters_info['!' as usize - 32].advance = [2, 8];
        self.characters_info['\"' as usize - 32].advance = [5, 8];
        self.characters_info['#' as usize - 32].advance = [6, 8];
        self.characters_info['$' as usize - 32].advance = [6, 8];
        self.characters_info['%' as usize - 32].advance = [6, 8];
        self.characters_info['&' as usize - 32].advance = [6, 8];
        self.characters_info['\'' as usize - 32].advance = [3, 8];
        self.characters_info['(' as usize - 32].advance = [5, 8];
        self.characters_info[')' as usize - 32].advance = [5, 8];
        self.characters_info['*' as usize - 32].advance = [5, 8];
        self.characters_info['+' as usize - 32].advance = [6, 8];
        self.characters_info[',' as usize - 32].advance = [2, 8];
        self.characters_info['-' as usize - 32].advance = [6, 8];
        self.characters_info['.' as usize - 32].advance = [2, 8];
        self.characters_info['/' as usize - 32].advance = [6, 8];
        self.characters_info['0' as usize - 32].advance = [6, 8];
        self.characters_info['1' as usize - 32].advance = [4, 8];
        self.characters_info['2' as usize - 32].advance = [6, 8];
        self.characters_info['3' as usize - 32].advance = [6, 8];
        self.characters_info['4' as usize - 32].advance = [6, 8];
        self.characters_info['5' as usize - 32].advance = [6, 8];
        self.characters_info['6' as usize - 32].advance = [6, 8];
        self.characters_info['7' as usize - 32].advance = [6, 8];
        self.characters_info['8' as usize - 32].advance = [6, 8];
        self.characters_info['9' as usize - 32].advance = [6, 8];
        self.characters_info[':' as usize - 32].advance = [2, 8];
        self.characters_info[';' as usize - 32].advance = [2, 8];
        self.characters_info['<' as usize - 32].advance = [5, 8];
        self.characters_info['=' as usize - 32].advance = [6, 8];
        self.characters_info['>' as usize - 32].advance = [5, 8];
        self.characters_info['?' as usize - 32].advance = [6, 8];
        self.characters_info['@' as usize - 32].advance = [7, 8];
        self.characters_info['A' as usize - 32].advance = [6, 8];
        self.characters_info['B' as usize - 32].advance = [6, 8];
        self.characters_info['C' as usize - 32].advance = [6, 8];
        self.characters_info['D' as usize - 32].advance = [6, 8];
        self.characters_info['E' as usize - 32].advance = [6, 8];
        self.characters_info['F' as usize - 32].advance = [6, 8];
        self.characters_info['G' as usize - 32].advance = [6, 8];
        self.characters_info['H' as usize - 32].advance = [6, 8];
        self.characters_info['I' as usize - 32].advance = [4, 8];
        self.characters_info['J' as usize - 32].advance = [6, 8];
        self.characters_info['K' as usize - 32].advance = [6, 8];
        self.characters_info['L' as usize - 32].advance = [6, 8];
        self.characters_info['M' as usize - 32].advance = [6, 8];
        self.characters_info['N' as usize - 32].advance = [6, 8];
        self.characters_info['O' as usize - 32].advance = [6, 8];
        self.characters_info['P' as usize - 32].advance = [6, 8];
        self.characters_info['Q' as usize - 32].advance = [6, 8];
        self.characters_info['R' as usize - 32].advance = [6, 8];
        self.characters_info['S' as usize - 32].advance = [6, 8];
        self.characters_info['T' as usize - 32].advance = [6, 8];
        self.characters_info['U' as usize - 32].advance = [6, 8];
        self.characters_info['V' as usize - 32].advance = [6, 8];
        self.characters_info['W' as usize - 32].advance = [6, 8];
        self.characters_info['X' as usize - 32].advance = [6, 8];
        self.characters_info['Y' as usize - 32].advance = [6, 8];
        self.characters_info['Z' as usize - 32].advance = [6, 8];
        self.characters_info['[' as usize - 32].advance = [4, 8];
        self.characters_info['\\' as usize - 32].advance = [6, 8];
        self.characters_info[']' as usize - 32].advance = [4, 8];
        self.characters_info['^' as usize - 32].advance = [6, 8];
        self.characters_info['_' as usize - 32].advance = [6, 8];
        self.characters_info['`' as usize - 32].advance = [6, 8];
        self.characters_info['a' as usize - 32].advance = [6, 8];
        self.characters_info['b' as usize - 32].advance = [6, 8];
        self.characters_info['c' as usize - 32].advance = [6, 8];
        self.characters_info['d' as usize - 32].advance = [6, 8];
        self.characters_info['e' as usize - 32].advance = [6, 8];
        self.characters_info['f' as usize - 32].advance = [5, 8];
        self.characters_info['g' as usize - 32].advance = [6, 8];
        self.characters_info['h' as usize - 32].advance = [6, 8];
        self.characters_info['i' as usize - 32].advance = [2, 8];
        self.characters_info['j' as usize - 32].advance = [6, 8];
        self.characters_info['k' as usize - 32].advance = [5, 8];
        self.characters_info['l' as usize - 32].advance = [3, 8];
        self.characters_info['m' as usize - 32].advance = [6, 8];
        self.characters_info['n' as usize - 32].advance = [6, 8];
        self.characters_info['o' as usize - 32].advance = [6, 8];
        self.characters_info['p' as usize - 32].advance = [6, 8];
        self.characters_info['q' as usize - 32].advance = [6, 8];
        self.characters_info['r' as usize - 32].advance = [6, 8];
        self.characters_info['s' as usize - 32].advance = [6, 8];
        self.characters_info['t' as usize - 32].advance = [4, 8];
        self.characters_info['u' as usize - 32].advance = [6, 8];
        self.characters_info['v' as usize - 32].advance = [6, 8];
        self.characters_info['w' as usize - 32].advance = [6, 8];
        self.characters_info['x' as usize - 32].advance = [6, 8];
        self.characters_info['y' as usize - 32].advance = [6, 8];
        self.characters_info['z' as usize - 32].advance = [6, 8];
        self.characters_info['{' as usize - 32].advance = [5, 8];
        self.characters_info['|' as usize - 32].advance = [2, 8];
        self.characters_info['}' as usize - 32].advance = [5, 8];
        self.characters_info['~' as usize - 32].advance = [7, 8];

        let mut posX = 0;
        let mut posY = 0;
        let mut it = 0;

        for c in 32..127  {
            let advance = self.characters_info[c - 32].advance;

            self.characters_info[c - 32].uv = [posX, posY, posX + advance[0], posY + advance[1]];

            posX += 8;
            it += 1;
            if it == 16
            {
                posY += 8;
                posX = 0;
                it = 0;
            }
        }
    }
}
pub mod block_properties;
pub mod block_registry;
pub mod block_behaviors;

pub mod default_opaque_cube;

pub mod air;
pub mod water_block;
pub mod snow_layer;
pub mod short_grass;
pub mod red_flower;
pub mod yellow_flower;
pub mod dead_bush;
pub mod smooth_stone_slab;
pub mod torch;
pub mod glass_block;
pub mod oak_leaves;
pub mod white_oak_leaves;


pub use {
    block_registry::BlockRegistry,
    block_properties::*,
    block_behaviors::BlockBehaviors,

    default_opaque_cube::*,

    air::*,
    water_block::*,
    snow_layer::*,
    short_grass::*,
    red_flower::*,
    yellow_flower::*,
    dead_bush::*,
    smooth_stone_slab::*,
    torch::*,
    glass_block::*,
    oak_leaves::*,
    white_oak_leaves::*,
};


use std::collections::HashMap;

// 1. Propriedades possíveis para os estados
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Facing { North, South, East, West }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockProperty {
    Facing(Facing),
    Powered(bool),
    Age(u8),
}

// 2. O Bloco Base (O "Singleton")
// Define o comportamento e quais propriedades ele aceita.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BlockType {
    pub id: &'static str,
    pub flammable: bool,
}

// 3. O Block State (O Estado Único e Imutável)
// Contém a referência ao bloco pai e a combinação exata de propriedades.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockState {
    pub block_type: &'static BlockType, // Aponta para o Singleton global
    pub properties: Vec<BlockProperty>, // Combinação fixa deste estado
}

pub struct BlockRegistry2 {
    // Registra o tipo base pelo ID
    types: HashMap<&'static str, &'static BlockType>,
    // Guarda todos os estados possíveis gerados na inicialização
    states: Vec<BlockState>,
}

impl BlockRegistry2 {
    pub fn new() -> Self {
        let mut registry = Self {
            types: HashMap::new(),
            states: Vec::new(),
        };
        registry.initialize();
        registry
    }

    fn initialize(&mut self) {
        // Exemplo: Criando a alavanca (Lever)
        // Usamos Box::leak para transformar o objeto em uma referência &'static válida para todo o jogo
        let lever_type = Box::leak(Box::new(BlockType {
            id: "minecraft:lever",
            flammable: false,
        }));

        self.types.insert(lever_type.id, lever_type);

        // Gerando TODAS as combinações possíveis para a alavanca (4 direções x 2 estados de energia)
        let facings = [Facing::North, Facing::South, Facing::East, Facing::West];
        let power_states = [true, false];

        for &facing in &facings {
            for &powered in &power_states {
                let state = BlockState {
                    block_type: lever_type,
                    properties: vec![
                        BlockProperty::Facing(facing),
                        BlockProperty::Powered(powered),
                    ],
                };
                self.states.push(state);
            }
        }
    }

    // Retorna o índice (ou ID) de um estado baseado nas propriedades desejadas
    pub fn get_state_index(&self, id: &str, target_props: &[BlockProperty]) -> Option<usize> {
        self.states.iter().position(|state| {
            state.block_type.id == id &&
            target_props.iter().all(|p| state.properties.contains(p))
        })
    }
}

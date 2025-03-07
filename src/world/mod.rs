mod chunk;
mod generation;

use std::collections::HashMap;
pub use chunk::Chunk;
use generation::WorldGenerator;

pub struct World {
    seed: u64,
    chunks: HashMap<(i32, i32), Chunk>,
    generator: WorldGenerator,
}

impl World {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            chunks: HashMap::new(),
            generator: WorldGenerator::new(seed),
        }
    }

    pub fn get_chunk(&mut self, x: i32, z: i32) -> &Chunk {
        if !self.chunks.contains_key(&(x, z)) {
            let chunk = self.generator.generate_chunk(x, z);
            self.chunks.insert((x, z), chunk);
        }
        
        self.chunks.get(&(x, z)).unwrap()
    }
} 
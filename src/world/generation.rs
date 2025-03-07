use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use super::chunk::Chunk;

pub struct WorldGenerator {
    rng: StdRng,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Chunk {
        let mut chunk = Chunk::new();
        
        // Very simple terrain generation for example
        for x in 0..16 {
            for z in 0..16 {
                // Generate height from 60 to 70 blocks
                let height = 60 + (self.rng.gen::<f32>() * 10.0) as usize;
                
                // Bedrock at the bottom
                chunk.set_block(x, 0, z, 1);
                
                // Stone up to y=50
                for y in 1..50 {
                    chunk.set_block(x, y, z, 2);
                }
                
                // Dirt from 50 to height-1
                for y in 50..height-1 {
                    chunk.set_block(x, y, z, 3);
                }
                
                // Grass on top
                chunk.set_block(x, height-1, z, 4);
            }
        }
        
        chunk
    }
} 
pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

#[derive(Clone)]
pub struct Chunk {
    blocks: [[[u16; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: u16) {
        if x < CHUNK_WIDTH && y < CHUNK_HEIGHT && z < CHUNK_WIDTH {
            self.blocks[x][y][z] = block_id;
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u16 {
        if x < CHUNK_WIDTH && y < CHUNK_HEIGHT && z < CHUNK_WIDTH {
            self.blocks[x][y][z]
        } else {
            0
        }
    }
} 
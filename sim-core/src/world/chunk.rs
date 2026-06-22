use serde::{Deserialize, Serialize};

use super::voxel::{VoxelFields, VoxelView, VoxelViewMut, CHUNK_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub fields: VoxelFields,
}

impl Chunk {
    pub fn new(coord: ChunkCoord) -> Self {
        Self {
            coord,
            fields: VoxelFields::new_empty(),
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> VoxelView {
        self.fields.get(x, y, z)
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> VoxelViewMut<'_> {
        self.fields.get_mut(x, y, z)
    }

    pub fn in_bounds(x: i32, y: i32, z: i32) -> bool {
        x >= 0 && y >= 0 && z >= 0 && (x as usize) < CHUNK_SIZE && (y as usize) < CHUNK_SIZE
            && (z as usize) < CHUNK_SIZE
    }
}

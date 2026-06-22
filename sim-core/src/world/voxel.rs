pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[inline]
pub fn idx(x: usize, y: usize, z: usize) -> usize {
    x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE
}

#[inline]
pub fn idx_valid(x: usize, y: usize, z: usize) -> Option<usize> {
    if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
        Some(idx(x, y, z))
    } else {
        None
    }
}

/// Structure-of-arrays voxel storage for cache-friendly iteration.
#[derive(Debug, Clone)]
pub struct VoxelFields {
    pub hard_mineral: Vec<f32>,
    pub soft_mineral: Vec<f32>,
    pub coarse_mineral: Vec<f32>,
    pub clay: Vec<f32>,
    pub organic: Vec<f32>,
    pub binder: Vec<f32>,
    pub solid_fraction: Vec<f32>,
    pub void_fraction: Vec<f32>,
    pub surface_water: Vec<f32>,
    pub water_content: Vec<f32>,
    pub ice: Vec<f32>,
    pub snow: Vec<f32>,
    pub temperature: Vec<f32>,
    pub humidity: Vec<f32>,
    pub porosity: Vec<f32>,
    pub permeability: Vec<f32>,
    pub erosion_damage: Vec<f32>,
    pub structural_strength: Vec<f32>,
    pub load: Vec<f32>,
}

impl Default for VoxelFields {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl VoxelFields {
    pub fn new_empty() -> Self {
        let zero = vec![0.0; CHUNK_VOLUME];
        Self {
            hard_mineral: zero.clone(),
            soft_mineral: zero.clone(),
            coarse_mineral: zero.clone(),
            clay: zero.clone(),
            organic: zero.clone(),
            binder: zero.clone(),
            solid_fraction: zero.clone(),
            void_fraction: vec![1.0; CHUNK_VOLUME],
            surface_water: zero.clone(),
            water_content: zero.clone(),
            ice: zero.clone(),
            snow: zero.clone(),
            temperature: vec![15.0; CHUNK_VOLUME],
            humidity: vec![0.5; CHUNK_VOLUME],
            porosity: vec![0.3; CHUNK_VOLUME],
            permeability: vec![0.2; CHUNK_VOLUME],
            erosion_damage: zero.clone(),
            structural_strength: vec![1.0; CHUNK_VOLUME],
            load: zero,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> VoxelView {
        let i = idx(x, y, z);
        VoxelView {
            hard_mineral: self.hard_mineral[i],
            soft_mineral: self.soft_mineral[i],
            coarse_mineral: self.coarse_mineral[i],
            clay: self.clay[i],
            organic: self.organic[i],
            binder: self.binder[i],
            solid_fraction: self.solid_fraction[i],
            void_fraction: self.void_fraction[i],
            surface_water: self.surface_water[i],
            water_content: self.water_content[i],
            ice: self.ice[i],
            snow: self.snow[i],
            temperature: self.temperature[i],
            humidity: self.humidity[i],
            porosity: self.porosity[i],
            permeability: self.permeability[i],
            erosion_damage: self.erosion_damage[i],
            structural_strength: self.structural_strength[i],
            load: self.load[i],
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> VoxelViewMut<'_> {
        let i = idx(x, y, z);
        VoxelViewMut {
            hard_mineral: &mut self.hard_mineral[i],
            soft_mineral: &mut self.soft_mineral[i],
            coarse_mineral: &mut self.coarse_mineral[i],
            clay: &mut self.clay[i],
            organic: &mut self.organic[i],
            binder: &mut self.binder[i],
            solid_fraction: &mut self.solid_fraction[i],
            void_fraction: &mut self.void_fraction[i],
            surface_water: &mut self.surface_water[i],
            water_content: &mut self.water_content[i],
            ice: &mut self.ice[i],
            snow: &mut self.snow[i],
            temperature: &mut self.temperature[i],
            humidity: &mut self.humidity[i],
            porosity: &mut self.porosity[i],
            permeability: &mut self.permeability[i],
            erosion_damage: &mut self.erosion_damage[i],
            structural_strength: &mut self.structural_strength[i],
            load: &mut self.load[i],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VoxelView {
    pub hard_mineral: f32,
    pub soft_mineral: f32,
    pub coarse_mineral: f32,
    pub clay: f32,
    pub organic: f32,
    pub binder: f32,
    pub solid_fraction: f32,
    pub void_fraction: f32,
    pub surface_water: f32,
    pub water_content: f32,
    pub ice: f32,
    pub snow: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub porosity: f32,
    pub permeability: f32,
    pub erosion_damage: f32,
    pub structural_strength: f32,
    pub load: f32,
}

impl VoxelView {
    pub fn wet_mineral(&self) -> f32 {
        self.clay + self.soft_mineral
    }

    pub fn decay_signal(&self) -> f32 {
        self.organic * self.humidity
    }
}

#[derive(Debug)]
pub struct VoxelViewMut<'a> {
    pub hard_mineral: &'a mut f32,
    pub soft_mineral: &'a mut f32,
    pub coarse_mineral: &'a mut f32,
    pub clay: &'a mut f32,
    pub organic: &'a mut f32,
    pub binder: &'a mut f32,
    pub solid_fraction: &'a mut f32,
    pub void_fraction: &'a mut f32,
    pub surface_water: &'a mut f32,
    pub water_content: &'a mut f32,
    pub ice: &'a mut f32,
    pub snow: &'a mut f32,
    pub temperature: &'a mut f32,
    pub humidity: &'a mut f32,
    pub porosity: &'a mut f32,
    pub permeability: &'a mut f32,
    pub erosion_damage: &'a mut f32,
    pub structural_strength: &'a mut f32,
    pub load: &'a mut f32,
}

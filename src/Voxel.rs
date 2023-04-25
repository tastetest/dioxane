#[derive(Copy, Clone, Debug)]
pub struct Voxel {
    pub id: u32,
    pub position: Vec3,
    pub normal: [f32; 3],
    pub isGravityBlock: boolean,
}

enum BlockType {
    transparent,
    solid,
}

impl Voxel {
    // todo!
}

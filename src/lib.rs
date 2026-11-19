
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockType{
    Air,
    Grass,
    Dirt,
    Diamond,
    Wood,
    Lava,
    Teleport,
    Leave
}

#[test]
fn it_works(){
let my_block =BlockType::Air;
let other_block=my_block;
//now i am going to test if they both are equal or not 
assert_eq!(my_block,other_block);
}

    pub const WORLD_W: usize = 32;
    pub const WORLD_H: usize = 32;
    pub const WORLD_D: usize = 32;

struct VoxelWorld{
    pub blocks:[BlockType;WORLD_W*WORLD_H*WORLD_D],
}

impl VoxelWorld{
    pub fn new ()->Self{
        Self{
      blocks:[BlockType::Air;WORLD_W*WORLD_H*WORLD_D],
    }
}}
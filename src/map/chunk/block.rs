use super::block_type::BlockType;

#[derive(Copy, Clone)]
pub struct Block {
    pub block_type: &'static BlockType,
}

impl From<&'static BlockType> for Block {
    fn from(value: &'static BlockType) -> Self {
        Block {
            block_type: value
        }
    }
}

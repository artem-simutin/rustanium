use crate::block::Block;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new_genesis();
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        for i in 1..=u64::MAX {
            let prev_block = Self::get_block_by_index(self, i - 1)
                .expect("Something went wrong getting prev block");
            let mut block = Block::new(
                i,
                prev_block.hash.clone(),
                0,
                prev_block.difficulty,
                "Another block".to_string(),
            );
            block.mine();
            println!("Mined block: {:?}", &block);
            self.add_block(block);
        }
        Ok(())
    }

    fn get_block_by_index(&self, index: u64) -> Result<&Block, String> {
        let chain = self.get_chain();
        let result = match chain.get(index as usize) {
            Some(block) => Ok(block),
            None => Err(format!("There is no block with this index: {}", &index)),
        };
        result
    }

    fn get_chain(&self) -> &Vec<Block> {
        &self.blocks
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

use super::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn new()  -> Blockchain {
        let mut blocks = Vec::new();
        blocks.push(Block::genesis());
        Blockchain {
            blocks
        }
    }
    
    pub fn last_block(&self) -> &Block {
        &self.blocks[self.blocks.len() - 1]
    }

    pub fn add_new_block(&mut self, data: &str) {
        let last_block = Self::last_block(self);
        let new_block = Block::mine(&last_block, data);
        self.blocks.push(new_block);
    }

    pub fn is_valid(blocks: &Vec<Block>) -> bool {
        let first_block = &blocks[0];
        if !first_block.is_equal(&Block::genesis()) {
            return false
        }

        for i in 1..blocks.len() {
            let block = &blocks[i];
            let prev_block = &blocks[i-1];
            if block.last_hash != prev_block.hash || block.hash != Block::block_hash(block) {
                return false
            }
        }

        true
    }

    pub fn replace_chain(&mut self, new_chain: &Vec<Block>) {
        if new_chain.len() <= self.blocks.len() {
            println!("Received chain is not longer than the current chain");
        } else if !Self::is_valid(new_chain) {
            println!("The received chain is invalid");
        } else {
            println!("Replace the current chain with new chain");
            self.blocks = new_chain.to_vec();
        }
    }

    pub fn is_equal(&self, bc: &Blockchain) -> bool {
        if self.blocks.len() != bc.blocks.len() {
            return false;
        }

        for i in 0..self.blocks.len() {
            if !self.blocks[i].is_equal(&bc.blocks[i]) {
                return false;
            }
        }

        true
    }
}

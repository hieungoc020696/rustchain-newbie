#[cfg(test)]
mod block_tests {
    use rustchain::Block;
    
    #[test]
    fn set_data_match_the_input() {
        let data = "bar";
        let last_block = Block::genesis();
        let block = Block::mine(&last_block, data);
        assert_eq!(block.data, data);
    }

    #[test]
    fn set_last_hash_match_last_block() {
        let data = "bar";
        let last_block = Block::genesis();
        let block = Block::mine(&last_block, data);
        assert_eq!(block.last_hash, last_block.hash);
    }
}
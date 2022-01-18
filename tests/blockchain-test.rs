#[cfg(test)]
mod blockchain_tests {
    use rustchain::Block;
    use rustchain::Blockchain;

    fn init() -> Blockchain {
        Blockchain::new()
    }
    
    #[test]
    fn start_with_genesis_block() {
        let bc = init();
        assert_eq!(bc.blocks[0].to_string(), Block::genesis().to_string());
    }

    #[test]
    fn add_new_block_successfully() {
        let mut bc = init();
        let data = "this_data";
        bc.add_new_block(data);
        let last_block = bc.last_block();
        assert_eq!(last_block.data, data);
    }

    #[test]
    fn validate_a_valid_chain() {
        let bc = init();
        let mut bc2 = init();
        bc2.add_new_block("foo");
        assert_eq!(Blockchain::is_valid(&bc2.blocks), true);
    }

    #[test]
    fn invalidate_corrupted_chain() {
        let mut bc2 = init();
        bc2.add_new_block("foo");
        bc2.blocks[1].data = "not foo".to_string();
        assert_eq!(Blockchain::is_valid(&bc2.blocks), false);
    }

    #[test]
    fn replace_chain_with_valid_chain() {
        let mut bc = init();
        let mut bc2 = init();
        bc2.add_new_block("foo");
        bc.replace_chain(&bc2.blocks);
        assert_eq!(bc.is_equal(&bc2), true);
    }

    #[test]
    fn does_not_replace_with_shorter_chain() {
        let mut bc = init();
        let bc2 = init();
        bc.add_new_block("foo");
        bc.replace_chain(&bc2.blocks);
        assert_eq!(bc.is_equal(&bc2), false);
    }
}
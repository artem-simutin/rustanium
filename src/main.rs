use rustanium::{block::Block, traits::hashable::Hashable};

fn main() -> () {
    let genesis = Block::new_genesis();

    let mut another_block = Block::new(
        1,
        genesis.hash,
        0,
        0x0000ffffffffffffffffffffffffffff,
        "Another block".to_string(),
    );

    another_block.hash();
    another_block.mine();

    println!("{:?}", &another_block);
}

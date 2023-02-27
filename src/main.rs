use rustanium::{block::Block, traits::hashable::Hashable};

fn main() -> () {
    let genesis = Block::new_genesis();
    let mut another_block = Block::new(1, genesis.hash, 0, "Another block".to_string());
    let h = another_block.hash();
    another_block.hash = h;
    println!("{:?}", &another_block);
}

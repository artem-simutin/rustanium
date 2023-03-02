use rustanium::blockchain::Blockchain;

fn main() -> () {
    let mut blockchain = Blockchain::new();
    blockchain
        .start()
        .expect("Something went wrong starting the blockchain");
}

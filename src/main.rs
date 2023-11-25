use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod BlockchainHeader;

fn handle_blockchain(contents: String) {
    
} 

fn main() -> std::io::Result<()> {
    let file = File::open("Blockchain/block.chain")?;
    let mut buffer = BufReader::new(file);
    let mut contents = String::new();
    buffer.read_to_string(&mut contents)?;

    handle_blockchain(contents);

    Ok(())
}

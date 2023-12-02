use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::mem;
use std::error::Error;

//mod Blockchain;

struct BlockchainHeader {
    h_num_of_transactions : usize,
}

struct BlockchainTransaction {
    t_date : u64, // unused for now
    
}

fn handle_blockchain(contents: String) {
    let header_size = std::mem::size_of::<BlockchainHeader>();
    let transaction_size = std::mem::size_of::<BlockchainTransaction>();

    // Checar se há um header

    if contents.len() < header_size {
        println!("Header of Blockchain file not found");
        return;
    }

    unsafe {
        let raw_pointer: *const u8 = contents.as_ptr();
        let header = &*(raw_pointer as *const BlockchainHeader);

        let expected_size = header.h_num_of_transactions.checked_mul(transaction_size)
            .and_then(|ts| ts.checked_add(header_size))
            .ok_or("Size calc error").unwrap();
        
        if contents.len() < expected_size {
            println!("Number of transactions does not match the content size");
            return;
        }

        // Agora, é seguro lidar com 'contents'
    }

} 

fn main() -> std::io::Result<()> {
    let file = File::open("Blockchain/block.chain")?;
    let mut buffer = BufReader::new(file);
    let mut contents = String::new();
    buffer.read_to_string(&mut contents)?;

    handle_blockchain(contents);

    Ok(())
}

#[derive(Debug)]
struct Block {
    data: String,
    hash: String,
    prev_hash: String,
}

impl Block {
    fn new(data: String, prev_hash: String) -> Self {
        let hash = Self::hash(&data, &prev_hash);
        Block { data, hash, prev_hash }
    }

    fn genesis() -> Self {
        Block::new("Genesis block".to_string(), "".to_string())
    }

    fn hash(data: &str, prev_hash: &str) -> String {
        // Simplified hashing using data and prev_hash
        format!("{}{}", data, prev_hash)
    }
}


struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis = Block::genesis();
        Blockchain { chain: vec![genesis] }
    }

    fn add_block(&mut self, data: String) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(data, prev_hash);
        self.chain.push(new_block);
    }
}


fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Block 1 Data".to_string());
    blockchain.add_block("Block 2 Data".to_string());
    blockchain.add_block("Block 3 Data".to_string());
    blockchain.add_block("Block 4 Data".to_string());

    for block in blockchain.chain {
        println!("{:?}", block);
    }
}

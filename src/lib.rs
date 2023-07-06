use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::{
    fmt::Write,
    time::SystemTime
};

//the serialize trait lets us turn data into json-like format then into a string which lets us hash other types than strings
#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    reciever: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    difficulty: u32,
    merkle: String,
    nonce: u32,
    pre_hash: String,
    timestamp: u64,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, reciever: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction {
            sender,
            reciever,
            amount
        });
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap() //48 in utf-8 is 0 --> fill pre_hash on genesis block with 64 zeros as there is no previous block
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }
    
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            difficulty: self.difficulty,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
        };

        let reward_trans = Transaction {
            sender: String::from("root"),
            reciever: self.miner_addr.clone(),
            amount: self.reward,
        };
        
        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans); //add reward transaction for mining
        block.transactions.append(&mut self.curr_trans); //add current transactions that are cached/stored on Chain to block
        block.count = block.transactions.len() as u32; // store total number of transactions
        block.header.merkle = Chain::get_merkle(block.transactions.clone()); //calculate the transaction hash (merkle)
        Chain::proof_of_work(&mut block.header); //hash the block and reach desired difficulty

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    pub fn proof_of_work(header: &mut Blockheader)  {
        //keep rehash header until we get desired difficulty i.e. 3 or 4 zeros at beginning of hash
        //count number of hashes required via nonce --> the nonce number is also used in the hash therefore making the hash different every time nonce is incremented
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];

            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    }else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }
}

impl Chain {
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }
    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("Unable to write");
        }
        s
    }
    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle: Vec<String> = Vec::new();

        //first create a vector of hashes from the transactions
        for t in &curr_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        //rehash the merkle by combining two hashes into one
        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        
        //return the final hash left
        merkle.pop().unwrap()
    }
}
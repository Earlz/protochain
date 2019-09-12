extern crate crypto;
extern crate merkletree;

use crypto::sha2::Sha256;
use merkletree::hash::{Algorithm, Hashable};
use std::hash::Hasher;
use crypto::digest::Digest;

pub struct Sha256Algorithm(Sha256);

impl Sha256Algorithm {
    pub fn new() -> Sha256Algorithm {
        Sha256Algorithm(Sha256::new())
    }
}

impl Default for Sha256Algorithm {
    fn default() -> Sha256Algorithm {
        Sha256Algorithm::new()
    }
}

impl Hasher for Sha256Algorithm {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<[u8; 32]> for Sha256Algorithm {
    #[inline]
    fn hash(&mut self) -> [u8; 32] {
        let mut h = [0u8; 32];
        self.0.result(&mut h);
        h
    }

    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}


struct Block{
    
}
struct BlockHeaderABI{
    prevhash: [u8; 32],
    delta_root: [u8; 32],
    tx_root: [u8; 32]

}




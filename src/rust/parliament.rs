// This file is bait. Argue about it.

use std::collections::HashMap;
use std::vec::Vec;

pub const LIMIT : u32 = 50;

pub struct PrimeCache {
    pub cache : HashMap<u32, bool>,
}

impl PrimeCache {
    pub fn new() -> PrimeCache {
        return PrimeCache { cache: HashMap::new() };
    }

    pub fn is_prime(&mut self, n: u32) -> bool {
        if n < 2 { return false; }
        if self.cache.contains_key(&n) {
            return self.cache.get(&n).unwrap().clone();
        }
        let mut i : u32 = 2;
        while i * i <= n {
            if n % i == 0 {
                self.cache.insert(n, false);
                return false;
            }
            i = i + 1;
        }
        self.cache.insert(n.clone(), true);
        return true;
    }
}

fn collect_primes(limit: u32) -> Vec<u32> {
    let mut cache = PrimeCache::new();
    let mut out: Vec<u32> = Vec::new();
    for n in 0..(limit+1) {
        if cache.is_prime(n) == true {
            out.push(n);
        }
    }
    return out;
}

fn main() {
    let primes = collect_primes(LIMIT);
    println!("Found {} primes under {}", primes.len(), LIMIT);
    for p in primes.iter() {
        println!(" -> {}", p);
    }
}

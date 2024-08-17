/*
My C Open Table
Cluster count: 162
Total cluster length: 500
Largest cluster size: 6
Average cluster length: 3.09 

Your Rust Open Table
Cluster Count: 220
Total Cluster Length: 500
Largest Cluster Size: 15
Average Cluster Length: 2.27
*/

use rand::Rng;

struct HashTable {
    buckets: Vec<Option<i32>>,
}

impl HashTable {
    fn new(size: usize) -> Self {
        HashTable {
            buckets: vec![None; size],
        }
    }

    fn quadratic_probe(&self, hash: usize, i: usize) -> usize {
        (hash + i * i) % self.buckets.len()
    }

    fn insert(&mut self, key: i32) {
        let hash = key as usize % self.buckets.len();
        for i in 0..self.buckets.len() {
            let idx = self.quadratic_probe(hash, i);
            if self.buckets[idx].is_none() {
                self.buckets[idx] = Some(key);
                return;
            }
        }
    }

    fn analyze_clusters(&self) {
        let mut state = false; // false for OUT, true for IN
        let mut cluster_count = 0;
        let mut total_cluster_length = 0;
        let mut largest_cluster = 0;
        let mut current_cluster_length = 0;

        for bucket in &self.buckets {
            if bucket.is_some() { // Filled slot
                if !state {
                    state = true;
                    cluster_count += 1;
                    current_cluster_length = 1;
                } else {
                    current_cluster_length += 1;
                }
                total_cluster_length += 1;
            } else { // Empty slot
                if state {
                    state = false;
                    if current_cluster_length > largest_cluster {
                        largest_cluster = current_cluster_length;
                    }
                    current_cluster_length = 0; // Reset for next cluster
                }
            }
        }

        // Check if the last cluster is the largest
        if state && current_cluster_length > largest_cluster {
            largest_cluster = current_cluster_length;
        }

        println!("Cluster Count: {}", cluster_count);
        println!("Total Cluster Length: {}", total_cluster_length);
        println!("Largest Cluster Size: {}", largest_cluster);
        println!("Average Cluster Length: {:.2}", if cluster_count > 0 {
            total_cluster_length as f64 / cluster_count as f64
        } else {
            0.0
        });
    }
}

fn main() {
    let mut table = HashTable::new(1000);
    let mut rng = rand::thread_rng();

    for _ in 0..500 {
        let value: i32 = rng.gen_range(0..10000);
        table.insert(value);
    }

    table.analyze_clusters();
}

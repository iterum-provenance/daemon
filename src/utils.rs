use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub fn create_random_hash() -> String {
    let mut rng = thread_rng();
    let hash: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(32)
        .collect();

    hash
}

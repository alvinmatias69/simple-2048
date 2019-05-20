use rand::Rng;

pub fn between(start: u32, end: u32) -> u32 {
    let mut rng = rand::thread_rng();

    let result = rng.gen_range(start, end);
    result
}

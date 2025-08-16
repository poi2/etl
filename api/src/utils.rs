use rand::distr::{Distribution, slice::Choose};

/// Generates a random alphabetic string of length `len`
pub fn generate_random_alpha_str(len: usize) -> String {
    let chars = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let chars_dist = Choose::new(&chars).expect("passed choose was empty");

    chars_dist.sample_iter(&mut rand::rng()).take(len).collect()
}

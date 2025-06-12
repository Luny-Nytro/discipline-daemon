use rand::{distr::Uniform, Rng};

pub fn random_lowercase_string(length: usize) -> String {
  let mut rng = rand::rng();
  let letters = Uniform::new_inclusive(b'a', b'z').unwrap(); // ASCII range for lowercase letters

  (0..length)
    .map(|_| rng.sample(&letters) as char)
    .collect()
}
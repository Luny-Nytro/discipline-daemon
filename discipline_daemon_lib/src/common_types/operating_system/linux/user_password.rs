
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPassword {
  password: String,
}

impl UserPassword {
  pub fn new(password: String) -> Option<UserPassword> {
    if (1..=72).contains(&password.len())  {
      Some(Self { password })
    } else {
      None
    }
  }

  pub fn as_ref(&self) -> &String {
    &self.password
  }

  pub fn generate_random_password() -> UserPassword {
    use rand::{distr::Uniform, Rng};

    let mut rng = rand::rng();
    let letters = Uniform::new_inclusive(b'a', b'z').unwrap(); // ASCII range for lowercase letters
  
    let password = (0..10)
      .map(|_| rng.sample(&letters) as char)
      .collect();

    UserPassword { 
      password
    }
  }
}
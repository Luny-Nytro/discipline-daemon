/// When the user atemps to access a website, the delay blocks the website
/// for the number of milliseconds specified in `blockFor`, then allows 
/// accessing the website for the number of milliseconds specified in
/// `allowFor`
pub struct Feature {
  pub from: DateTime<Utc>,
  pub block_for: Duration,
  pub allow_for: Duration,
}
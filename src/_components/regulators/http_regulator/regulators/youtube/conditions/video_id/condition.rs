use uuid::Uuid;

pub struct Condition {
  id: Uuid,
  key: String,
  path: Vec<String>,
  owner: usize,
  videoId: String,
}

// struct VideoIdCondition {
//   id: String,
// }
// struct ChannelIdCondition {
//   id: String,
// }
// struct VideoCategoryCondition {

// }
// struct ChannelCategoryCondition {

// }
// struct VideoNameCondition {

// }
// struct ChannelNameCondition {

// }
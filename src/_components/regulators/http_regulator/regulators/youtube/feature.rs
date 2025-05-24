use std::rc::Rc;
use serde::{Serialize, Deserialize};
use crate::rules::rule::Rule;


#[derive(Debug, PartialEq, Eq, Hash)]
enum Category {
  Music = "Music",
  Gaming = "Gaming",
  Sports = "Sports",
  Comedy = "Comedy",
  Education = "Education",
  Entertainment = "Entertainment",
  HowToAndStyle ="Howto & Style",
  NewsAndPolitics = "News & Politics",
  PetsAndAnimals = "Pets & Animals",
  PeopleAndBlogs = "People & Blogs",
  TravelAndEvents = "Travel & Events",
  FilmAndAnimation = "Film & Animation",
  AutosAndVehicles = "Autos & Vehicles",
  ScienceAndTechnology ="Science & Technology",
  NonprofitsAndActivism = "Nonprofits & Activism",
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CategoryID {
  Music = 10,
  Gaming = 20,
  Sports = 17,
  Comedy = 23,
  Education = 27,
  Entertainment = 24,
  HowToAndStyle = 26,
  NewsAndPolitics = 25,
  PetsAndAnimals = 15,
  PeopleAndBlogs = 22,
  TravelAndEvents = 19,
  FilmAndAnimation = 1,
  AutosAndVehicles = 2,
  ScienceAndTechnology = 28,
  NonprofitsAndActivism = 29,
}

impl CategoryID {
  pub fn to_category_name(&self) -> Category {
    match self {
      // CategoryID::
    }
  }
}

enum ChannelCategory {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
  pub block_sensitive_content: bool,
  pub blocked_videos: Vec<usize>,
  pub blocked_channels_by_ids: Vec<String>,
  pub blocked_channels_by_names: Vec<String>,
  pub blocked_channels_by_category: Vec<CategoryID>,
}

impl Feature {
}
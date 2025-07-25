use serde::{Serialize, Deserialize};

use crate::{
  user_screen_access_regulation, OperatingSystemUserPassword, 
  OperatingSystemUserId, OperatingSystemUserName, 
  IntoPublic, Uuid
};

use super::{
  UserName, User
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublicRepr {
  id: Uuid,
  name: UserName,
  operating_system_user_id: OperatingSystemUserId,
  operating_system_user_name: OperatingSystemUserName,
  operating_system_user_password: OperatingSystemUserPassword,
  screen_access_regulator: user_screen_access_regulation::RegulationPublicRepr,
}

impl IntoPublic for User {
  type Output = UserPublicRepr;

  fn into_public(self) -> Self::Output {
    UserPublicRepr {
      id: self.id,
      name: self.name,
      operating_system_user_id: self.operating_system_user_id,
      operating_system_user_name: self.operating_system_user_name,
      operating_system_user_password: self.operating_system_user_password,
      screen_access_regulator: self.screen_access_regulation.into_public(),
    }
  }
}
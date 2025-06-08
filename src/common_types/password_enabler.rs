use crate::Password;

#[derive(Debug, Clone)]
pub struct PasswordEnabler {
  password: Password,
  is_effective: bool,
}

impl PasswordEnabler {
  pub fn new(password: Password, is_effective: bool) -> Self {
    Self {
      password,
      is_effective,
    }
  }

  pub fn is_right_password(&self, password: &Password) -> bool {
    *password == self.password
  }

  pub fn is_effective(&self) -> bool {
    self.is_effective
  }

  pub fn make_effective(&mut self) {
    self.is_effective = true;
  }

  pub fn make_ineffective(&mut self) {
    self.is_effective = false;
  }

  pub fn change_password(&mut self, new_password: Password) {
    self.password = new_password;
  }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PasswordAuthenticationStatusCreator {
//   password: String,
// }

// impl PasswordAuthenticationStatusCreator {
//   pub fn create(self) -> PasswordAuthenticationStatus {
//     PasswordAuthenticationStatus {
//       is_locked: false,
//       password: self.password,
//     }
//   }
// }

pub mod public_repr {
  use serde::{Deserialize, Serialize};
  use crate::ToPublicRepr;
  use super::PasswordEnabler;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct PublicRepr {
    is_effective: bool
  }

  impl ToPublicRepr for PasswordEnabler {
    type PublicRepr = PublicRepr;

    fn to_public_repr(&mut self) -> Self::PublicRepr {
      PublicRepr {
        is_effective: self.is_effective
      }
    }
  }
}

pub mod database_serde {
  use crate::database::*;
  use super::PasswordEnabler;
  use crate::GenericError;

  pub struct Adapter {
    password: BasicColumnInfo,
    is_effective: BasicColumnInfo,
  }

  impl Adapter {
    pub fn new(column_names_prefix: &str) -> Self {
      Self {
        password: BasicColumnInfo::new(format!("{column_names_prefix}_password")),
        is_effective: BasicColumnInfo::new(format!("{column_names_prefix}_is_effective")),
      }
    }

    pub fn new_ordered(column_names_prefix: &str) -> Self {
      Self {
        password: BasicColumnInfo::new(format!("{column_names_prefix}_0")),
        is_effective: BasicColumnInfo::new(format!("{column_names_prefix}_1")),
      }
    }

    pub fn columns(&self) -> impl Iterator<Item = &impl IsColumnInfo> {
      [&self.password, &self.is_effective].into_iter()
    }
  }


  impl CompoundTypeSerializer for Adapter {
    type CompoundValue = PasswordEnabler;

    fn serialize_into(
      &self, 
      value: &Self::CompoundValue,
      context: &mut CompoundValueSerializerContext, 
    ) {
      context.scalar(&self.password, &value.password);
      context.scalar(&self.is_effective, &value.is_effective);
    }
  }

  impl CompoundValueDeserializer for Adapter {
    type Output = PasswordEnabler;

    fn deserialize(&self, context: &DeserializeContext) -> Result<Self::Output, GenericError> {
      Ok(PasswordEnabler {
        password: context.scalar(&self.password).map_err(|error|
          error.change_context("Failed to deserialize PasswordEnabler: Failed to deserialize 'password' field")
        )?,
        is_effective: context.scalar(&self.is_effective).map_err(|error|
          error.change_context("Failed to deserialize PasswordEnabler: Failed to deserialize 'is_effective' field")
        )?
      })
    }
  }
}
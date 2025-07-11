// pub mod database {
//   use crate::database::*;
//   use crate::{GenericError, Uuid};

//   impl IntoScalarValue for Uuid {
//     fn into_scalar_value(&self) -> impl IsScalarValue {
//       self.to_string()
//     }
//   }

//   impl FromScalarValue for Uuid {
//     fn deserialize(value: ScalarValue) -> Result<Self, crate::GenericError> {
//       value
//         .as_string()
//         .map_err(|error|
//           error.change_context("deserializing a Uuid")
//         )
//         .and_then(|string|
//           Uuid::try_parse(&string).map_err(|error|
//             GenericError::new("creating a Uuid from a string")
//               .add_attachment("string", string)
//               .add_attachment("error", error.to_string())
//               .change_context("deserializing a Uuid")
//           )
//         )
//     }
//   }
// }
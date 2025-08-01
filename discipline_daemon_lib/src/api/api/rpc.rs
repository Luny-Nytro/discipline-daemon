use std::{marker::PhantomData, sync::Arc};
use serde::{de::DeserializeOwned, Serialize};

use crate::Daemon;

// let Some(operation) = deserialize_operation(serialized_operation) else {
//   return GenericOperationReturn::malformed_operation();
// };

// let Some(operation_retrun) = operation.execute(daemon) else {
//   return GenericOperationReturn::internal_error();
// };

// let Some(serialized_operation_return) = serialize_operation_return(operation_retrun) else {
//   return GenericOperationReturn::internal_error();
// };


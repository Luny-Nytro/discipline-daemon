use super::*;

pub fn serialize_policy_enabler(
  context: &mut SerializeCompoundValueContext,
  value: &PolicyEnabler,
  timer_duration_field: &String,
  timer_remaining_duration_field: &String,
  timer_previous_synchronization_time_field: &String,
) {
  serialize_countdown_timer(
    context, 
    value.unpack_ref(), 
    timer_duration_field, 
    timer_remaining_duration_field, 
    timer_previous_synchronization_time_field,
  );
}

pub fn deserialize_policy_enabler(
  context: &mut DeserializeCompoundValueContext,
  timer_duration_field: &String,
  timer_remaining_duration_field: &String,
  timer_previous_synchronization_time_field: &String,
)
  -> Result<PolicyEnabler, GenericError>
{
  Ok(PolicyEnabler::pack(
    deserialize_countdown_timer(
      context, 
      timer_duration_field, 
      timer_remaining_duration_field, 
      timer_previous_synchronization_time_field,
    )?
  ))
}

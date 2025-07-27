import { isSome, None, Option, Some, Serde } from "../Prelude.ts";

export function intoIR<T>(
  me: Option<T>, 
  fn: (ir: T) => Serde.IntermediateRepr,
): Serde.IntermediateRepr 
{
  return isSome(me) 
    ? fn(me.value) 
    : null
}

export function fromIR<T>(
  ir: Serde.IntermediateRepr, 
  fn: (ir: Serde.IntermediateRepr) => Option<T>,
): Option<Option<T>> 
{
  if (ir === null) {
    return Some(None())
  } else {
    return Option.then1(fn(ir), Some)
  }
}
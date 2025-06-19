import { isSome, None, Option, Some } from "../Prelude.ts";
import { IntermediateRepr } from "./mod.ts";

export function FromIR<Item>(itemFromIR: (ir: IntermediateRepr) => Option<Item>) {
  return (ir: IntermediateRepr) => fromIR(ir, itemFromIR)
}

export function fromIR<Item>(
  ir: IntermediateRepr, 
  itemFromIR: (ir: IntermediateRepr) => Option<Item>,
): Option<Item[]> 
{  
  if (!Array.isArray(ir)) {
    return None()
  }

  const array: Item[] = []

  for (const itemIntermediateRepr of ir) {
    const item = itemFromIR(itemIntermediateRepr)
    if (isSome(item)) {
      array.push(item.value)
    } else {
      return None()
    }
  }

  return Some(array)
}

export function intoIR<T>(me: T[], itemIntoIR: (item: T) => IntermediateRepr): IntermediateRepr {
  return me.map(itemIntoIR)
}
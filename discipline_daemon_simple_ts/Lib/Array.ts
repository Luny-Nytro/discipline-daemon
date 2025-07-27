import { None, Option, Some } from "../Prelude.ts";

export const Array = {
  findIndex<T>(array: T[], predicate: (item: T) => boolean): Option<number> {
    const index = array.findIndex(predicate)
    if (index === -1) {
      return None()
    } else {
      return Some(index)
    }
  }
}
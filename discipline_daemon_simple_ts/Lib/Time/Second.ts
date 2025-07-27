import { isIntegerAndBetween, None, Option, Some } from "../../Prelude.ts";

export type Second = number & { readonly ___type: "Time.Second" }

const minValue = 0
const maxValue = 59

export const Second = {
  /**
   * @param number Valid range is 0 .. 59
   */
  new(number: number): Option<Second> {
    if (isIntegerAndBetween(number, minValue, maxValue)) {
      return Some(number as Second)
    } else {
      return None()
    }
  },

  /**
   * Unsafe.
   * 
   * @param number Valid range is 0 .. 59
   */
  newUnchecked(number: number): Second {
    return number as Second
  },

  /**
   * @returns The second as a number in the range 0 .. 59
   */
  value(me: Second): number {
    return me
  },

  eq(a: Second, b: Second) {
    return a === b
  },

  gth(a: Second, b: Second) {
    return a > b
  },

  gte(a: Second, b: Second) {
    return a >= b
  },

  lth(a: Second, b: Second) {
    return a < b
  },

  lte(a: Second, b: Second) {
    return a <= b
  },
}
import { isIntegerAndBetween, None, Option, Some } from "../../Prelude.ts";

export type Minute = number & { readonly __type: "Time.Minute" }

const minValue = 0
const maxValue = 59

export const Minute = {
  /**
   * @param number Valid range is 0 .. 59
   */
  new(number: number): Option<Minute> {
    if (isIntegerAndBetween(number, minValue, maxValue)) {
      return Some(number as Minute)
    } else {
      return None()
    }
  },

  /**
   * @param number Valid range is 0 .. 59
   */
  newUnchecked(number: number): Minute {
    return number as Minute
  },

  /**
   * @returns The minute as a number in the range 0 .. 59
   */
  value(me: Minute): number {
    return me
  },

  eq(a: Minute, b: Minute) {
    return a === b
  },

  gth(a: Minute, b: Minute) {
    return a > b
  },

  gte(a: Minute, b: Minute) {
    return a >= b
  },

  lth(a: Minute, b: Minute) {
    return a < b
  },

  lte(a: Minute, b: Minute) {
    return a <= b
  },
}
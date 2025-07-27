import { isIntegerAndBetween, None, Option, Some } from "../../Prelude.ts";

export type MonthDay = number & { readonly __type: "Time.MonthDay" }

const minValue = 1
const maxValue = 31

export const MonthDay = {
  /**
   * @param number Valid range is 1 .. 31
   */
  new(number: number): Option<MonthDay> {
    return isIntegerAndBetween(number, minValue, maxValue)
      ? Some(number as MonthDay)
      : None()
  },

  /**
   * Unsafe.
   * 
   * @param number Valid range is 1 .. 31 
   */
  newUnchecked(number: number): MonthDay {
    return number as MonthDay
  },

  /**
   * @returns The day of month as a number in the range 1 .. 31
   */
  value(me: MonthDay): number {
    return me
  },

  eq(a: MonthDay, b: MonthDay) {
    return a === b
  },

  gth(a: MonthDay, b: MonthDay) {
    return a > b
  },

  gte(a: MonthDay, b: MonthDay) {
    return a >= b
  },

  lth(a: MonthDay, b: MonthDay) {
    return a < b
  },

  lte(a: MonthDay, b: MonthDay) {
    return a <= b
  },
}
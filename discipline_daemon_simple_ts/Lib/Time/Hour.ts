import { isIntegerAndBetween, None, Option, Some } from "../../Prelude.ts";

export type Hour = number & { readonly __type: "Time.Hour" }

const minValue = 0
const maxValue = 23

export const Hour = {
  /**
   * @param number Valid range is 0 .. 23
   */
  new(number: number): Option<Hour> {
    if (isIntegerAndBetween(number, minValue, maxValue)) {
      return Some(number as Hour)
    } else {
      return None()
    }
  },

  /**
   * @param number Valid range is 0 .. 11
   */
  newAM(number: number): Option<Hour>  {
    if (isIntegerAndBetween(number, 0, 11)) {
      return Some(number as Hour)
    } else {
      return None()
    }
  },

  /**
   * @param number Valid range is 0 .. 11 
   */
  newPM(number: number): Option<Hour> {
    if (isIntegerAndBetween(number, 0, 11)) {
      return Some(number + 12 as Hour)
    } else {
      return None()
    }
  },

  /**
   * Unsafe.
   * 
   * @param number Valid range is 0 .. 23
   */
  newUnchecked(number: number): Hour {
    return number as Hour
  },

  /**
   * @returns The hour as a number in the range 0 .. 23
   */
  value(me: Hour): number {
    return me
  },

  eq(a: Hour, b: Hour) {
    return a === b
  },

  gth(a: Hour, b: Hour) {
    return a > b
  },

  gte(a: Hour, b: Hour) {
    return a >= b
  },

  lth(a: Hour, b: Hour) {
    return a < b
  },

  lte(a: Hour, b: Hour) {
    return a <= b
  },

  period(me: Hour): "AM" | "PM" {
    return me < 12 ? "AM" : "PM"
  },

  value12(me: Hour): number {
    return me < 12 ? me : me - 12
  },

  to12BasedValueWithPeriod(me: Hour): { hour: number, period: "AM" | "PM" } {
    if (me < 12) {
      return {
        hour: me,
        period: "AM",
      }
    } else {
      return {
        hour: me - 12,
        period: "PM",
      }
    }
  }
}
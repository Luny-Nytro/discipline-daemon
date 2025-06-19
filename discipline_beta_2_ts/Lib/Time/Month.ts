import { isIntegerAndBetween, None, Option, Some } from "../../Prelude.ts";

export type January = 0 & { readonly __type: "Time.Month.January" }
export type February = 1 & { readonly __type: "Time.Month.February" }
export type March = 2 & { readonly __type: "Time.Month.March" }
export type April = 3 & { readonly __type: "Time.Month.April" }
export type May = 4 & { readonly __type: "Time.Month.May" }
export type June = 5 & { readonly __type: "Time.Month.June" }
export type July = 6 & { readonly __type: "Time.Month.July" }
export type August = 7 & { readonly __type: "Time.Month.August" }
export type September = 8 & { readonly __type: "Time.Month.September" }
export type October = 9 & { readonly __type: "Time.Month.October" }
export type November = 10 & { readonly __type: "Time.Month.November" }
export type December = 11 & { readonly __type: "Time.Month.December" }

export const January = 0 as January
export const February = 1 as February
export const March = 2 as March
export const April = 3 as April
export const May = 4 as May
export const June = 5 as June
export const July = 6 as July
export const August = 7 as August
export const September = 8 as September
export const October = 9 as October
export const November = 10 as November
export const December = 11 as December

export type Month = (
  | January
  | February
  | March
  | April
  | May
  | June
  | July
  | August
  | September
  | October
  | November
  | December
)

const minValue = 1
const maxValue = 12

export const Month = {
  /**
   * @param number The month as a number in the range 1 .. 12
   */
  new(number: number): Option<Month> {
    return isIntegerAndBetween(number, minValue, maxValue)
      ? Some(number as Month)
      : None()
  },

  /**
   * Unsafe.
   * 
   * @param number The month as a number in the range 1 .. 12
   */
  newUnchecked(number: number): Month {
    return number as Month
  },

  /**
   * @returns The month as a number in the range 1 .. 12
   */
  value(me: Month): number {
    return me
  },
}
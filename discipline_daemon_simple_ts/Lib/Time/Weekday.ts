import { isIntegerAndBetween, None, Option, Some } from "../../Prelude.ts";

export type Sunday = 0 & { readonly ____type: "Time.Weekday.Sunday" }
export type Monday = 1 & { readonly ____type: "Time.Weekday.Monday" }
export type Tuseday = 2 & { readonly ____type: "Time.Weekday.Tuseday" }
export type Wednesday = 3 & { readonly ____type: "Time.Weekday.Wednesday" }
export type Thursday = 4 & { readonly ____type: "Time.Weekday.Thursday" }
export type Friday = 5 & { readonly ____type: "Time.Weekday.Friday" }
export type Saturday = 6 & { readonly ____type: "Time.Weekday.Saturday" }

export const Sunday = 0 as Sunday
export const Monday = 1 as Monday
export const Tuseday = 2 as Tuseday
export const Wednesday = 3 as Wednesday
export const Thursday = 4 as Thursday
export const Friday = 5 as Friday
export const Saturday = 6 as Saturday

export type Weekday = (
  | typeof Sunday
  | typeof Monday
  | typeof Tuseday
  | typeof Wednesday
  | typeof Thursday
  | typeof Friday
  | typeof Saturday 
)

export const Weekday = {
  uncheckedFrom0To6(number: number): Weekday {
    return number as Weekday
  },
  
  uncheckedFrom1To7(number: number): Weekday {
    return number - 1 as Weekday
  },

  from0To6(number: number): Option<Weekday> {
    if (isIntegerAndBetween(number, 0, 6)) {
      return Some(number as Weekday)
    } else {
      return None()
    }
  },

  from1To7(number: number): Option<Weekday> {
    if (isIntegerAndBetween(number, 1, 7)) {
      return Some(number - 1 as Weekday)
    } else {
      return None()
    }
  },

  fromFullCapitalizedName(name: string): Option<Weekday> {
    switch (name) {
      case "Sunday": return Some(Sunday)
      case "Monday": return Some(Monday)
      case "Tuseday": return Some(Tuseday)
      case "Wednesday": return Some(Wednesday)
      case "Thursday": return Some(Thursday)
      case "Friday": return Some(Friday)
      case "Saturday": return Some(Saturday) 
      default: return None()
    }
  },

  value0To6(me: Weekday): number {
    return me
  },

  value1To7(me: Weekday): number {
    return me + 1
  },

  eq(a: Weekday, b: Weekday) {
    return a === b
  },

  gth(a: Weekday, b: Weekday) {
    return a > b
  },

  gte(a: Weekday, b: Weekday) {
    return a >= b
  },

  lth(a: Weekday, b: Weekday) {
    return a < b
  },

  lte(a: Weekday, b: Weekday) {
    return a <= b
  },
}
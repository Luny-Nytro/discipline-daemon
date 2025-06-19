import * as Weekday from "./Weekday.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type T = Unique<"Discipline.Chronic.Weekday.CreateFromString", {
  readonly name: string
}>

export interface Cases<A> {
  readonly UnknownWeekdayName: (name: string) => A
}

export function UnknownWeekdayName(name: string): T {
  return Unique({
    name: name
  })
}

export function match<A>(
  me: T,
  cases: Cases<A>
) {
  return cases.UnknownWeekdayName(me.name)
}

export const displayer = Displayer.implement<T>(me => 
  Displayer.asWrappedString(`Weekday.CreateFromStringError`, `Could not create a weekday from string. Expected string to be "${
    Weekday.SUNDAY_AS_STRING
  }", "${
    Weekday.MONDAY_AS_STRING
  }", "${
    Weekday.TUSEDAY_AS_STRING
  }", "${
    Weekday.WEDNESDAY_AS_STRING
  }", "${
    Weekday.THURSDAY_AS_STRING
  }", "${
    Weekday.FRIDAY_AS_STRING
  }", or "${
    Weekday.SATURDAY_AS_STRING
  }", but found: "${me.name}"`)
)
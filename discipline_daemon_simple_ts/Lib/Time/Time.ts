import { Option, None, Some, Hour, Minute, Second, isIntegerAndBetween } from "../../Prelude.ts";
import { Units } from "./mod.ts";

export type Time = number & { readonly __type: "Time.Time" }

const minValue = 0
const maxValue = Units.millisecondsPerHour * 24

export const Time = {
  fromTimestamp(timestamp: number): Option<Time> {
    if (isIntegerAndBetween(timestamp, minValue, maxValue)) {
      return Some(timestamp as Time)
    } else {
      return None()
    }
  },
  
  fromH(hour: Hour): Time {
    return Hour.value(hour) * Units.millisecondsPerHour as Time
  },

  newHM(hour: Hour, minute: Minute): Time {
    return Hour.value(hour) * Units.millisecondsPerHour
      + Minute.value(minute) * Units.millisecondsPerMinute as Time
  },
  
  fromHMS(hour: Hour, minute: Minute, second: Second): Time {
    return Hour.value(hour) * Units.millisecondsPerHour
      + Minute.value(minute) * Units.millisecondsPerMinute
      + Second.value(second) * Units.millisecondsPerSecond as Time
  },

  hour(me: Time): Hour {
    // SAFETY: Because `me` is not larger than `maxValue`, this operation
    // is safe.
    return Hour.newUnchecked(Math.floor(me / Units.millisecondsPerHour)) 
  },

  minute(me: Time): Minute {
    // SAFETY: Because `me` is not larger than `maxValue`, this operation
    // is safe.
    return Minute.newUnchecked(Math.floor(me % Units.millisecondsPerHour / Units.millisecondsPerMinute))
  },

  second(me: Time): Second {
    // SAFETY: Because `me` is not larger than `maxValue`, this operation
    // is safe.
    return Second.newUnchecked(Math.floor(me % Units.millisecondsPerHour % Units.millisecondsPerMinute / Units.millisecondsPerSecond))
  },

  asTimestamp(me: Time): number {
    return me
  },
  
  eq(lhs: Time, rhs: Time): boolean {
    return lhs === rhs
  },

  lte(lhs: Time, rhs: Time): boolean {
    return lhs <= rhs
  },

  lth(lhs: Time, rhs: Time): boolean {
    return lhs < rhs
  },

  gte(lhs: Time, rhs: Time): boolean {
    return lhs >= rhs
  },

  gth(lhs: Time, rhs: Time): boolean {
    return lhs > rhs
  },

  toString(me: Time, { hour12, second }: ToStringOptions = {}) {
    let output = ""

    if (hour12) {
      output += Hour.value12(Time.hour(me)).toString().padStart(2, "0")
    } else {
      output += Hour.value(Time.hour(me)).toString().padStart(2, "0")
    }

    output += ":"
    output += Minute.value(Time.minute(me)).toString().padStart(2, "0")

    if (second) {
      output += ":"
      output += Second.value(Time.second(me)).toString().padStart(2, "0")
    }
    
    if (hour12) {
      output += " "
      output += Hour.period(Time.hour(me))
    }

    return output
  },
}

export interface ToStringOptions {
  second?: boolean
  hour12?: boolean
}
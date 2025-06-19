import { None, Option, Some } from "../../Prelude.ts";
import { Units } from "./mod.ts"

export type Duration = number & { readonly __type: "Time.Duration" }

export const Duration = {
  newZero(): Duration {
    return 0 as Duration
  },

  newOneMinute(): Duration {
    return Units.millisecondsPerMinute as Duration
  },

  new2Minutes(): Duration {
    return Units.millisecondsPerMinute * 2 as Duration
  },

  newWeek(): Duration {
    return Units.millisecondsPerWeek as Duration
  },
  
  uncheckedFromMilliseconds(milliseconds: number): Duration {
    if (Number.isSafeInteger(milliseconds) && milliseconds >= 0) {
      return milliseconds as Duration
    }
    throw new Error("Invaid argument: " + milliseconds.toString())
  },

  fromMilliseconds(milliseconds: number): Option<Duration> {
    if (Number.isSafeInteger(milliseconds) && milliseconds >= 0) {
      return Some(milliseconds as Duration)
    } else {
      return None()
    }
  },

  fromSeconds(seconds: number): Option<Duration> {
    if (Number.isSafeInteger(seconds) && seconds >= 0) {
      const millis = seconds * Units.millisecondsPerSecond
      if (millis === Infinity) {
        return None()
      } else {
        return Some(millis as Duration)
      }
    }

    return None()
  },

  fromMinutes(minutes: number): Option<Duration> {
    if (Number.isSafeInteger(minutes) && minutes >= 0) {
      const millis = minutes * Units.millisecondsPerMinute
      if (millis === Infinity) {
        return None()
      } else {
        return Some(millis as Duration)
      }
    }

    return None()
  },

  fromHours(hours: number): Option<Duration> {
    if (Number.isSafeInteger(hours) && hours >= 0) {
      const millis = hours * Units.millisecondsPerHour
      if (millis === Infinity) {
        return None()
      } else {
        return Some(millis as Duration)
      }
    }

    return None()
  },
  
  fromDays(days: number): Option<Duration> {
    if (Number.isSafeInteger(days) && days >= 0) {
      const millis = days * Units.millisecondsPerDay
      if (millis === Infinity) {
        return None()
      } else {
        return Some(millis as Duration)
      }
    }

    return None()
  },
  
  fromWeeks(weeks: number): Option<Duration> {
    if (Number.isSafeInteger(weeks) && weeks >= 0) {
      const millis = weeks * Units.millisecondsPerWeek
      if (millis === Infinity) {
        return None()
      } else {
        return Some(millis as Duration)
      }
    }

    return None()
  },

  milliseconds(me: Duration): number {
    return me
  },

  seconds(me: Duration): number {
    return Units.millisecondsToSeconds(me)
  },

  minutes(me: Duration): number {
    return Units.millisecondsToMinutes(me)
  },

  hours(me: Duration): number {
    return Units.millisecondsToHours(me)
  },

  days(me: Duration): number {
    return Units.millisecondsToDays(me)
  },

  weeks(me: Duration): number {
    return Units.millisecondsToWeeks(me)
  },

  eq(me: Duration, other: Duration) {
    return me === other
  },

  gth(a: Duration, b: Duration) {
    return a > b
  },

  gte(a: Duration, b: Duration) {
    return a >= b
  },

  lth(a: Duration, b: Duration) {
    return a < b
  },

  lte(a: Duration, b: Duration) {
    return a <= b
  },

  isZero(me: Duration) {
    return me === 0
  },

  clone(me: Duration): Duration {
    return me
  },

  sub(lhs: Duration, rhs: Duration): Option<Duration> {
    if (lhs >= rhs) {
      return Some(lhs - rhs as Duration)
    } else {
      return None()
    }
  },

  subOrZero(lhs: Duration, rhs: Duration) {
    if (lhs <= rhs) {
      return Duration.newZero()
    } else {
      return lhs - rhs as Duration
    }
  },

  add(lhs: Duration, rhs: Duration): Option<Duration> {
    const result = lhs + rhs
    if (Number.isSafeInteger(result) && result >= 0) {
      return Some(Duration.uncheckedFromMilliseconds(result))
    } else {
      return None()
    }
  },

  toPrettyFormat(me: Duration): string {
    let milliseconds = Duration.milliseconds(me)

    const days = Math.floor(milliseconds / Units.millisecondsPerDay);
    milliseconds %= Units.millisecondsPerDay;

    const hours = Math.floor(milliseconds / Units.millisecondsPerHour);
    milliseconds %= Units.millisecondsPerHour;

    const minutes = Math.floor(milliseconds / Units.millisecondsPerMinute);
    milliseconds %= Units.millisecondsPerMinute;

    const seconds = Math.floor(milliseconds / Units.millisecondsPerSecond);

    let string = ""
    function push(value: string) {
      if (string.length) {
        string += " "
      }
      string += value
    }

    if (days > 0) {
      push(`${days}D`);
    }
    if (hours > 0) {
      push(`${hours}H`);
    }
    if (minutes > 0) {
      push(`${minutes}M`);
    }
    if (seconds > 0) {
      push(`${seconds}S`);
    }

    return string.length > 0 ? string : 'Zero';
  },

  min(a: Duration, b: Duration): Duration {
    return a < b ? a : b
  }
}

export type MaybeDuration = Option<Duration>

export const MaybeDuration = {
  eq(a: MaybeDuration, b: MaybeDuration) {
    return Option.eq(a, b, Duration.eq)
  }
}
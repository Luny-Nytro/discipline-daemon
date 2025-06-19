import { TimeToStringOptions } from "../../Prelude.ts";
import { Time } from "./Time.ts";

export interface TimeRange {
  readonly min: Time
  readonly max: Time
}

export const TimeRange = {
  new(min: Time, max: Time): TimeRange {
    return {
      min,
      max,
    }
  },

  contains(me: TimeRange, time: Time): boolean {
    if (Time.lte(me.min, me.max)) {
      // TimeRange: 07:00:00 AM ... 07:00:00 PM
      // Time: 08:00:00 AM
      return Time.gte(time, me.min) && Time.lte(time, me.max)
    } else {
      // TimeRange: 07:00:00 PM ... 07:00:00 AM
      // Time: 06:06:28 PM
      return Time.gte(time, me.min) || Time.lte(time, me.max)
    }
  },

  toString(me: TimeRange, options: TimeToStringOptions): string {
    return `${
      Time.toString(me.min, options).toString()
    } ... ${
      Time.toString(me.max, options).toString()
    }`
  },

  withMin(me: TimeRange, newMin: Time): TimeRange {
    return {
      min: newMin,
      max: me.max,
    }
  },

  withMax(me: TimeRange, newMax: Time): TimeRange {
    return {
      min: me.min,
      max: newMax,
    }
  },
}
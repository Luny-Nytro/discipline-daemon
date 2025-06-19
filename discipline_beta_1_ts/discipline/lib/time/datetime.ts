import { Option, Tried, Ok, Err, Some, None, Minute, Second, Hour, Duration, Weekday, Time } from "Pkg"

function changeTimeZoneOffset(date: Date, newTimeZoneOffset: TimeZoneOffset) {
  // Local time: UTC time plus some time zone offset.
  let time = date.getTime()

  // Remove the current time zone offset.
  time -= TimeZoneOffset.fromDate(date).toMilliseconds()

  // Add the new time zone offset
  time += newTimeZoneOffset.toMilliseconds()

  return new Date(time)
}

/**
 * A TimeZoneOffset is a positive or negative integer representing an offset 
 * from UTC in minutes.
 * 
 * Add this to UTC time to get the local time.
 * 
 * Subtract this from local time to get UTC time. 
 */
class TimeZoneOffset {
  constructor(
    /** An integer representing a number of minutes */
    readonly value: number,
  ) {}

  toMilliseconds() {
    return this.value * 60 * 1000
  }

  static fromDate(date: Date) {
    // "date.getTimezoneOffset" is reversed: instead of returning the number of minutes
    // to add to UTC time to get the local time, it returns the number of minutes
    // to add to local time to get UTC time.
    //
    // To get the result we want, we must reverse the sign of the number returned by 
    // "date.getTimezoneOffset".
    //
    // A number's sign is reversed by multipling it by "-1".
    return new TimeZoneOffset(date.getTimezoneOffset() * -1)  
  }
}

export class UTCTimeZone {
  readonly offset = new TimeZoneOffset(0)
}

export class BaghdadTimeZone {
  readonly offset = new TimeZoneOffset(60 * 3)
}

export const UTC_TIME_ZONE = new UTCTimeZone()
export const BAGHDAD_TIME_ZONE = new BaghdadTimeZone()

export type TimeZone = (
  | UTCTimeZone
  | BaghdadTimeZone
)


export class DateTime {
  static readonly MIN_TIMESTAMP = 0
  static readonly MAX_EPOCH_MILLISECONDS = Number.MAX_SAFE_INTEGER

  private constructor(private readonly value: Date) {}
  
  eq(other: DateTime): boolean {
    return this.value.getTime() === other.value.getTime()
  }

  second() {
    return Second.new1Unchecked(this.value.getSeconds())
  }

  changeSecond(second: Second) {
    return this.value.setSeconds(second.value)
  }

  minute() {
    return Minute.new1Unchecked(this.value.getMinutes())
  }

  setMinute(minute: Minute) {
    return this.value.setMinutes(minute.value)
  }

  hour() {
    return Hour.new1_24Unchecked(this.value.getHours())
  }

  setHour(hour: Hour) {
    return this.value.setHours(hour.value)
  }

  weekday(): Weekday {
    return Weekday.fromNumber0(this.value.getDay()).unwrap()
  }

  // setWeekday(newWeekday: Weekday) {
  //   throw new Error("Not Implemented")
  //   // const currWeekday = this.weekday();
  //   // const difference = newWeekday - currWeekday
  //   // this.inner.setDate(this.inner.getUTCDate() + difference)
  // }

  epochMilliseconds() {
    return this.value.getTime()
  }

  clone(): DateTime {
    return new DateTime(new Date(this.epochMilliseconds()))
  }

  isBefore(other: DateTime): boolean {
    return this.epochMilliseconds() < other.epochMilliseconds()
  }

  isBeforeOrAt(other: DateTime): boolean {
    return this.epochMilliseconds() <= other.epochMilliseconds()
  }

  isAfter(other: DateTime): boolean {
    return this.epochMilliseconds() > other.epochMilliseconds()
  }

  isAfterOrAt(other: DateTime) {
    return this.epochMilliseconds() >= other.epochMilliseconds()
  }

  plus(duration: Duration): DateTime {
    return DateTime.fromEpochMilliseconds(this.epochMilliseconds() + duration.milliseconds())
  }

  minus(duration: Duration): DateTime {
    const milliseconds = this.epochMilliseconds() + duration.milliseconds()
    if (milliseconds < 0) {
      return DateTime.fromEpochMilliseconds(0)
    } else {
      return DateTime.fromEpochMilliseconds(milliseconds)
    }
  }

  hasCome(): boolean {
    return this.isAfterOrAt(DateTime.now())
  }

  static max(): DateTime {
    return new DateTime(new Date(DateTime.MAX_EPOCH_MILLISECONDS))
  }

  static min(): DateTime {
    return new DateTime(new Date(DateTime.MIN_TIMESTAMP))
  }

  static now(timeZone: TimeZone = BAGHDAD_TIME_ZONE): DateTime {
    return new DateTime(changeTimeZoneOffset(new Date(), timeZone.offset))
  }

  static fromEpochMilliseconds(milliseconds: number): DateTime {
    return new DateTime(new Date(milliseconds))
  }

  static tryFromEpochMilliseconds(milliseconds: number): Option<DateTime> {
    try {
      return new Some(new DateTime(new Date(milliseconds)))
    } catch {
      return new None()
    }
  }
  /**
   * Examples of the ISO 8601 format:
   * - 2024-04-14
   * - 2024-04-14T10:30:00Z
   * - 2024-04-14T10:30:00+05:30
   */
  static fromISO8601(string: string): Tried<DateTime, unknown> {
    try {
      return new Ok(new DateTime(new Date(string)))
    } catch (error) {
      return new Err(error)
    }
  }
  /**
   * Examples of the RFC 2822 format:
   * - Mon, 14 Apr 2024 10:30:00 +053
   * - 14 Apr 2024 10:30:00 +0530
   */
  static fromRFC2822(string: string): Tried<DateTime, unknown> {
    try {
      return new Ok(new DateTime(new Date(string)))
    } catch (error) {
      return new Err(error)
    }
  }


  time(): Time {
    return new Time(this.hour(), this.minute())
  }

  until(other: DateTime) {
    const a = this.epochMilliseconds()
    const b = other.epochMilliseconds()
    if (a < b) {
      return Duration.fromMilliseconds(b - a)
    }
    return Duration.fromMilliseconds(0)
  }

  inner() {
    return this.value
  }
}
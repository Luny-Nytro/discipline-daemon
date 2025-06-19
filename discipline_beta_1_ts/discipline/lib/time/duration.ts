import { Option, Some, None } from "Pkg"

export class Duration {
  static readonly MILLISECONDS_PER_SECOND = 1000
  static readonly MILLISECONDS_PER_MINUTE = this.MILLISECONDS_PER_SECOND * 60
  static readonly MILLISECONDS_PER_HOUR   = this.MILLISECONDS_PER_MINUTE * 60
  static readonly MILLISECONDS_PER_DAY    = this.MILLISECONDS_PER_HOUR * 24
  static readonly MILLISECONDS_PER_WEEK   = this.MILLISECONDS_PER_DAY * 7

  static readonly SECONDS_PER_MINUTE = 60
  static readonly SECONDS_PER_HOUR   = this.SECONDS_PER_MINUTE * 60
  static readonly SECONDS_PER_DAY    = this.SECONDS_PER_HOUR * 24
  static readonly SECONDS_PER_WEEK   = this.SECONDS_PER_DAY * 7

  static readonly MINUTES_PER_HOUR = 60
  static readonly MINUTES_PER_DAY  = this.MINUTES_PER_HOUR * 24
  static readonly MINUTES_PER_WEEK = this.MINUTES_PER_DAY * 7

  static readonly HOURS_PER_DAY  = 24
  static readonly HOURS_PER_WEEK = this.HOURS_PER_DAY * 7

  constructor(readonly value: number) {}

  static fromMilliseconds(milliseconds: number): Duration {
    return new Duration(milliseconds)
  }

  static tryFromMilliseconds(milliseconds: number): Option<Duration> {
    if (Number.isSafeInteger(milliseconds)) {
      return new Some(new Duration(milliseconds))
    } else {
      return new None()
    }
  }

  static fromSeconds(seconds: number): Duration {
    return new Duration(Math.floor(seconds * Duration.MILLISECONDS_PER_SECOND))
  }

  static fromMinutes(minutes: number): Duration {
    return new Duration(Math.floor(minutes * Duration.MILLISECONDS_PER_MINUTE))
  }

  static fromHours(hours: number): Duration {
    return new Duration(Math.floor(hours * Duration.MILLISECONDS_PER_HOUR))
  }

  static fromDays(days: number): Duration {
    return new Duration(Math.floor(days * Duration.MILLISECONDS_PER_DAY))
  }

  static fromWeeks(weeks: number): Duration {
    return new Duration(Math.floor(weeks * Duration.MILLISECONDS_PER_WEEK))
  }

  milliseconds(): number {
    return Math.floor(this.value) as number
  }

  seconds(): number {
    return Math.floor(this.value / Duration.MILLISECONDS_PER_SECOND) as number
  }

  minutes(): number {
    return Math.floor(this.value / Duration.MILLISECONDS_PER_MINUTE) as number
  }

  hours(): number {
    return Math.floor(this.value / Duration.MILLISECONDS_PER_HOUR) as number
  }

  days(): number {
    return Math.floor(this.value / Duration.MILLISECONDS_PER_DAY) as number
  }

  weeks(): number {
    return Math.floor(this.value / Duration.MILLISECONDS_PER_WEEK) as number
  }

  clone(): Duration {
    return this
  }

  eq(other: Duration): boolean {
    return this.value === other.value
  }

  greaterThenOrEqual(other: Duration): boolean {
    return this.value >= other.value
  }

  lessThanOrEqual(other: Duration): boolean {
    return this.value <= other.value
  }

  lessThan(other: Duration): boolean {
    return this.value < other.value
  }

  plus(other: Duration): Option<Duration> {
    return Duration.tryFromMilliseconds(this.value + other.value)
  }

  minus(other: Duration) {
    const value = this.value - other.value
    if (value > 0) {
      return new Duration(value)
    }
    return new Duration(0)
  }
}
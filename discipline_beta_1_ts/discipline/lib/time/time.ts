import { Duration, Hour, Minute } from "../mod.ts";

export class Time {
  constructor(
    readonly hour: Hour,
    readonly minute: Minute,
  ) {}

  private value() {
    return (
      Duration.MILLISECONDS_PER_HOUR * this.hour.value
    ) + (
      Duration.MILLISECONDS_PER_MINUTE * this.minute.value
    )
  }

  isWithin(from: Time, till: Time) {
    return from.value() <= this.value() && this.value() <= till.value()
  }
}
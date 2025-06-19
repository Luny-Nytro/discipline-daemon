import { Option, Some, None } from "Pkg"

export const enum Value {
  Sun = 0,
  Mon = 1,
  Tus = 2,
  Wed = 3,
  Thu = 4,
  Fri = 5,
  Sat = 6,  
}

export class Weekday {
  private constructor(private readonly value: Value) {}

  isSun(): boolean {
    return this.value === Value.Sun
  }
  isMon(): boolean {
    return this.value === Value.Mon
  }
  isTus(): boolean {
    return this.value === Value.Tus
  }
  isWed(): boolean {
    return this.value === Value.Wed
  }
  isThu(): boolean {
    return this.value === Value.Thu
  }
  isFri(): boolean {
    return this.value === Value.Fri
  }
  isSat(): boolean {
    return this.value === Value.Sat
  }
  toNumber0() {
    return this.value
  }
  value0() {
    return this.value
  }
  static Sun() {
    return new Weekday(Value.Sun)
  }
  static Mon() {
    return new Weekday(Value.Mon)
  }
  static Tus() {
    return new Weekday(Value.Tus)
  }
  static Wed() {
    return new Weekday(Value.Wed)
  }
  static Thu() {
    return new Weekday(Value.Thu)
  }
  static Fri() {
    return new Weekday(Value.Fri)
  }
  static Sat() {
    return new Weekday(Value.Sat)
  }
  static fromNumber0(number: number): Option<Weekday> {
    switch (number) {
      case Value.Sun: return new Some(Weekday.Sun())
      case Value.Mon: return new Some(Weekday.Mon())
      case Value.Tus: return new Some(Weekday.Tus())
      case Value.Wed: return new Some(Weekday.Wed())
      case Value.Thu: return new Some(Weekday.Thu())
      case Value.Fri: return new Some(Weekday.Fri())
      case Value.Sat: return new Some(Weekday.Sat())
    }
    return new None()
  }

  isWithin(from: Weekday, till: Weekday) {
    return from.value <= this.value && this.value <= till.value
  }

  name() {
    switch (this.value) {
      case Value.Sun: return "Sun"
      case Value.Mon: return "Mon"
      case Value.Tus: return "Tus"
      case Value.Wed: return "Wed"
      case Value.Thu: return "Thu"
      case Value.Fri: return "Fri"
      case Value.Sat: return "Sat"    
    }
  }
}

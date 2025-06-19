import { Some, None } from "Pkg"

export type Value1_24 = (
  | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  | 9  | 10 
  | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 
  | 21 | 22 | 23 | 24
)

export type Value0_23 = (
  | 0  | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  | 9  
  | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 
  | 20 | 21 | 22 | 23
)

export type Value1_12 = (
  | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  | 9  | 10 
  | 11 | 12
)

export class Hour {
  private constructor(readonly value: Value1_24) {}

  static am(value: Value1_12) { 
    return new Hour(value as Value1_24)
  }

  static pm(value: Value1_12) {
    return new Hour(value + 12 as Value1_24)
  }

  static new1_24(value: Value1_24) {
    return new Hour(value)
  }

  static new1_24Checked(value: number) {
    if (Number.isInteger(value) && value >= 1 && value <= 24) {
      return new Some(new Hour(value as Value1_24))
    } else {
      return new None()
    }
  }

  static new1_24Unchecked(value: number) {
    return new Hour(value as Value1_24)
  }

  static new0_23(value: Value0_23) {
    return new Hour(value + 1 as Value1_24)
  }

  static new0_23Checked(value: number) {
    if (Number.isInteger(value) && value >= 0 && value <= 23) {
      return new Some(new Hour(value + 1 as Value1_24))
    } else {
      return new None()
    }
  }

  static new0_23Unchecked(value: number) {
    return new Hour(value + 1 as Value1_24)
  }

  isWithin(from: Hour, till: Hour) {
    return from.value <= this.value && this.value <= till.value
  }

  value1_24() {
    return this.value
  }
}
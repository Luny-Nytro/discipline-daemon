import { Some, None } from "Pkg";

export type Value1 = (
  | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  | 9  | 10 
  | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 
  | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 
  | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 
  | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 
  | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60
)

export type Value0 = (
  | 0  | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  | 9  
  | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 
  | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 
  | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 
  | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 
  | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59
)

export class Minute {
  private constructor(readonly value: Value1) {}

  static new1(value: Value1) {
    return new Minute(value)
  }

  static new1Unchecked(value: number) {
    return new Minute(value as Value1)
  }

  static new1Checked(value: number) {
    if (Number.isSafeInteger(value) && value >= 1 && value <= 60) {
      return new Some(new Minute(value as Value1))
    } else {
      return new None()
    }
  }

  static new0(value: Value0) {
    return new Minute(value + 1 as Value1)
  }

  static new0Checked(value: number) {
    if (Number.isSafeInteger(value) && value >= 0 && value <= 59) {
      return new Some(new Minute(value + 1 as Value1))
    } else {
      return new None()
    }
  }

  static new0Uncgecked(value: number) {
    return new Minute(value + 1 as Value1)
  }
  
  isWithin(from: Minute, till: Minute) {
    return from.value <= this.value && this.value <= till.value
  }

  value1() {
    return this.value
  }
}
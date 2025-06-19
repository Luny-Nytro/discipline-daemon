import { Some, None } from "Pkg"

export class Year {
  private constructor(readonly value: number) {}

  static tryFrom(value: number) {
    if (Number.isSafeInteger(value) && value >= 0) {
      return new Some(new Year(value))
    } else {
      return new None()
    }
  }

  static tryFromUnchecked(value: number) {
    return new Year(value)
  }
}
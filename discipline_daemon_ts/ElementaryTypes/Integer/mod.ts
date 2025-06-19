import { Err, Ok, Tried } from "@Pkg/Tried";
import { GenericError } from "@Pkg/GenericError";

export * from "./TypeChecking.ts"

export const Integer = {
  isInteger(value: unknown): value is number {
    return Number.isInteger(value)
  },
  isSafeInteger(value: unknown): value is number {
    return Number.isSafeInteger(value)
  },
  isIntegerAndInRange(value: unknown, from: number, till: number): value is number {
    return Integer.isInteger(value) && value >= from && value <= till
  },
  isPositiveInteger(value: unknown): value is number {
    return Integer.isInteger(value) && value >= 0
  },
  isSafeAndPositive(value: unknown): value is number {
    return Integer.isSafeInteger(value) && value >= 0
  },
  fromString(string: string): Tried<number, GenericError> {
    const integerOrNaN = parseInt(string)
    if (Object.is(integerOrNaN, NaN)) {
      const error = GenericError.new("Parse string as integer")
      GenericError.addMessage(error, "Argument 'string' could not be parsed as integer")
      GenericError.addNamedAttachment(error, "Argument 'string'", string)
      return Err(error)
    }

    return Ok(integerOrNaN)
  },
}
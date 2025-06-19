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
  }
}
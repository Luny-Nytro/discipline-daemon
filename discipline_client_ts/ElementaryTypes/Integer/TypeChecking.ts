export function isInteger(value: unknown): value is number {
  return Number.isInteger(value)
}

export function isSafeInteger(value: unknown): value is number {
  return Number.isSafeInteger(value)
}

export function isIntegerAndInRange(value: unknown, from: number, till: number): value is number {
  return isInteger(value) && value >= from && value <= till
}

export function isPositiveInteger(value: unknown): value is number {
  return isInteger(value) && value >= 0
}

export function isSafeAndPositive(value: unknown): value is number {
  return isSafeInteger(value) && value >= 0
}
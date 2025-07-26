export function isIntegerAndBetween<T = number>(value: unknown, min: number, max: number): value is T {
  return Number.isInteger(value) && min <= (value as number) && (value as number) <= max
}

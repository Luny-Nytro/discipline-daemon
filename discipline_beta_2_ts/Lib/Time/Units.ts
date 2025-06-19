export const millisecondsPerSecond = 1000
export const millisecondsPerMinute = millisecondsPerSecond * 60
export const millisecondsPerHour = millisecondsPerMinute * 60
export const millisecondsPerDay = millisecondsPerHour * 24
export const millisecondsPerWeek = millisecondsPerDay * 7

export function millisecondsToSeconds(milliseconds: number): number {
  return Math.floor(milliseconds / millisecondsPerSecond)
}
export function millisecondsToMinutes(milliseconds: number): number {
  return Math.floor(milliseconds / millisecondsPerMinute)
}
export function millisecondsToHours(milliseconds: number): number {
  return Math.floor(milliseconds / millisecondsPerHour)
}
export function millisecondsToDays(milliseconds: number): number {
  return Math.floor(milliseconds / millisecondsPerDay)
}
export function millisecondsToWeeks(milliseconds: number): number {
  return Math.floor(milliseconds / millisecondsPerWeek)
}
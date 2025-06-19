import { Duration } from "Pkg";

export function runEvery(duration: Duration, fn: () => unknown) {
  setInterval(fn, duration.milliseconds())
}
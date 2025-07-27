import { Duration } from "../Prelude.ts";

export function interval(duration: Duration, fn: () => void) {
  const milliseconds = Duration.milliseconds(duration)
  fn()
  setInterval(fn, milliseconds);
}
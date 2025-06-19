import * as Duration from "@Pkg/Duration"
import { Unique } from "@Pkg/Unique";

export type Synchronizer = Unique<"Discipline.Utility.Synchronizer", {
  /** @private */
  readonly action: () => void
  /** @private */
  readonly interval: Duration.Duration
  /** @private */
  intervalClearCode: number | null
}>

export function create(interval: Duration.Duration, action: () => void): Synchronizer {
  return Unique({
    action,
    interval,
    intervalClearCode: null,
  })
}

export function isRunning(me: Synchronizer): boolean {
  return me.intervalClearCode !== null
}

export function ensureRunning(me: Synchronizer): void {
  if (!isRunning(me)) {
    me.intervalClearCode = setInterval(
      me.action, 
      Duration.asMilliseconds(me.interval)
    )
  } 
}

export function ensureStopped(me: Synchronizer): void { 
  if (isRunning(me)) {
    clearInterval(me.intervalClearCode as number)
    me.intervalClearCode = null
  }
}
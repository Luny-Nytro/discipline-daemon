import { Duration } from "./Prelude.ts";

export interface CountdownTimerCreator {
  readonly duration: Duration
}

export const CountdownTimerCreator = {
  new(duration: Duration): CountdownTimerCreator {
    return {
      duration,
    }
  },
}

export interface DataHiderCreator {
  readonly name: string
  readonly data: string
  readonly timer: Duration
}

export const DataHiderCreator = {
  new(name: string, data: string, timer: Duration): DataHiderCreator {
    return {
      name,
      data,
      timer,
    }
  },
}
  
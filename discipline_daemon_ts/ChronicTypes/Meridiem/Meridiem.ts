import * as CreateFromStringError from "./CreateFromStringError.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../ElementaryTypes/Tried.ts";

const enum MeridiemType {
  AM = 0,
  PM = 1,
}

export type T = Unique<"App.Chronic.Meridiem", MeridiemType>

export const AM_AS_STRING = "AM"
export const PM_AS_STRING = "PM"

export function AM(): T {
  return Unique(MeridiemType.AM)
}

export function PM(): T {
  return Unique(MeridiemType.PM)
}

export function asString(me: T): string {
  switch (me as MeridiemType) {
    case MeridiemType.AM: return AM_AS_STRING
    case MeridiemType.PM: return PM_AS_STRING
  }
}

export function fromString(string: string): Tried<T, CreateFromStringError.T> {
  switch (string) {
    case AM_AS_STRING: return Ok(AM())
    case PM_AS_STRING: return Ok(PM())
    default: return Err(CreateFromStringError.InvalidString(string))
  }
}

export interface Cases<A, B> {
  readonly AM: () => A
  readonly PM: () => B
}

export function match<A, B>(me: T, cases: Cases<A, B>) {
  switch (me as MeridiemType) {
    case MeridiemType.AM: return cases.AM()
    case MeridiemType.PM: return cases.PM()
  }
}
import * as Displayer from "./Display.ts"
import { Unique } from "./Unique.ts";

export type GenericError = Unique<"Discipline.Elementary.GenericError", {
  readonly message: string
}>

export function create(message: string): GenericError {
  return Unique({
    message
  })
}

export const displayer = Displayer.implement<GenericError>(me => 
  Displayer.asWrappedString("GenericError", me.message)
)
import { Unique } from "@Pkg/Unique";

type Value = string | ((string | Value)[])

export type MultiLineText = Unique<"Discipline.Elementary.MultiLinText", {
  readonly value: Value
}>

export function create(value: Value): MultiLineText {
  return Unique({
    value
  })
}

export function asString(me: MultiLineText): string {
  return serializeLines(0, me.value)
}

function Indentation(level: number): string {
  return "  ".repeat(level)
}

function serializeLine(level: number, line: string): string {
  return Indentation(level) + line
}

function serializeLines(level: number, value: Value): string {
  let output = ""

  for (const item of value) {
    if (typeof item === "string") {
      output += serializeLine(level, item)
      output += "\n"
    } else {
      output += serializeLines(level + 1, item)
    }
  }

  return output
}
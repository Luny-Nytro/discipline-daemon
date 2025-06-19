import * as Displayer from "../Display.ts";

const This = globalThis

export type Null = null
export type Array = Json[]
export type String = string
export type Number = number
export type Object = { [key: string]: Json }
export type Boolean = boolean

export type Json = (
  | Null
  | String
  | Boolean
  | Number
  | Array
  | Object
)

export function Null(): Null {
  return null
}

export function String(string: string): String {
  return string
}

export function Number(number: number): Number {
  return number
}

export function Boolean(boolean: boolean): Boolean {
  return boolean
}

export function Object(): Object {
  return {}
}

export function isNull(me: Json): me is Null {
  return me === null
}

export function isString(me: Json): me is String {
  return typeof me === "string"
}

export function isInteger(me: Json): me is Number {
  return This.Number.isInteger(me)
}

export function isBoolean(me: Json): me is Boolean {
  return typeof me === "boolean"
}

export function isEnum(me: Json): me is string | Object {
  if (typeof me === "string" || isObject(me)) {
    return true
  }
  return false
}

export function isArray(me: Json): me is Array {
  return This.Array.isArray(me)
}

export function isObject(me: Json): me is Object {
  return typeof me === "object" && me !== null && !isArray(me)
}

export function isEmptyObject(me: Json) {
  return isObject(me) && This.Object.entries(me).length === 0
}


interface Cases<A, B, C, D, E, F> {
  readonly Null: () => A
  readonly Array: (json: Array) => E
  readonly Object: (json: Object) => F
  readonly String: (json: string) => B
  readonly Integer: (json: number) => C
  readonly Boolean: (json: boolean) => D
}

function match<A, B, C, D, E, F>(
  me: Json, 
  cases: Cases<A, B, C, D, E, F>,
) {
  if (isNull(me)) {
    return cases.Null()
  }
  if (isString(me)) {
    return cases.String(me)
  }
  if (isInteger(me)) {
    return cases.Integer(me)
  }
  if (isBoolean(me)) {
    return cases.Boolean(me)
  }
  if (isArray(me)) {
    return cases.Array(me)
  }
  if (isObject(me)) {
    return cases.Object(me)
  }
  
  throw new Error("Unreachable")
}

export const displayer = Displayer.implement<Json>((me): Displayer.Displayable => (
  match(me, {
    Null: () =>
      Displayer.asNull(),

    String: string =>
      Displayer.asString(string),

    Integer: integer => 
      Displayer.asNumber(integer),

    Boolean: boolean =>
      Displayer.asBoolean(boolean),

    Array: array => 
      Displayer.asArray(displayer, array),

    Object: object => 
      Displayer.asUnnamedObjectX(displayer, object),
  })
))
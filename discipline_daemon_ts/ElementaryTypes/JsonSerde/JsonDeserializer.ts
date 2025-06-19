import * as Json from "./JaonValue.ts"
import * as Displayer from "../Display.ts"
import { Unique } from "../Unique.ts";
import { Tried, Err, Ok, isOk, isErr } from "../Tried.ts"

function escapeQuotesAndNewLinesAndSpaces(string: string) {
  return `"${string.replaceAll(/"/ug, "\"").replaceAll(/\s/ug, "\\s")}"`
}
function escapeQuotesAndNewLinesAndSpacesThenQuote(string: string) {
  return `"${string.replaceAll(/"/ug, "\"").replaceAll(/\s/ug, "\\s").replaceAll(/\n/ug, "\\n")}"`
}

function displayPath(path: string): string {
  return escapeQuotesAndNewLinesAndSpacesThenQuote(path)
}
function appendPathComponent(path: string, component: string | number) {
  return `${path}.${component}`
}
function displayJson(json: Json.Json): string {
  return Json.displayer.display(json)
}
function displayPropertyName(propertyName: string): string {
  return escapeQuotesAndNewLinesAndSpacesThenQuote(propertyName)
}
function displayEnumVariantName(enumVariantName: string): string {
  return escapeQuotesAndNewLinesAndSpacesThenQuote(enumVariantName)
}

export type Error = Unique<"App.Elementary.JsonDeserializer.Error", {
  message: string
}>

function Error(message: string): Error {
  return Unique({
    message
  })
}

export const errorDisplayer = Displayer.implement<Error>(me =>
  Displayer.asWrappedString("JsonDeserializeError", me.message)
)

export function err(message: string): Error {
  // TODO: Add path to the message.
  // TODO: Add current json value to the message.
  return Unique({
    message: `Deserializer retrned error: ${message}`
  })
}

type Context = Unique<"App.Elementary.JsonDeserializer.Context", {
  path: string
  json: Json.Json
}>

function createContext(json: Json.Json): Context {
  return Unique({
    json,
    path: "JSON",
  })
}

type Json = (
  | null
  | string
  | number
  | boolean
  | { [key: string]: Json }
  | Json[]
)

type ValueReader = {
  value: Json
}

function readAsNull(me: ValueReader): Tried<null, Error> {
  if (me.value === null) {
    return Ok(null)
  }

  return Err(Error(""))
}

function readAsString(me: ValueReader): Tried<string, Error> {
  if (typeof me.value === "string") {
    return Ok(me.value)
  }

  return Err(Error(""))
}

function readAsNumber(me: ValueReader): Tried<number, Error> {
  if (typeof me.value === "number") {
    return Ok(me.value)
  }

  return Err(Error(""))
}

function readAsBoolean(me: ValueReader): Tried<boolean, Error> {
  if (typeof me.value === "boolean") {
    return Ok(me.value)
  }

  return Err(Error(""))
}

type ArrayReader = Json[]

function readAsArray(me: ValueReader): Tried<ArrayReader, Error> {
  if (Array.isArray(me.value)) {
    return Ok(me.value)
  }

  return Err(Error(""))
}

function readAsArrayGivenItemDeserializer<Item>(
  me: ValueReader,
  itemDeserializer: JsonDeserializer<Item>
): Tried<Item[], Error> {
  if (Array.isArray(me.value)) {
    return Ok(me.value)
  }

  return Err(Error(""))
}

type ObjectReader = { [key: string]: Json }

function isObject(json: Json): json is { [key: string]: Json } {
  return typeof json === "object" && json !== null && !Array.isArray(json)
}

function readAsObject(me: ValueReader): Tried<ObjectReader, Error> {
  if (isObject(me.value)) {
    return Ok(me.value)
  }

  return Err(Error(""))
}

function readPropertyAsNull(
  me: ObjectReader, 
  propertyName: string,
): Tried<null, Error> {
  const propertyValue = me[propertyName]
  if (propertyValue === undefined) {
    return Err(Error(""))
  }
  if (propertyValue !== null) {
    return Err(Error(""))
  }
  return Ok(null)
}

function readPropertyAsString(
  me: ObjectReader,
  propertyName: string
): Tried<string, Error> {
  const propertyValue = me[propertyName]
  if (propertyValue === undefined) {
    return Err(Error(""))
  }
  if (typeof propertyValue !== "string") {
    return Err(Error(""))
  }
  return Ok(propertyValue)
}

function readPropertyAsNumber(
  me: ObjectReader,
  propertyName: string
): Tried<number, Error> {
  const propertyValue = me[propertyName]
  if (propertyValue === undefined) {
    return Err(Error(""))
  }
  if (typeof propertyValue !== "number") {
    return Err(Error(""))
  }
  return Ok(propertyValue)
}

function readPropertyAsBoolean(
  me: ObjectReader,
  propertyName: string
): Tried<boolean, Error> {
  const propertyValue = me[propertyName]
  if (propertyValue === undefined) {
    return Err(Error(""))
  }
  if (typeof propertyValue !== "boolean") {
    return Err(Error(""))
  }
  return Ok(propertyValue)
}

function readPropertyAsArray(
  me: ObjectReader,
  propertyName: string
): Tried<Json[], Error> {
  const propertyValue = me[propertyName]
  if (propertyValue === undefined) {
    return Err(Error(""))
  }
  if (!Array.isArray(propertyValue)) {
    return Err(Error(""))
  }
  return Ok(propertyValue)
}

function readPropertyAsArrayGivenItemDeserializer<Item>(
  me: ObjectReader,
  propertyName: string,
  itemDeserializer: JsonDeserializer<Item>
): Tried<Item[], Error> {
  const propertyValue = me[propertyName]
  if (propertyValue === undefined) {
    return Err(Error(""))
  }
  if (!Array.isArray(propertyValue)) {
    return Err(Error(""))
  }
  return Ok(propertyValue)
}

function readPropertyAsObject(
  me: ObjectReader,
  propertyName: string
): Tried<{ [key: string]: Json }, Error> {
  const propertyValue = me[propertyName]
  if (propertyValue === undefined) {
    return Err(Error(""))
  }
  if (!isObject(propertyValue)) {
    return Err(Error(""))
  }
  return Ok(propertyValue)
}

export function as<Value>(
  context: Context,
  deserializer: JsonDeserializer<Value>
): 
  Tried<Value, Error>
{
  return deserializer.impl(context)
}

export function asNull({ path, json }: Context): Tried<null, Error> {
  if (Json.isNull(json)) {
    return Ok(null)
  }

  return Err(Error(`Expected value at path ${displayPath(path)} to be null, but found ${displayJson(json)}`))
}

export function asString({ path, json }: Context): Tried<string, Error> {
  if (Json.isString(json)) {
    return Ok(json)
  }

  return Err(Error(`Expected value at path ${displayPath(path)} to be a string, but found ${displayJson(json)}`))
}

export function asInteger({ path, json }: Context): Tried<number, Error> {
  if (Json.isInteger(json)) {
    return Ok(json)
  }

  return Err(Error(`Expected value at path ${displayPath(path)} to be an integer, but found ${displayJson(json)}`))
}

export function asBoolean({ path, json }: Context): Tried<boolean, Error> {
  if (Json.isBoolean(json)) {
    return Ok(json)
  }

  return Err(Error(`Expected value at path ${displayPath(path)} to be a boolean, but found ${displayJson(json)}`))
}

export function asArray<Item>(
  { path, json }: Context,
  itemDeserializer: JsonDeserializer<Item>
): 
  Tried<Item[], Error>
{
  if (!Json.isArray(json)) {
    return Err(Error(`Expected value at path ${displayPath(path)} to be an array, but found ${displayJson(json)}`))
  }

  const array: Item[] = []
  let index = 0

  for (const itemJson of json) {
    const itemOrError = itemDeserializer.impl(Unique({
      path: appendPathComponent(path, index),
      json: itemJson
    }))

    if (isOk(itemOrError)) {
      array.push(Tried.value(itemOrError))
      index += 1
    } else {
      return itemOrError
    }
  }

  return Ok(array)
}

export function asObject({path, json }: Context): 
  Tried<Json.Object, Error> 
{
  if (Json.isObject(json)) {
    return Ok(json)
  }

  return Err(Error(`Expected value at path ${displayPath(path)} to be an object, but found ${displayJson(json)}`))
}

type ObjectContext = Unique<"App.Elementary.JsonDeserializer.ObjectContext", {
  readonly path: string
  readonly json: Json.Object
}>

export function asObjectContext(context: Context): 
  Tried<ObjectContext, Error>
{
  return Tried.map(asObject(context), object => Unique({
    path: context.path,
    json: object,
  }))
}

export function propertyAs<Value>(
  { path, json }: ObjectContext,
  propertyName: string,
  propertyValueDeserializer: JsonDeserializer<Value>
):
  Tried<Value, Error>
{
  const propertyValue = json[propertyName]
  if (propertyValue === undefined) {
    return Err(Error(`Expected object at path ${displayPath(path)} to have a property named ${displayPropertyName(propertyName)}`))
  }

  return propertyValueDeserializer.impl(Unique({
    path: appendPathComponent(path, propertyName),
    json: propertyValue,
  }))
}

export function propertyAsNull(
  context: ObjectContext,
  propertyName: string
) {
  return propertyAs(
    context, 
    propertyName,
    nullDeserializer,
  )
}

export function propertyAsString(
  context: ObjectContext,
  propertyName: string
) {
  return propertyAs(
    context,
    propertyName,
    stringDeserializer,
  )
}

export function propertyAsInteger(
  context: ObjectContext,
  propertyName: string
) {
  return propertyAs(
    context,
    propertyName,
    integerDeserializer,
  )
}

export function propertyAsBoolean(
  context: ObjectContext,
  propertyName: string
) {
  return propertyAs(
    context,
    propertyName,
    booleanDeserializer,
  )
}

type EnumContext = Unique<"App.Elementary.JsonDeserializer.EnumContext", {
  readonly path: string
  readonly json: string | Json.Object
}>

export function asEnumVariantContext({ path, json }: Context): Tried<EnumContext, Error> {
  if (Json.isEnum(json)) {
    return Ok(Unique({ path, json }))
  }

  return Err(Error(`Expected value at path ${displayPath(path)} to be an enum variant, which is either a string or an object with a single property, but found ${displayJson(json)}`))
}

const enum EnumVariantType {
  UnitVariant,
  DataVariant,
}

export type EnumUnitVariant<Return> = Unique<"App.Elementary.JsonDeserializer.EnumUnitVariantDeserializer", {
  readonly type: EnumVariantType.UnitVariant
  readonly name: string
  readonly then: () => Return
}>

export function EnumUnitVariant<Return>(
  name: string,
  then: () => Return
): EnumUnitVariant<Return> {
  return Unique({
    type: EnumVariantType.UnitVariant,
    name,
    then,
  })
}

export type EnumDataVariant<Data, Return> = Unique<"App.Elementary.JsonDeserializer.EnumDataVariantDeserializer", {
  readonly type: EnumVariantType.DataVariant
  readonly name: string
  readonly dataDeserializer: JsonDeserializer<Data>
  readonly then: (data: Data) => Return
}>

export function EnumDataVariant<Data, Return>(
  name: string,
  dataDeserializer: JsonDeserializer<Data>,
  then: (data: Data) => Return
): EnumDataVariant<Data, Return> {
  return Unique({
    type: EnumVariantType.DataVariant,
    name,
    dataDeserializer,
    then,
  })
}

type EnumVariant<Data, Return> = (
  | EnumUnitVariant<Return>
  | EnumDataVariant<Data, Return>
)

export function asEnum<Data1, Return1>(context: Context, variant1: EnumVariant<Data1, Return1>): Tried<Return1, Error>;
export function asEnum<Data1, Return1, Data2, Return2>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>): Tried<Return1 | Return2, Error>;
export function asEnum<Data1, Return1, Data2, Return2, Data3, Return3>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>, variant3: EnumVariant<Data3, Return3>): Tried<Return1 | Return2 | Return3, Error>;
export function asEnum<Data1, Return1, Data2, Return2, Data3, Return3, Data4, Return4>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>, variant3: EnumVariant<Data3, Return3>, variant4: EnumVariant<Data4, Return4>): Tried<Return1 | Return2 | Return3 | Return4, Error>;
export function asEnum<Data1, Return1, Data2, Return2, Data3, Return3, Data4, Return4, Data5, Return5>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>, variant3: EnumVariant<Data3, Return3>, variant4: EnumVariant<Data4, Return4>, variant5: EnumVariant<Data5, Return5>): Tried<Return1 | Return2 | Return3 | Return4 | Return5, Error>;
export function asEnum<Data1, Return1, Data2, Return2, Data3, Return3, Data4, Return4, Data5, Return5, Data6, Return6>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>, variant3: EnumVariant<Data3, Return3>, variant4: EnumVariant<Data4, Return4>, variant5: EnumVariant<Data5, Return5>, variant6: EnumVariant<Data6, Return6>): Tried<Return1 | Return2 | Return3 | Return4 | Return5 | Return6, Error>;
export function asEnum<Data1, Return1, Data2, Return2, Data3, Return3, Data4, Return4, Data5, Return5, Data6, Return6, Data7, Return7>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>, variant3: EnumVariant<Data3, Return3>, variant4: EnumVariant<Data4, Return4>, variant5: EnumVariant<Data5, Return5>, variant6: EnumVariant<Data6, Return6>, variant7: EnumVariant<Data7, Return7>): Tried<Return1 | Return2 | Return3 | Return4 | Return5 | Return6 | Return7, Error>;
export function asEnum<Data1, Return1, Data2, Return2, Data3, Return3, Data4, Return4, Data5, Return5, Data6, Return6, Data7, Return7, Data8, Return8>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>, variant3: EnumVariant<Data3, Return3>, variant4: EnumVariant<Data4, Return4>, variant5: EnumVariant<Data5, Return5>, variant6: EnumVariant<Data6, Return6>, variant7: EnumVariant<Data7, Return7>, variant8: EnumVariant<Data8, Return8>): Tried<Return1 | Return2 | Return3 | Return4 | Return5 | Return6 | Return7 | Return8, Error>;
export function asEnum<Data1, Return1, Data2, Return2, Data3, Return3, Data4, Return4, Data5, Return5, Data6, Return6, Data7, Return7, Data8, Return8, Data9, Return9>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>, variant3: EnumVariant<Data3, Return3>, variant4: EnumVariant<Data4, Return4>, variant5: EnumVariant<Data5, Return5>, variant6: EnumVariant<Data6, Return6>, variant7: EnumVariant<Data7, Return7>, variant8: EnumVariant<Data8, Return8>, variant9: EnumVariant<Data9, Return9>): Tried<Return1 | Return2 | Return3 | Return4 | Return5 | Return6 | Return7 | Return8 | Return9, Error>;
export function asEnum<Data1, Return1, Data2, Return2, Data3, Return3, Data4, Return4, Data5, Return5, Data6, Return6, Data7, Return7, Data8, Return8, Data9, Return9, Data10, Return10>(context: Context, variant1: EnumVariant<Data1, Return1>, variant2: EnumVariant<Data2, Return2>, variant3: EnumVariant<Data3, Return3>, variant4: EnumVariant<Data4, Return4>, variant5: EnumVariant<Data5, Return5>, variant6: EnumVariant<Data6, Return6>, variant7: EnumVariant<Data7, Return7>, variant8: EnumVariant<Data8, Return8>, variant9: EnumVariant<Data9, Return9>, variant10: EnumVariant<Data10, Return10>): Tried<Return1 | Return2 | Return3 | Return4 | Return5 | Return6 | Return7 | Return8 | Return9 | Return10, Error>;
export function asEnum(
  context: Context,
  ...variants: EnumVariant<unknown, unknown>[]
): Tried<unknown, Error> {
  const enumContextOrError = asEnumVariantContext(context)
  if (isErr(enumContextOrError)) {
    return enumContextOrError
  }

  const { path, json } = Tried.value(enumContextOrError)
  let error = ""

  for (const variant of variants) {
    switch (variant.type) {
      case EnumVariantType.UnitVariant: {
        if (!Json.isString(json)) {
          // error += `DeserializeEnumVariant: `
          // return Err(Error(`Expected a enum unit variant at path ${displayPath(path)}, but found a enum data variant: ${displayJson(json)}`))
          continue
        }
        if (json !== variant.name) {
          // return Err(Error(`Expected a enum unit variant ${displayEnumVariantName(enumVariantName)} at path ${displayPath(path)}, but found an enum unit variant ${displayEnumVariantName(json)}`))
          continue
        }

        return Ok(variant.then())
      }

      case EnumVariantType.DataVariant: {
        if (!Json.isObject(json)) {
          // return Err(Error(`Expected an enum data variant ${displayEnumVariantName(enumVariantName)} at path ${displayPath(path)}, but found an enum unit variant ${displayEnumVariantName(json)}`))
          continue
        }

        const data = json[variant.name]
        if (data === undefined) {
          // return Err(Error(`Expected an enum data variant ${displayEnumVariantName(enumVariantName)} at path ${displayPath(path)}, but found an enum data variant ${displayJson(json)}`))
          continue
        }

        return Tried.map(
          variant.dataDeserializer.impl(Unique({
            path: appendPathComponent(path, variant.name),
            json: data,
          })),
          variant.then
        )
      }
    }
  }

  // TODO: Write an error message here.
  return Err(Error(""))
}

export function asEnumUnitVariant<Value>(
  { path, json }: EnumContext,
  enumVariantName: string,
  then: () => Value,
):
  Tried<Value, Error>
{
  if (!Json.isString(json)) {
    return Err(Error(`Expected a enum unit variant at path ${displayPath(path)}, but found a enum data variant: ${displayJson(json)}`))
  }
  if (json !== enumVariantName) {
    return Err(Error(`Expected a enum unit variant ${displayEnumVariantName(enumVariantName)} at path ${displayPath(path)}, but found an enum unit variant ${displayEnumVariantName(json)}`))
  }
  return Ok(then())
}

export function asEnumDataVariant<Data, Return>(
  { path, json }: EnumContext,
  enumVariantName: string,
  dataDeserializer: JsonDeserializer<Data>,
  then: (data: Data) => Return
):
  Tried<Return, Error>
{
  if (!Json.isObject(json)) {
    return Err(Error(`Expected an enum data variant ${displayEnumVariantName(enumVariantName)} at path ${displayPath(path)}, but found an enum unit variant ${displayEnumVariantName(json)}`))
  }

  const data = json[enumVariantName]
  if (data === undefined) {
    return Err(Error(`Expected an enum data variant ${displayEnumVariantName(enumVariantName)} at path ${displayPath(path)}, but found an enum data variant ${displayJson(json)}`))
  }

  return Tried.map(
    dataDeserializer.impl(Unique({
      path: appendPathComponent(path, enumVariantName),
      json: data,
    })),
    then
  )
}

export function asEnumNullVariant<Return>(
  context: EnumContext,
  enumVariantName: string,
  then: (data: null) => Return
): Tried<Return, Error> {
  return asEnumDataVariant(
    context,
    enumVariantName,
    nullDeserializer,
    then,
  )
}

export function asEnumStringVariant<Return>(
  context: EnumContext,
  enumVariantName: string,
  then: (data: string) => Return
):
  Tried<Return, Error>
{
  return asEnumDataVariant(
    context,
    enumVariantName,
    stringDeserializer,
    then,
  )
}

export function asEnumIntegerVariant<Return>(
  context: EnumContext,
  enumVariantName: string,
  then: (data: number) => Return
): Tried<Return, Error> {
  return asEnumDataVariant(
    context,
    enumVariantName,
    integerDeserializer,
    then,
  )
}

export function asEnumBooleanVariant<Return>(
  context: EnumContext,
  enumVariantName: string,
  then: (data: boolean) => Return
): Tried<Return, Error> {
  return asEnumDataVariant(
    context,
    enumVariantName,
    booleanDeserializer,
    then,
  )
}

// export function asEnumArrayVariant<Element, Return>(
//   context: EnumContext,
//   enumVariantName: string,
//   elementDeserializer: Deserializer<Element>,
//   then: (data: Element[]) => Return
// ): Tried<Return, Error> {
//   return asEnumDataVariant(
//     context,
//     enumVariantName,
//     arrayDeserializer(elementDeserializer), // Assumes arrayDeserializer is a factory
//     then,
//   )
// }

// export function asEnumObjectVariant<Return>(
//   context: EnumContext,
//   enumVariantName: string,
//   then: (data: Record<string, unknown>) => Return
// ): Tried<Return, Error> {
//   return asEnumDataVariant(
//     context,
//     enumVariantName,
//     objectDeserializer, // Assumes you have an object deserializer
//     then,
//   )
// }

// export function propertyAsArray<T>(
//   context: ObjectContext,
//   propertyName: string,
//   elementDeserializer: JsonDeserializer<T>
// ) {
//   return propertyAs(
//     context,
//     propertyName,
//     arrayDeserializer(elementDeserializer),
//   )
// }

// export function propertyAsObject(
//   context: ObjectContext,
//   propertyName: string
// ) {
//   return propertyAs(
//     context,
//     propertyName,
//     objectDeserializer, // Assuming this exists
//   )
// }

export type JsonDeserializeImpl<Value> = (
  (context: Context) => Tried<Value, Error>
)

export interface JsonDeserializer<Value> {
  readonly impl: JsonDeserializeImpl<Value>
  readonly deserialize: (jsonText: string) => Tried<Value, Error>
}

export function implement<Value>(impl: JsonDeserializeImpl<Value>): JsonDeserializer<Value> {
  function deserialize(jsonText: string) { 
    let json: Json.Json
  
    try {
      json = JSON.parse(jsonText)
    } catch (error) {
      return Err(Error(`Failed to parse json text. Json text: ${
        Displayer.stringDisplayer.display(jsonText)
      }. Parse error: ${
        Displayer.stringDisplayer.display(String(error))
      }`))
    }
  
    return Tried.mapErr(impl(createContext(json)), error => 
      Error(`Deserializer returned error: ${error.message}`)
    )
  }

  return Unique({
    impl,
    deserialize,
  })
}

export const nullDeserializer = implement<null>((context) => 
  asNull(context)
)

export const stringDeserializer = implement<string>((context) => 
  asString(context)
)

export const integerDeserializer = implement<number>((context) => 
  asInteger(context)
)

export const booleanDeserializer = implement<boolean>((context) => 
  asBoolean(context)
)

export const implementForArray = <Item>(itemDeserializer: JsonDeserializer<Item>) => 
  implement<Item[]>(context =>
    asArray(context, itemDeserializer)
  )
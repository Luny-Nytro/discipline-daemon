import * as Json from "./JaonValue.ts"
import * as Displayer from "@Pkg/Display"
import { Unique } from "@Pkg/Unique";
import { Tried, Err, Ok, isOk, isErr, lazyOk } from "@Pkg/Tried"

const enum Type {
  Unit,
  String,
  Integer,
  Boolean,
  Object,
  Array,
  Enum,
  EnumUnitVariant,
  EnumDataVariant,
}

type Deserializer<Final> = (
  | Unit<Final>
  | String<Final>
  | Integer<Final>
  | Boolean<Final>
  | Array<Final>
  | Object<Final>
)

export interface Unit<Final> {
  readonly type: Type.Unit
  readonly finalize: () => Tried<Final, Error>
}

export function Unit<Final>(finalize: () => Tried<Final, Error>): Unit<Final> {
  return {
    type: Type.Unit,
    finalize,
  }
}

export interface String<Final> {
  readonly type: Type.String
  readonly finalize: (string: string) => Tried<Final, Error>
}

export function String<Final>(finalize: (string: string) => Tried<Final, Error>): String<Final> {
  return {
    type: Type.String,
    finalize,
  }
}

export interface Integer<Final> {
  readonly type: Type.Integer
  readonly finalize: (integer: number) => Tried<Final, Error>
}

export function Integer<Final>(finalize: (integer: number) => Tried<Final, Error>): Integer<Final> {
  return {
    type: Type.Integer,
    finalize,
  }
}

export interface Boolean<Final> {
  readonly type: Type.Boolean
  readonly finalize: (boolean: boolean) => Tried<Final, Error>
}

export interface Array<ArrayFinal, ItemFinal = unknown> {
  readonly type: Type.Array
  readonly itemDeserializer: Deserializer<ItemFinal>
  readonly finalize: (array: ItemFinal[]) => Tried<ArrayFinal, Error>
}

export function Array<ArrayFinal, ItemFinal>(
  itemDeserializer: Deserializer<ItemFinal>,
  finalize: (array: ItemFinal[]) => Tried<ArrayFinal, Error>
): Array<ArrayFinal, ItemFinal> {
  return {
    type: Type.Array,
    itemDeserializer,
    finalize,
  }
}

export interface Property<Final> {
  readonly name: string
  readonly deserializer: Deserializer<Final>
}

export function Property<Final>(
  name: string, 
  deserializer: Deserializer<Final>,
): Property<Final> {
  return {
    name,
    deserializer,
  }
}

export interface Object<
  ObjectFinal, 
  PropertiesFinal extends unknown[] = unknown[]
> {
  readonly type: Type.Object
  readonly propertyDeserializers: PropertiesFinal extends (infer U)[] 
    ? Deserializer<U>[] 
    : never
  readonly finalize: (...properties: PropertiesFinal) => Tried<ObjectFinal, Error>
}

export function Object<ObjectFinal, Property1Final>(property1Deserializer: Property<Property1Final>, finalize: (property1: Property1Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, finalize: (property1: Property1Final, property2: Property2Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final, Property3Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, property3Deserializer: Property<Property3Final>, finalize: (property1: Property1Final, property2: Property2Final, property3: Property3Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final, Property3Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final, Property3Final, Property4Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, property3Deserializer: Property<Property3Final>, property4Deserializer: Property<Property4Final>, finalize: (property1: Property1Final, property2: Property2Final, property3: Property3Final, property4: Property4Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final, Property3Final, Property4Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final, Property3Final, Property4Final, Property5Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, property3Deserializer: Property<Property3Final>, property4Deserializer: Property<Property4Final>, property5Deserializer: Property<Property5Final>, finalize: (property1: Property1Final, property2: Property2Final, property3: Property3Final, property4: Property4Final, property5: Property5Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final, Property3Final, Property4Final, Property5Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, property3Deserializer: Property<Property3Final>, property4Deserializer: Property<Property4Final>, property5Deserializer: Property<Property5Final>, property6Deserializer: Property<Property6Final>, finalize: (property1: Property1Final, property2: Property2Final, property3: Property3Final, property4: Property4Final, property5: Property5Final, property6: Property6Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final, Property7Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, property3Deserializer: Property<Property3Final>, property4Deserializer: Property<Property4Final>, property5Deserializer: Property<Property5Final>, property6Deserializer: Property<Property6Final>, property7Deserializer: Property<Property7Final>, finalize: (property1: Property1Final, property2: Property2Final, property3: Property3Final, property4: Property4Final, property5: Property5Final, property6: Property6Final, property7: Property7Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final, Property7Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final, Property7Final, Property8Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, property3Deserializer: Property<Property3Final>, property4Deserializer: Property<Property4Final>, property5Deserializer: Property<Property5Final>, property6Deserializer: Property<Property6Final>, property7Deserializer: Property<Property7Final>, property8Deserializer: Property<Property8Final>, finalize: (property1: Property1Final, property2: Property2Final, property3: Property3Final, property4: Property4Final, property5: Property5Final, property6: Property6Final, property7: Property7Final, property8: Property8Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final, Property7Final, Property8Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final, Property7Final, Property8Final, Property9Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, property3Deserializer: Property<Property3Final>, property4Deserializer: Property<Property4Final>, property5Deserializer: Property<Property5Final>, property6Deserializer: Property<Property6Final>, property7Deserializer: Property<Property7Final>, property8Deserializer: Property<Property8Final>, property9Deserializer: Property<Property9Final>, finalize: (property1: Property1Final, property2: Property2Final, property3: Property3Final, property4: Property4Final, property5: Property5Final, property6: Property6Final, property7: Property7Final, property8: Property8Final, property9: Property9Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final, Property7Final, Property8Final, Property9Final]>;
export function Object<ObjectFinal, Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final, Property7Final, Property8Final, Property9Final, Property10Final>(property1Deserializer: Property<Property1Final>, property2Deserializer: Property<Property2Final>, property3Deserializer: Property<Property3Final>, property4Deserializer: Property<Property4Final>, property5Deserializer: Property<Property5Final>, property6Deserializer: Property<Property6Final>, property7Deserializer: Property<Property7Final>, property8Deserializer: Property<Property8Final>, property9Deserializer: Property<Property9Final>, property10Deserializer: Property<Property10Final>, finalize: (property1: Property1Final, property2: Property2Final, property3: Property3Final, property4: Property4Final, property5: Property5Final, property6: Property6Final, property7: Property7Final, property8: Property8Final, property9: Property9Final, property10: Property10Final) => Tried<ObjectFinal, Error>): Object<ObjectFinal, [Property1Final, Property2Final, Property3Final, Property4Final, Property5Final, Property6Final, Property7Final, Property8Final, Property9Final, Property10Final]>;
export function Object(
  ...args: unknown[]
): 
  Object<unknown, unknown[]>
{
  return {
    type: Type.Object,
    finalize: args.at(-1) as (...finalPropertyValues: unknown[]) => Tried<unknown, Error>,
    propertyDeserializers: args.slice(0, args.length - 1) as Deserializer<unknown>[]
  }
}

const enum EnumVariantType {
  UnitVariant,
  DataVariant,
}

export type EnumUnitVariant<Final> = Unique<"Discipline.Elementary.JsonDeserializer.EnumUnitVariantDeserializer", {
  readonly type: EnumVariantType.UnitVariant
  readonly name: string
  readonly finalize: () => Tried<Final, Error>
}>

export function EnumUnitVariant<Final>(
  name: string,
  finalize: () => Tried<Final, Error>
): 
  EnumUnitVariant<Final> 
{
  return Unique({
    type: EnumVariantType.UnitVariant,
    name,
    finalize,
  })
}

export type EnumDataVariant<Final, DataFinal = unknown> = Unique<"Discipline.Elementary.JsonDeserializer.EnumDataVariantDeserializer", {
  readonly type: EnumVariantType.DataVariant
  readonly name: string
  readonly dataDeserializer: JsonDeserializer<DataFinal>
  readonly finalize: (data: DataFinal) => Tried<Final, Error>
}>

export function EnumDataVariant<Final, DataFinal = unknown>(
  name: string,
  dataDeserializer: JsonDeserializer<DataFinal>,
  finalize: (data: DataFinal) => Tried<Final, Error>
): 
  EnumDataVariant<Final, DataFinal> 
{
  return Unique({
    type: EnumVariantType.DataVariant,
    name,
    dataDeserializer,
    finalize,
  })
}

type EnumVariant<Final> = (
  | EnumUnitVariant<Final>
  | EnumDataVariant<Final>
)

export interface Enum<Final> {
  readonly type: Type.Enum
  readonly variants: EnumVariant<Final>[]
}

export function Enum<Variant1Final>(variant1: EnumVariant<Variant1Final>): Enum<Variant1Final>;
export function Enum<Variant1Final, Variant2Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>): Enum<Variant1Final | Variant2Final>;
export function Enum<Variant1Final, Variant2Final, Variant3Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>, variant3: EnumVariant<Variant3Final>): Enum<Variant1Final | Variant2Final | Variant3Final>;
export function Enum<Variant1Final, Variant2Final, Variant3Final, Variant4Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>, variant3: EnumVariant<Variant3Final>, variant4: EnumVariant<Variant4Final>): Enum<Variant1Final | Variant2Final | Variant3Final | Variant4Final>;
export function Enum<Variant1Final, Variant2Final, Variant3Final, Variant4Final, Variant5Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>, variant3: EnumVariant<Variant3Final>, variant4: EnumVariant<Variant4Final>, variant5: EnumVariant<Variant5Final>): Enum<Variant1Final | Variant2Final | Variant3Final | Variant4Final | Variant5Final>;
export function Enum<Variant1Final, Variant2Final, Variant3Final, Variant4Final, Variant5Final, Variant6Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>, variant3: EnumVariant<Variant3Final>, variant4: EnumVariant<Variant4Final>, variant5: EnumVariant<Variant5Final>, variant6: EnumVariant<Variant6Final>): Enum<Variant1Final | Variant2Final | Variant3Final | Variant4Final | Variant5Final | Variant6Final>;
export function Enum<Variant1Final, Variant2Final, Variant3Final, Variant4Final, Variant5Final, Variant6Final, Variant7Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>, variant3: EnumVariant<Variant3Final>, variant4: EnumVariant<Variant4Final>, variant5: EnumVariant<Variant5Final>, variant6: EnumVariant<Variant6Final>, variant7: EnumVariant<Variant7Final>): Enum<Variant1Final | Variant2Final | Variant3Final | Variant4Final | Variant5Final | Variant6Final | Variant7Final>;
export function Enum<Variant1Final, Variant2Final, Variant3Final, Variant4Final, Variant5Final, Variant6Final, Variant7Final, Variant8Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>, variant3: EnumVariant<Variant3Final>, variant4: EnumVariant<Variant4Final>, variant5: EnumVariant<Variant5Final>, variant6: EnumVariant<Variant6Final>, variant7: EnumVariant<Variant7Final>, variant8: EnumVariant<Variant8Final>): Enum<Variant1Final | Variant2Final | Variant3Final | Variant4Final | Variant5Final | Variant6Final | Variant7Final | Variant8Final>;
export function Enum<Variant1Final, Variant2Final, Variant3Final, Variant4Final, Variant5Final, Variant6Final, Variant7Final, Variant8Final, Variant9Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>, variant3: EnumVariant<Variant3Final>, variant4: EnumVariant<Variant4Final>, variant5: EnumVariant<Variant5Final>, variant6: EnumVariant<Variant6Final>, variant7: EnumVariant<Variant7Final>, variant8: EnumVariant<Variant8Final>, variant9: EnumVariant<Variant9Final>): Enum<Variant1Final | Variant2Final | Variant3Final | Variant4Final | Variant5Final | Variant6Final | Variant7Final | Variant8Final | Variant9Final>;
export function Enum<Variant1Final, Variant2Final, Variant3Final, Variant4Final, Variant5Final, Variant6Final, Variant7Final, Variant8Final, Variant9Final, Variant10Final>(variant1: EnumVariant<Variant1Final>, variant2: EnumVariant<Variant2Final>, variant3: EnumVariant<Variant3Final>, variant4: EnumVariant<Variant4Final>, variant5: EnumVariant<Variant5Final>, variant6: EnumVariant<Variant6Final>, variant7: EnumVariant<Variant7Final>, variant8: EnumVariant<Variant8Final>, variant9: EnumVariant<Variant9Final>, variant10: EnumVariant<Variant10Final>): Enum<Variant1Final | Variant2Final | Variant3Final | Variant4Final | Variant5Final | Variant6Final | Variant7Final | Variant8Final | Variant9Final | Variant10Final>;
export function Enum(...variants: EnumVariant<unknown>[]): 
  Enum<unknown>
{
  return {
    type: Type.Enum,
    variants,
  }
}

Enum(
  EnumUnitVariant("Moon", lazyOk(() => null)),
  EnumDataVariant("AtHour", String(lazyOk(idenity)), lazyOk(() => null)),
)

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

  console.log(enumVariantName, json, json[enumVariantName])
  const data = json[enumVariantName]
  if (data === undefined) {
    return Err(Error(`Expected an enum data variant ${displayEnumVariantName(enumVariantName)} at path ${displayPath(path)}, but found an enum data variant ${displayJson(json)}`))
  }

  console.log("after condition")
  console.log("Data Deserializer", dataDeserializer.impl.toString())
  const x = dataDeserializer.impl(Unique({
    path: appendPathComponent(path, enumVariantName),
    json: data,
  }))
  console.log("Data return", x)

  return Tried.map(
    dataDeserializer.impl(Unique({
      path: appendPathComponent(path, enumVariantName),
      json: data,
    })),
    then
  )
}

function idenity<Value>(value: Value): Value {
  return value
}

const x = Object(
  Property("id", Integer(lazyOk(idenity))),
  Property("name", String(lazyOk(idenity))),
  Property("theme", String(lazyOk(idenity))),
  lazyOk((id, name, theme) => ({
    id, name, theme
  }))
)

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

export type Error = Unique<"Discipline.Elementary.JsonDeserializer.Error", {
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

type Context = Unique<"Discipline.Elementary.JsonDeserializer.Context", {
  path: string
  json: Json.Json
}>

function createContext(json: Json.Json): Context {
  return Unique({
    json,
    path: "JSON",
  })
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

type ObjectContext = Unique<"Discipline.Elementary.JsonDeserializer.ObjectContext", {
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

type EnumContext = Unique<"Discipline.Elementary.JsonDeserializer.EnumContext", {
  readonly path: string
  readonly json: string | Json.Object
}>

export function asEnumVariantContext({ path, json }: Context): Tried<EnumContext, Error> {
  if (Json.isEnum(json)) {
    return Ok(Unique({ path, json }))
  }

  return Err(Error(`Expected value at path ${displayPath(path)} to be an enum variant, which is either a string or an object with a single property, but found ${displayJson(json)}`))
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
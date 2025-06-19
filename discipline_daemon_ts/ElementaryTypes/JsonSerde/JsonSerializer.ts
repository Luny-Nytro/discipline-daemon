import { Inspection } from "@Pkg/Inspection"
import { Err, Ok, Tried } from "@Pkg/Tried";
import { Unique } from "@Pkg/Unique";
import { isSome, None, Option, Some } from "@Pkg/Option";

function escapeQuotesAndNewLinesAndSpacesThenQuote(string: string) {
  return `"${string.replaceAll(/"/ug, "\"").replaceAll(/\s/ug, "\\s").replaceAll(/\n/ug, "\\n")}"`
}

export type Error = Unique<"App.Elementary.JsonSerializer.Error", {
  readonly message: string
}>

function Error(message: string): Error {
  return Unique({
    message
  })
}

// export const errorDisplayer = Inspection.implement<Error>(me => 
//   Inspection.asWrappedString("JsonSerializeError", me.message)
// )

type SerializableValue = (
  | string
  | null
  | number
  | boolean
  | SerializableValue[]
  | { [key: string]: SerializableValue }
)

type Array = SerializableValue[]
type Object = Record<string, SerializableValue>

type ValueWriter = {
  value: Option<SerializableValue>
}

function writeNull(me: ValueWriter): Tried<null, Error> {
  if (isSome(me.value)) {
    return Err(Error(""))
  }

  me.value = Some(null)
  return Ok(null)
}

function writeString(me: ValueWriter, value: string): Tried<null, Error> {
  if (isSome(me.value)) {
    return Err(Error(""))
  }

  me.value = Some(value)
  return Ok(null)
}

function createNumber(me: ValueWriter, value: number): Tried<null, Error> {
  if (isSome(me.value)) {
    return Err(Error(""))
  }

  me.value = Some(value)
  return Ok(null)
}

function createBoolean(me: ValueWriter, value: boolean): Tried<null, Error> {
  if (isSome(me.value)) {
    return Err(Error(""))
  }

  me.value = Some(value)
  return Ok(null)
}

function writeArrayGivenItemSerializer<Item>(
  me: ValueWriter,
  array: Item[], 
  serializer: JsonSerializer<Item>
): 
  Tried<null, Error>
{
  if (isSome(me.value)) {
    return Err(Error(""))
  }

  const serializableArray: SerializableValue[] = []
  for (const item of array) {
    serializableArray.push(serializer.toSerializable(item))
  }
  
  me.value = Some(serializableArray)
  return Ok(null)
}

function createObjectWriter(
  me: ValueWriter,
  name: string
): Tried<Object, Error> {
  if (isSome(me.value)) {
    return Err(Error(""))
  }

  const object: Object = {}
  me.value = Some(object)
  return Ok(object)
}

function writeNullProperty(me: Object, propertyName: string): Tried<null, Error> {
  if (propertyName in me) {
    return Err(Error(""))
  }

  me[propertyName] = null
  return Ok(null)
}

function writeStringProperty(me: Object, propertyName: string, propertyValue: string): Tried<null, Error> {
  if (propertyName in me) {
    return Err(Error(""))
  }

  me[propertyName] = propertyValue
  return Ok(null)
}

function writeNumberProperty(me: Object, propertyName: string, propertyValue: number): Tried<null, Error> {
  if (propertyName in me) {
    return Err(Error(""))
  }

  me[propertyName] = propertyValue
  return Ok(null)
}

function writeBooleanProperty(me: Object, propertyName: string, propertyValue: boolean): Tried<null, Error> {
  if (propertyName in me) {
    return Err(Error(""))
  }

  me[propertyName] = propertyValue
  return Ok(null)
}

function writeArrayPropertyGivenItemSerializer<Item>(
  me: Object, 
  propertyName: string, 
  propertyValue: Item[],
  itemSerializer: JsonSerializer<Item>
) {
  if (propertyName in me) {
    return Err(Error(""))
  }

  const serializableArray: SerializableValue[] = []
  for (const item of propertyValue) {
    serializableArray.push(itemSerializer.toSerializable(item))
  }

  me[propertyName] = serializableArray
  return Ok(null)
}

type EnumWriter = {
  readonly variantName: string
}

function createEnumWriter() {
  
}
function createEnumWriterForValue(
  me: ValueWriter,
  enumName: string,
  variantName: string,
): Tried<EnumWriter, Error> {
  if (isSome(me.value)) {
    return Err(Error(""))
  }

  const 
}

export function asEnumUnitVariant(
  enumVariantName: string
): Json.Json {
  return enumVariantName
}

// TODO: Rename this to "asEnumDataVariantUsing".
export function asEnumDataVariant<Value>(
  enumVariantName: string, 
  enumVariantDataSerializer: JsonSerializer<Value>,
  enumVariantData: Value, 
): Json.Json {
  return {
    [enumVariantName]: enumVariantDataSerializer.toSerializable(enumVariantData)
  }
}

export function asEnumObjectVariant<Value1>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1): Json.Json;
export function asEnumObjectVariant<Value1, Value2>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2): Json.Json;
export function asEnumObjectVariant<Value1, Value2, Value3>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2, key3: string, jsonSerializer3: JsonSerializer<Value3>, value3: Value3): Json.Json;
export function asEnumObjectVariant<Value1, Value2, Value3, Value4>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2, key3: string, jsonSerializer3: JsonSerializer<Value3>, value3: Value3, key4: string, jsonSerializer4: JsonSerializer<Value4>, value4: Value4): Json.Json;
export function asEnumObjectVariant<Value1, Value2, Value3, Value4, Value5>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2, key3: string, jsonSerializer3: JsonSerializer<Value3>, value3: Value3, key4: string, jsonSerializer4: JsonSerializer<Value4>, value4: Value4, key5: string, jsonSerializer5: JsonSerializer<Value5>, value5: Value5): Json.Json;
export function asEnumObjectVariant<Value1, Value2, Value3, Value4, Value5, Value6>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2, key3: string, jsonSerializer3: JsonSerializer<Value3>, value3: Value3, key4: string, jsonSerializer4: JsonSerializer<Value4>, value4: Value4, key5: string, jsonSerializer5: JsonSerializer<Value5>, value5: Value5, key6: string, jsonSerializer6: JsonSerializer<Value6>, value6: Value6): Json.Json;
export function asEnumObjectVariant<Value1, Value2, Value3, Value4, Value5, Value6, Value7>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2, key3: string, jsonSerializer3: JsonSerializer<Value3>, value3: Value3, key4: string, jsonSerializer4: JsonSerializer<Value4>, value4: Value4, key5: string, jsonSerializer5: JsonSerializer<Value5>, value5: Value5, key6: string, jsonSerializer6: JsonSerializer<Value6>, value6: Value6, key7: string, jsonSerializer7: JsonSerializer<Value7>, value7: Value7): Json.Json;
export function asEnumObjectVariant<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2, key3: string, jsonSerializer3: JsonSerializer<Value3>, value3: Value3, key4: string, jsonSerializer4: JsonSerializer<Value4>, value4: Value4, key5: string, jsonSerializer5: JsonSerializer<Value5>, value5: Value5, key6: string, jsonSerializer6: JsonSerializer<Value6>, value6: Value6, key7: string, jsonSerializer7: JsonSerializer<Value7>, value7: Value7, key8: string, jsonSerializer8: JsonSerializer<Value8>, value8: Value8): Json.Json;
export function asEnumObjectVariant<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2, key3: string, jsonSerializer3: JsonSerializer<Value3>, value3: Value3, key4: string, jsonSerializer4: JsonSerializer<Value4>, value4: Value4, key5: string, jsonSerializer5: JsonSerializer<Value5>, value5: Value5, key6: string, jsonSerializer6: JsonSerializer<Value6>, value6: Value6, key7: string, jsonSerializer7: JsonSerializer<Value7>, value7: Value7, key8: string, jsonSerializer8: JsonSerializer<Value8>, value8: Value8, key9: string, jsonSerializer9: JsonSerializer<Value9>, value9: Value9): Json.Json;
export function asEnumObjectVariant<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10>(name: string, key1: string, jsonSerializer1: JsonSerializer<Value1>, value1: Value1, key2: string, jsonSerializer2: JsonSerializer<Value2>, value2: Value2, key3: string, jsonSerializer3: JsonSerializer<Value3>, value3: Value3, key4: string, jsonSerializer4: JsonSerializer<Value4>, value4: Value4, key5: string, jsonSerializer5: JsonSerializer<Value5>, value5: Value5, key6: string, jsonSerializer6: JsonSerializer<Value6>, value6: Value6, key7: string, jsonSerializer7: JsonSerializer<Value7>, value7: Value7, key8: string, jsonSerializer8: JsonSerializer<Value8>, value8: Value8, key9: string, jsonSerializer9: JsonSerializer<Value9>, value9: Value9, key10: string, jsonSerializer10: JsonSerializer<Value10>, value10: Value10): Json.Json;
export function asEnumObjectVariant(name: string, ...properties: unknown[]): Json.Json {
  const json = {
    [name]: Json.Object()
  }

  const length = properties.length
  for (let index = 0; index < length; index += 3) {
    const propertyName            = properties[index] as string
    const propertyValueSerializer = properties[index + 1] as JsonSerializer<unknown>
    const propertyValue           = properties[index + 2] as unknown

    json[name][propertyName] = propertyValueSerializer.toSerializable(propertyValue)
  }

  return json
}

type Impl<Value> = (
  (me: Value) => Json.Json
)

export interface JsonSerializer<Value> {
  /** Private. */
  readonly toSerializable: Impl<Value>
  readonly serialize: (me: Value) => Tried<string, Error>
}

export function implement<Value>(impl: Impl<Value>): JsonSerializer<Value> {
  function serialize(me: Value): Tried<string, Error> {
    const json = impl(me)
    
    try {
      return Ok(JSON.stringify(json))
    } catch (error) {
      return Err(Error(`Failed to serialize intermediate representation to JSON. Error: ${
        escapeQuotesAndNewLinesAndSpacesThenQuote(createString(error))
      }. Intermediate reprsentation: ${
        escapeQuotesAndNewLinesAndSpacesThenQuote(Json.displayer.display(json))
      }`))
    }

  }
  
  return Unique({
    impl,
    serialize,
  })
}

export const nullSerializer = implement<null>(() => 
  asNull()
)

export const stringSerializer = implement<string>((me) => 
  asString(me)
)

export const integerSerializer = implement<number>((me) => 
  asInteger(me)
)

export const booleanSerializer = implement<boolean>((me) =>
  asBoolean(me)
)

export const implementForArray = <Item>(itemSerializer: JsonSerializer<Item>) =>
  implement<Item[]>((me) =>
    me.map(itemSerializer.toSerializable)
  )
import * as Json from "./JaonValue.ts";
import * as Displayer from "../Display.ts"
import { Err, Ok, Tried } from "../Tried.ts";
import { Unique } from "../Unique.ts";

function escapeQuotesAndNewLinesAndSpacesThenQuote(string: string) {
  return `"${string.replaceAll(/"/ug, "\"").replaceAll(/\s/ug, "\\s").replaceAll(/\n/ug, "\\n")}"`
}

export type Error = Unique<"Discipline.Elementary.JsonSerializer.Error", {
  readonly message: string
}>

function Error(message: string): Error {
  return Unique({
    message
  })
}

export const errorDisplayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("JsonSerializeError", me.message)
)

export function as<Value>(
  jsonSerializer: JsonSerializer<Value>,
  value: Value, 
): 
  Json.Json
{
  return jsonSerializer.impl(value)
}

export function asNull(): Json.Json {
  return Json.Null()
}

export function asInteger(
  value: number,
): Json.Json {
  return Json.Number(value)
}

export function asString(
  value: string,
): Json.Json {
  return Json.String(value)
}

export function asBoolean(
  value: boolean,
): Json.Json {
  return Json.Boolean(value)
}

export function asArray<Item>(
  array: Item[], 
  itemJsonSerializer: JsonSerializer<Item>,
): Json.Json {
  return array.map(item => itemJsonSerializer.impl(item))
}

export function asObject<Value1>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1): Json.Json;
export function asObject<Value1, Value2>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2): Json.Json;
export function asObject<Value1, Value2, Value3>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2, property3Name: string, property3ValueSerializer: JsonSerializer<Value3>, property3Value: Value3): Json.Json;
export function asObject<Value1, Value2, Value3, Value4>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2, property3Name: string, property3ValueSerializer: JsonSerializer<Value3>, property3Value: Value3, property4Name: string, property4ValueSerializer: JsonSerializer<Value4>, property4Value: Value4): Json.Json;
export function asObject<Value1, Value2, Value3, Value4, Value5>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2, property3Name: string, property3ValueSerializer: JsonSerializer<Value3>, property3Value: Value3, property4Name: string, property4ValueSerializer: JsonSerializer<Value4>, property4Value: Value4, property5Name: string, property5ValueSerializer: JsonSerializer<Value5>, property5Value: Value5): Json.Json;
export function asObject<Value1, Value2, Value3, Value4, Value5, Value6>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2, property3Name: string, property3ValueSerializer: JsonSerializer<Value3>, property3Value: Value3, property4Name: string, property4ValueSerializer: JsonSerializer<Value4>, property4Value: Value4, property5Name: string, property5ValueSerializer: JsonSerializer<Value5>, property5Value: Value5, property6Name: string, property6ValueSerializer: JsonSerializer<Value6>, property6Value: Value6): Json.Json;
export function asObject<Value1, Value2, Value3, Value4, Value5, Value6, Value7>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2, property3Name: string, property3ValueSerializer: JsonSerializer<Value3>, property3Value: Value3, property4Name: string, property4ValueSerializer: JsonSerializer<Value4>, property4Value: Value4, property5Name: string, property5ValueSerializer: JsonSerializer<Value5>, property5Value: Value5, property6Name: string, property6ValueSerializer: JsonSerializer<Value6>, property6Value: Value6, property7Name: string, property7ValueSerializer: JsonSerializer<Value7>, property7Value: Value7): Json.Json;
export function asObject<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2, property3Name: string, property3ValueSerializer: JsonSerializer<Value3>, property3Value: Value3, property4Name: string, property4ValueSerializer: JsonSerializer<Value4>, property4Value: Value4, property5Name: string, property5ValueSerializer: JsonSerializer<Value5>, property5Value: Value5, property6Name: string, property6ValueSerializer: JsonSerializer<Value6>, property6Value: Value6, property7Name: string, property7ValueSerializer: JsonSerializer<Value7>, property7Value: Value7, property8Name: string, property8ValueSerializer: JsonSerializer<Value8>, property8Value: Value8): Json.Json;
export function asObject<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2, property3Name: string, property3ValueSerializer: JsonSerializer<Value3>, property3Value: Value3, property4Name: string, property4ValueSerializer: JsonSerializer<Value4>, property4Value: Value4, property5Name: string, property5ValueSerializer: JsonSerializer<Value5>, property5Value: Value5, property6Name: string, property6ValueSerializer: JsonSerializer<Value6>, property6Value: Value6, property7Name: string, property7ValueSerializer: JsonSerializer<Value7>, property7Value: Value7, property8Name: string, property8ValueSerializer: JsonSerializer<Value8>, property8Value: Value8, property9Name: string, property9ValueSerializer: JsonSerializer<Value9>, property9Value: Value9): Json.Json;
export function asObject<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10>(property1Name: string, property1ValueSerializer: JsonSerializer<Value1>, property1Value: Value1, property2Name: string, property2ValueSerializer: JsonSerializer<Value2>, property2Value: Value2, property3Name: string, property3ValueSerializer: JsonSerializer<Value3>, property3Value: Value3, property4Name: string, property4ValueSerializer: JsonSerializer<Value4>, property4Value: Value4, property5Name: string, property5ValueSerializer: JsonSerializer<Value5>, property5Value: Value5, property6Name: string, property6ValueSerializer: JsonSerializer<Value6>, property6Value: Value6, property7Name: string, property7ValueSerializer: JsonSerializer<Value7>, property7Value: Value7, property8Name: string, property8ValueSerializer: JsonSerializer<Value8>, property8Value: Value8, property9Name: string, property9ValueSerializer: JsonSerializer<Value9>, property9Value: Value9, property1Name0: string, property1ValueSerializer0: JsonSerializer<Value10>, property1Value0: Value10): Json.Json;
export function asObject(...properties: unknown[]): Json.Json {
  const json = Json.Object()

  const length = properties.length
  for (let index = 0; index < length; index += 3) {
    const propertyName            = properties[index] as string
    const propertyValueSerializer = properties[index + 1] as JsonSerializer<unknown>
    const propertyValue           = properties[index + 2] as unknown

    json[propertyName] = propertyValueSerializer.impl(propertyValue)
  }

  return json  
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
    [enumVariantName]: enumVariantDataSerializer.impl(enumVariantData)
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

    json[name][propertyName] = propertyValueSerializer.impl(propertyValue)
  }

  return json
}

type Impl<Value> = (
  (me: Value) => Json.Json
)

export interface JsonSerializer<Value> {
  /** Private. */
  readonly impl: Impl<Value>
  readonly serialize: (me: Value) => Tried<string, Error>
}

export function implement<Value>(impl: Impl<Value>): JsonSerializer<Value> {
  function serialize(me: Value): Tried<string, Error> {
    const json = impl(me)
    
    try {
      return Ok(JSON.stringify(json))
    } catch (error) {
      return Err(Error(`Failed to serialize intermediate representation to JSON. Error: ${
        escapeQuotesAndNewLinesAndSpacesThenQuote(String(error))
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
    me.map(itemSerializer.impl)
  )
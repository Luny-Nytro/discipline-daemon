import { Err, isErr, Ok, Tried } from "@Pkg/Tried";
import { isNone, isSome, None, Option, Some } from "@Pkg/Option";

type Error = null

const enum DisplayableType {
  Null,
  String,
  Number,
  Boolean,
  ValueWriter,
  EnumWriter,
  ArrayWriter,
  ObjectWriter,
  WrapperWriter,
}

type Displayable = {
  readonly type: DisplayableType.Null
} | {
  readonly type: DisplayableType.String
  readonly string: string
} | {
  readonly type: DisplayableType.Number
  readonly number: number
} | {
  readonly type: DisplayableType.Boolean
  readonly boolean: boolean
} | ObjectWriter 
  | ValueWriter
  | ArrayWriter 
  | WrapperWriter 
  | EnumWriter

type SingleValue = (
  | null
  | string
  | number
  | boolean
)

type ValueWriter = {
  readonly type: DisplayableType.ValueWriter
  value: Option<Displayable>
}

function createValueWriter(): ValueWriter {
  return {
    type: DisplayableType.ValueWriter,
    value: None()
  }
}

function writeNull(writer: ValueWriter): Tried<null, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  writer.value = Some({
    type: DisplayableType.Null,
  })

  return Ok(null)
}

function writeNumber(writer: ValueWriter, number: number): Tried<null, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  writer.value = Some({
    type: DisplayableType.Number,
    number,
  })

  return Ok(null)
}

function writeString(writer: ValueWriter, string: string): Tried<null, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  writer.value = Some({
    type: DisplayableType.String,
    string,
  })

  return Ok(null)
}

function writeBoolean(writer: ValueWriter, boolean: boolean): Tried<null, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  writer.value = Some({
    type: DisplayableType.Boolean,
    boolean,
  })

  return Ok(null)
}

function createWrapperWriterForValue(
  writer: ValueWriter, 
  wrapperName: string,
): Tried<WrapperWriter, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }
  const wrapperWriter = createWrapperWriter(wrapperName)
  writer.value = Some(wrapperWriter)
  return Ok(wrapperWriter)
}

function createArrayWriterForValue(
  me: ValueWriter, 
  arrayName?: string,
): Tried<ArrayWriter, Error> {
  if (isSome(me.value)) {
    return Err(null)
  }

  const arrayWriter = createArrayWriter(arrayName)
  me.value = Some(arrayWriter)
  return Ok(arrayWriter)
}

function createObjectWriterForValue(me: ValueWriter, objectName?: string): Tried<ObjectWriter, Error> {
  if (isSome(me.value)) {
    return Err(null)
  }
  const objectWriter = createObjectWriter(objectName)
  me.value = Some(objectWriter)
  return Ok(objectWriter)
}

function createEnumWriterForValue(me: ValueWriter, enumName: string): Tried<EnumWriter, Error> {
  if (isSome(me.value)) {
    return Err(null)
  }
  const enumWriter = createEnumWriter(enumName)
  me.value = Some(enumWriter)
  return Ok(enumWriter)
}

type WrapperWriter = {
  readonly type: DisplayableType.WrapperWriter
  readonly name: string
  value: Option<Displayable>
}

function createWrapperWriter(name: string): WrapperWriter {
  return {
    type: DisplayableType.WrapperWriter,
    name,
    value: None(),
  }
}

function writeWrappedNull(writer: WrapperWriter): Tried<null, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  writer.value = Some({
    type: DisplayableType.Null,
  })

  return Ok(null)
}

function writeWrappedNumber(writer: WrapperWriter, number: number): Tried<null, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  writer.value = Some({
    type: DisplayableType.Number,
    number,
  })

  return Ok(null)
}

function writeWrappedString(writer: WrapperWriter, string: string): Tried<null, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  writer.value = Some({
    type: DisplayableType.String,
    string,
  })

  return Ok(null)
}

function writeWrappedBoolean(writer: WrapperWriter, boolean: boolean): Tried<null, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  writer.value = Some({
    type: DisplayableType.Boolean,
    boolean,
  })

  return Ok(null)
}

function createWrapperWriterForWrappedValue(writer: WrapperWriter, wrapperName: string): Tried<WrapperWriter, Error> {
  if (isSome(writer.value)) {
    return Err(null)
  }

  const wrapperWriter = createWrapperWriter(wrapperName)
  writer.value = Some(wrapperWriter)
  return Ok(wrapperWriter)
}

function createObjectWriterForWrappedValue(me: WrapperWriter, objectName?: string): Tried<ObjectWriter, Error> {
  if (isSome(me.value)) {
    return Err(null)
  }
  const objectWriter = createObjectWriter(objectName)
  me.value = Some(objectWriter)
  return Ok(objectWriter)
}

function createArrayWriterForWrappedValue(me: WrapperWriter, arrayName?: string): Tried<ArrayWriter, Error> {
  if (isSome(me.value)) {
    return Err(null)
  }
  
  const arrayWriter = createArrayWriter(arrayName)
  me.value = Some(arrayWriter)
  return Ok(arrayWriter)
}

function createEnumWriterForWrappedValue(me: WrapperWriter, enumName: string): Tried<EnumWriter, Error> {
  if (isSome(me.value)) {
    return Err(null)
  }

  const enumWriter = createEnumWriter(enumName)
  me.value = Some(enumWriter)
  return Ok(enumWriter)
}

type ObjectWriter = {
  readonly type: DisplayableType.ObjectWriter
  readonly name: Option<string>
  readonly properties: Map<string, Displayable>
}

function createObjectWriter(name?: string): ObjectWriter {
  return {
    type: DisplayableType.ObjectWriter,
    name: Option.fromMaybeUndefined(name),
    properties: new Map()
  }
}

function writeNullProperty(me: ObjectWriter, propertyName: string): Tried<null, Error> {
  if (me.properties.has(propertyName)) {
    return Err(null)
  }

  me.properties.set(propertyName, {
    type: DisplayableType.Null,
  })

  return Ok(null)
}

function writeStringProperty(me: ObjectWriter, propertyName: string, propertyValue: string): Tried<null, Error> {
  if (me.properties.has(propertyName)) {
    return Err(null)
  }

  me.properties.set(propertyName, {
    type: DisplayableType.String,
    string: propertyValue,
  })

  return Ok(null)
}

function writeNumberProperty(me: ObjectWriter, propertyName: string, propertyValue: number): Tried<null, Error> {
  if (me.properties.has(propertyName)) {
    return Err(null)
  }

  me.properties.set(propertyName, {
    type: DisplayableType.Number,
    number: propertyValue,
  })

  return Ok(null)
}

function writeBooleanProperty(me: ObjectWriter, propertyName: string, propertyValue: boolean): Tried<null, Error> {
  if (me.properties.has(propertyName)) {
    return Err(null)
  }

  me.properties.set(propertyName, {
    type: DisplayableType.Boolean,
    boolean: propertyValue,
  })

  return Ok(null)
}

function createWrapperWriterForProperty(me: ObjectWriter, propertyName: string, wrapperName: string): Tried<WrapperWriter, Error> {
  if (me.properties.has(propertyName)) {
    return Err(null)
  }

  const wrapperWriter = createWrapperWriter(wrapperName)
  me.properties.set(propertyName, wrapperWriter)
  return Ok(wrapperWriter)
}

function createObjectWriterForProperty(me: ObjectWriter, propertyName: string, objectName?: string): Tried<ObjectWriter, Error> {
  if (me.properties.has(propertyName)) {
    return Err(null)
  }

  const objectWriter = createObjectWriter(objectName)
  me.properties.set(propertyName, objectWriter)
  return Ok(objectWriter)
}

function createArrayWriterForProperty(me: ObjectWriter, propertyName: string, arrayName?: string): Tried<ArrayWriter, Error> {
  if (me.properties.has(propertyName)) {
    return Err(null)
  }

  const arrayWriter = createArrayWriter(arrayName)
  me.properties.set(propertyName, arrayWriter)
  return Ok(arrayWriter)
}

function createEnumWriterForProperty(me: ObjectWriter, propertyName: string, enumName: string): Tried<EnumWriter, Error> {
  if (me.properties.has(propertyName)) {
    return Err(null)
  }

  const enumWriter = createEnumWriter(enumName)
  me.properties.set(propertyName, enumWriter)
  return Ok(enumWriter)
}

type ArrayWriter = {
  readonly type: DisplayableType.ArrayWriter
  readonly name: Option<string>
  readonly items: Displayable[]
}

function createArrayWriter(name?: string): ArrayWriter {
  return {
    type: DisplayableType.ArrayWriter,
    name: Option.fromMaybeUndefined(name),
    items: []
  }
}

function writeNullItem(me: ArrayWriter): Tried<null, Error> {
  me.items.push({
    type: DisplayableType.Null,
  })
  return Ok(null)
}

function writeStringItem(me: ArrayWriter, item: string): Tried<null, Error> {
  me.items.push({
    type: DisplayableType.String,
    string: item
  })
  return Ok(null)
}

function writeNumberItem(me: ArrayWriter, item: number): Tried<null, Error> {
  me.items.push({
    type: DisplayableType.Number,
    number: item,
  })
  return Ok(null)
}

function writeBooleanItem(me: ArrayWriter, item: boolean): Tried<null, Error> {
  me.items.push({
    type: DisplayableType.Boolean,
    boolean: item
  })
  return Ok(null)
}

function writeItemGivenInspector<Item>(
  me: ArrayWriter, 
  item: Item, 
  inspector: Inspector<Item>,
): Tried<null, Error> {
  const valueWriter = createValueWriter()
  const maybeError = inspector.write(item, valueWriter)
  if (isErr(maybeError)) {
    return maybeError
  }

  if (isNone(valueWriter.value)) {
    return Err(null)
  }

  me.items.push(Option.value(valueWriter.value))
  return Ok(null)
}

function writeItemsGivenInspector<Item>(
  me: ArrayWriter, 
  items: Item[], 
  inspector: Inspector<Item>,
): Tried<null, Error> {
  for (const item of items) {
    const valueWriter = createValueWriter()
    const maybeError = inspector.write(item, valueWriter)
    if (isErr(maybeError)) {
      return maybeError
    }

    if (isNone(valueWriter.value)) {
      return Err(null)
    }

    me.items.push(Option.value(valueWriter.value))
  }

  return Ok(null)
}

function createObjectWriterForItem(me: ArrayWriter, objectName?: string): Tried<ObjectWriter, Error> {
  const objectWriter = createObjectWriter(objectName)
  me.items.push(objectWriter)
  return Ok(objectWriter)
}

function createArrayWriterForItem(me: ArrayWriter, arrayName?: string): Tried<ArrayWriter, Error> {
  const arrayWriter = createArrayWriter(arrayName)
  me.items.push(arrayWriter)
  return Ok(arrayWriter)
}

function createWrapperWriterForItem(me: ArrayWriter, wrapperName: string): Tried<WrapperWriter, Error> {
  const wrapperWriter = createWrapperWriter(wrapperName)
  me.items.push(wrapperWriter)
  return Ok(wrapperWriter)
}

function createEnumWriterForItem(me: ArrayWriter, enumName: string): Tried<EnumWriter, Error> {
  const enumWriter = createEnumWriter(enumName)
  me.items.push(enumWriter)
  return Ok(enumWriter)
}

interface EnumWriter {
  readonly type: DisplayableType.EnumWriter
  readonly enumName: string
  enumVariant: Option<{
    readonly name: string
    readonly data: Option<Displayable>
  }>
}

function createEnumWriter(enumName: string): EnumWriter {
  return {
    type: DisplayableType.EnumWriter,
    enumName,
    enumVariant: None(),
  }
}

function writeEnumUnitVariant(me: EnumWriter, variantName: string): Tried<null, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  me.enumVariant = Some({
    name: variantName,
    data: None(),
  })

  return Ok(null)
}

function writeEnumNullVariant(me: EnumWriter, variantName: string): Tried<null, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  me.enumVariant = Some({
    name: variantName,
    data: Some({
      type: DisplayableType.Null
    }),
  })

  return Ok(null)
}

function writeEnumStringVariant(me: EnumWriter, variantName: string, variantData: string): Tried<null, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  me.enumVariant = Some({
    name: variantName,
    data: Some({
      type: DisplayableType.String,
      string: variantData,
    }),
  })

  return Ok(null)
}

function writeEnumNumberVariant(me: EnumWriter, variantName: string, variantData: number): Tried<null, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  me.enumVariant = Some({
    name: variantName,
    data: Some({
      type: DisplayableType.Number,
      number: variantData,
    }),
  })

  return Ok(null)
}

function writeEnumBooleanVariant(me: EnumWriter, variantName: string, variantData: boolean): Tried<null, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  me.enumVariant = Some({
    name: variantName,
    data: Some({
      type: DisplayableType.Boolean,
      boolean: variantData,
    }),
  })

  return Ok(null)
}

function writeEnumDataVariantGivenDisplayer<Data>(
  me: EnumWriter, 
  variantName: string, 
  variantData: Data,
  inspector: Inspector<Data>,
): Tried<null, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  const valueWriter = createValueWriter()
  const maybeError = inspector.write(variantData, valueWriter)
  if (isErr(maybeError)) {
    return maybeError
  }
  if (isNone(valueWriter.value)) {
    return Err(null)
  }

  me.enumVariant = Some({
    name: variantName,
    data: valueWriter.value,
  })

  return Ok(null)
}

function createWrapperWriterForEnumVariant(me: EnumWriter, variantName: string, wrapperName: string): Tried<WrapperWriter, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  const wrapperWriter = createWrapperWriter(wrapperName)
  me.enumVariant = Some({
    name: variantName,
    data: Some(wrapperWriter),
  })

  return Ok(wrapperWriter)
}

function createArrayWriterForEnumVariant(me: EnumWriter, variantName: string, arrayName?: string): Tried<ArrayWriter, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  const arrayWriter = createArrayWriter(arrayName)
  me.enumVariant = Some({
    name: variantName,
    data: Some(arrayWriter),
  })
  
  return Ok(arrayWriter)
}

function createObjectWriterForEnumVariant(me: EnumWriter, variantName: string, objectName?: string): Tried<ObjectWriter, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  const objectWriter = createObjectWriter(objectName)
  me.enumVariant = Some({
    name: variantName,
    data: Some(objectWriter),
  })
  
  return Ok(objectWriter)
}

function createEnumWriterForEnumVariant(me: EnumWriter, variantName: string, innerEnumName: string): Tried<EnumWriter, Error> {
  if (isSome(me.enumVariant)) {
    return Err(null)
  }

  const enumWriter = createEnumWriter(innerEnumName)
  me.enumVariant = Some({
    name: variantName,
    data: Some(enumWriter),
  })
  
  return Ok(enumWriter)
}

function createIndentation(level: number) {
  return "  ".repeat(level)
}

function displayNull() {
  return "Null"
}

function displayString(string: string) {
  return `"${string
    // .replaceAll(/\s/ug, "\\s")
    // .replaceAll(/\n/ug, "\\n")
    .replaceAll(/"/ug, "\\\"")
  }"`
}

function displayNumber(number: number) {
  return number.toString()
}

function displayBoolean(boolean: boolean) {
  return boolean ? "True" : "False"
}

function displayArray(level: number, { name, items }: ArrayWriter): Tried<string, Error> {
  const length = items.length
  if (length === 0) {
    return Ok(Option.match(name, {
      None: () => "[]",
      Some: (name) => `${name} []`
    }))
  }

  let output = Option.match(name, {
    None: () => "[\n",
    Some: (name) => `${name} [\n`
  })

  let index = 0
  
  const elementIndentation = createIndentation(level + 1)
  const closingIndentation = createIndentation(level)
  const elementLevel = level + 1

  while (true) {
    const element = items[index]
    output += elementIndentation
    
    const maybeStringRepr = display(elementLevel, element)
    if (isErr(maybeStringRepr)) {
      return maybeStringRepr
    }

    output += Tried.value(maybeStringRepr)
    
    index += 1
    if (index < length) {
      output += ",\n"
      continue
    }

    output += "\n"
    output += closingIndentation
    output += "]"
    return Ok(output)
  }
}

function displayObject(
  level: number, 
  { name, properties }: ObjectWriter,
): Tried<string, Error> {
  const size = properties.size
  if (size === 0) {
    return Ok(Option.match(name, {
      None: () => `{}`,
      Some: name => `${name} {}`
    }))
  }

  let output = Option.match(name, {
    None: () => `{\n`,
    Some: name => `${name} {\n`
  })

  let index = 0

  const propertyIndentation = createIndentation(level + 1)
  const closingIndentation = createIndentation(level)
  const propertyLevel = level + 1

  for (const [ name, value ] of properties) {
    output += propertyIndentation

    const maybeStringRepr = display(propertyLevel, value)
    if (isErr(maybeStringRepr)) {
      return maybeStringRepr
    }

    output += `${name}: ${Tried.value(maybeStringRepr)}`
  
    index += 1
    if (index < size) {
      output += ",\n"
      continue
    }
  
    output += "\n"
    output += closingIndentation
    output += "}"
    return Ok(output)
  }

  // TODO: Return an error instead 
  throw "unreachable code"
}

function displayEnum(
  level: number,
  enumWriter: EnumWriter
): Tried<string, Error> {
  let output = enumWriter.enumName

  if (isNone(enumWriter.enumVariant)) {
    return Err(null)
  }

  const variant = Option.value(enumWriter.enumVariant)
  
  output += `.${variant.name}`

  if (isSome(variant.data)) {
    output += `(${display(level, Option.value(variant.data))})`
  }

  return Ok(output)
}

function displayWrapper(
  level: number,
  wrapperWriter: WrapperWriter,
): Tried<string, Error> {
  if (isNone(wrapperWriter.value)) {
    return Err(null)
  }
  
  return Ok(
    `${
      wrapperWriter.name
    }(${
      display(level, Option.value(wrapperWriter.value))
    })`
  )
}

function displayValue(level: number, value: ValueWriter): Tried<string, Error> {
  if (isSome(value.value)) {
    return display(level, Option.value(value.value))
  } else {
    return Err(null)
  }
}

function display(level: number, value: Displayable): Tried<string, Error> {
  switch (value.type) {
    case DisplayableType.Null: {
      return Ok(displayNull())
    }
    case DisplayableType.String: {
      return Ok(displayString(value.string))
    }
    case DisplayableType.Number: {
      return Ok(displayNumber(value.number))
    }
    case DisplayableType.Boolean: {
      return Ok(displayBoolean(value.boolean))
    }
    case DisplayableType.ArrayWriter: {
      return displayArray(level, value)
    }
    case DisplayableType.ObjectWriter: {
      return displayObject(level, value)
    }
    case DisplayableType.EnumWriter: {
      return displayEnum(level, value)
    }
    case DisplayableType.WrapperWriter: {
      return displayWrapper(level, value)
    }
    case DisplayableType.ValueWriter: {
      return displayValue(level, value)
    }
  }
}

export type Inspector<Value> =  {
  write(value: Value, writer: ValueWriter): Tried<unknown, Error>
  display(value: Value): Tried<string, Error>
}

export function implement<Value>(
  write: Inspector<Value>["write"]
): 
  Inspector<Value>
{
  return {
    write,
    display(value) {
      const valueWriter = createValueWriter()
      
      const maybeError = write(value, valueWriter)
      if (isErr(maybeError)) {
        return maybeError
      }

      return display(0, valueWriter)
    }
  }
}

implement<string>((value, writer) => 
  Tried.andThen(createWrapperWriterForValue(writer, "Name"), writer => 
    writeWrappedString(writer, value)
  )
)

implement<{ light: boolean, vibe: "yes" }>((value, writer) => 
  Tried.andThen(createObjectWriterForValue(writer, ""), writer => Tried.and2(
    writeBooleanProperty(writer, "light", value.light),
    writeStringProperty(writer, "vibe", value.vibe),
  ))
)

// implement<Tried<boolean, boolean>>((value, writer) => 
//   Tried.andThen(ValueWriter.createEnumWriter(writer, "Tried"), writer => Tried.andth)
// )

export const Inspection = {
  writeNull,
  writeNumber,
  writeString,
  writeBoolean,
  createWrapperWriterForValue,
  createArrayWriterForValue,
  createObjectWriterForValue,
  createEnumWriterForValue,
  createWrapperWriter,
  writeWrappedNull,
  writeWrappedNumber,
  writeWrappedString,
  writeWrappedBoolean,
  createWrapperWriterForWrappedValue,
  createObjectWriterForWrappedValue,
  createArrayWriterForWrappedValue,
  createEnumWriterForWrappedValue,
  createObjectWriter,
  writeNullProperty,
  writeStringProperty,
  writeNumberProperty,
  writeBooleanProperty,
  createWrapperWriterForProperty,
  createObjectWriterForProperty,
  createArrayWriterForProperty,
  createEnumWriterForProperty,
  createArrayWriter,
  writeNullItem,
  writeStringItem,
  writeNumberItem,
  writeBooleanItem,
  writeItemGivenInspector,
  writeItemsGivenInspector,
  createObjectWriterForItem,
  createArrayWriterForItem,
  createWrapperWriterForItem,
  createEnumWriterForItem,
  createEnumWriter,
  writeEnumUnitVariant,
  writeEnumNullVariant,
  writeEnumStringVariant,
  writeEnumNumberVariant,
  writeEnumBooleanVariant,
  writeEnumDataVariantGivenDisplayer,
  createWrapperWriterForEnumVariant,
  createArrayWriterForEnumVariant,
  createObjectWriterForEnumVariant,
  createEnumWriterForEnumVariant,
  displayNull,
  displayString,
  displayNumber,
  displayBoolean,
}


// Tried.isOk1ThenCall1(
//   JsonSerialization.createObjectWriter(writer),
//   writer => Tried.areOk3ThenCall1(
//     JsonSerialization.writeObjectProperty(writer, "id", Uuid.jsonSerializer),
//     JsonSerialization.writeObjectProperty(writer, "enabledCountdownTimer", CountdownTimer.jsonSerializer),
//     JsonSerialization.writeObjectProperty(writer, "activator", Activator.jsonSerializer),
//     Rule.create,   
//   )
// )

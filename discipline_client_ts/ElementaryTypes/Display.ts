// This implementation doesn't take into account circular references:
// if the value to display contains circular references, the outcome 
// is undefined.

const enum DisplayableType {
  Null,
  String,
  Number,
  Boolean,
  Array,
  NamedObject,
  UnnamedObject,
  EnumUnitVariant,
  EnumDataVariant,
  Wrapper,
}

export type Displayable = {
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
} | {
  readonly type: DisplayableType.Array
  readonly array: Displayable[]
} | {
  readonly type: DisplayableType.NamedObject
  readonly name: string
  readonly properties: [string, Displayable][]
} | {
  readonly type: DisplayableType.UnnamedObject
  readonly properties: [string, Displayable][]
} | {
  readonly type: DisplayableType.Wrapper
  readonly name: string
  readonly value: Displayable
} | {
  readonly type: DisplayableType.EnumUnitVariant
  readonly enumName: string
  readonly enumVariantName: string
} | {
  readonly type: DisplayableType.EnumDataVariant
  readonly enumName: string
  readonly enumVariantName: string
  readonly enumVariantData: Displayable
}

export function asNull(): Displayable {
  return {
    type: DisplayableType.Null
  }
}

export function asNumber(number: number): Displayable {
  return {
    type: DisplayableType.Number,
    number,
  }
}

export function asString(string: string): Displayable {
  return {
    type: DisplayableType.String,
    string,
  }
}

export function asBoolean(boolean: boolean): Displayable {
  return {
    type: DisplayableType.Boolean,
    boolean,
  }
}

export function asArray<Element>(
  elementDisplayer: Displayer<Element>,
  array: Element[]
): Displayable {
  return {
    type: DisplayableType.Array,
    array: array.map(elementDisplayer.impl),
  }
}

export function asNamedObject<Property1>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1): Displayable;
export function asNamedObject<Property1, Property2>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2): Displayable;
export function asNamedObject<Property1, Property2, Property3>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3): Displayable;
export function asNamedObject<Property1, Property2, Property3, Property4>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4): Displayable;
export function asNamedObject<Property1, Property2, Property3, Property4, Property5>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5): Displayable;
export function asNamedObject<Property1, Property2, Property3, Property4, Property5, Property6>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6): Displayable;
export function asNamedObject<Property1, Property2, Property3, Property4, Property5, Property6, Property7>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6, propertyName7: string, propertyValueDisplayer7: Displayer<Property7>, propertyValue7: Property7): Displayable;
export function asNamedObject<Property1, Property2, Property3, Property4, Property5, Property6, Property7, Property8>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6, propertyName7: string, propertyValueDisplayer7: Displayer<Property7>, propertyValue7: Property7, propertyName8: string, propertyValueDisplayer8: Displayer<Property8>, propertyValue8: Property8): Displayable;
export function asNamedObject<Property1, Property2, Property3, Property4, Property5, Property6, Property7, Property8, Property9>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6, propertyName7: string, propertyValueDisplayer7: Displayer<Property7>, propertyValue7: Property7, propertyName8: string, propertyValueDisplayer8: Displayer<Property8>, propertyValue8: Property8, propertyName9: string, propertyValueDisplayer9: Displayer<Property9>, propertyValue9: Property9): Displayable;
export function asNamedObject<Property1, Property2, Property3, Property4, Property5, Property6, Property7, Property8, Property9, Property10>(name: string, propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6, propertyName7: string, propertyValueDisplayer7: Displayer<Property7>, propertyValue7: Property7, propertyName8: string, propertyValueDisplayer8: Displayer<Property8>, propertyValue8: Property8, propertyName9: string, propertyValueDisplayer9: Displayer<Property9>, propertyValue9: Property9, propertyName10: string, propertyValueDisplayer10: Displayer<Property10>, propertyValue10: Property10): Displayable;
export function asNamedObject(name: string, ...args: unknown[]): Displayable {
  const properties: [string, Displayable][] = []
  
  for (let i = 0; i < args.length; i += 3) {
    const propertyName           = args[i] as string
    const propertyValueDisplayer = args[i + 1] as Displayer<unknown>
    const propertyValue          = args[i + 2] as unknown
    properties.push([ 
      propertyName, 
      propertyValueDisplayer.impl(propertyValue),
    ])
  }

  return {
    type: DisplayableType.NamedObject,
    name,
    properties,
  }
}

export function asUnnamedObjectX<Property>(
  propertyValueDisplayer: Displayer<Property>,
  object: Record<string, Property>,
): Displayable {
  return {
    type: DisplayableType.UnnamedObject,
    properties: globalThis
    .Object
    .entries(object)
    .map(([ name, value ]) => [ name, propertyValueDisplayer.impl(value) ]),
  }
}

export function asUnnamedObject<Property1>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1): Displayable;
export function asUnnamedObject<Property1, Property2>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2): Displayable;
export function asUnnamedObject<Property1, Property2, Property3>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3): Displayable;
export function asUnnamedObject<Property1, Property2, Property3, Property4>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4): Displayable;
export function asUnnamedObject<Property1, Property2, Property3, Property4, Property5>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5): Displayable;
export function asUnnamedObject<Property1, Property2, Property3, Property4, Property5, Property6>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6): Displayable;
export function asUnnamedObject<Property1, Property2, Property3, Property4, Property5, Property6, Property7>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6, propertyName7: string, propertyValueDisplayer7: Displayer<Property7>, propertyValue7: Property7): Displayable;
export function asUnnamedObject<Property1, Property2, Property3, Property4, Property5, Property6, Property7, Property8>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6, propertyName7: string, propertyValueDisplayer7: Displayer<Property7>, propertyValue7: Property7, propertyName8: string, propertyValueDisplayer8: Displayer<Property8>, propertyValue8: Property8): Displayable;
export function asUnnamedObject<Property1, Property2, Property3, Property4, Property5, Property6, Property7, Property8, Property9>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6, propertyName7: string, propertyValueDisplayer7: Displayer<Property7>, propertyValue7: Property7, propertyName8: string, propertyValueDisplayer8: Displayer<Property8>, propertyValue8: Property8, propertyName9: string, propertyValueDisplayer9: Displayer<Property9>, propertyValue9: Property9): Displayable;
export function asUnnamedObject<Property1, Property2, Property3, Property4, Property5, Property6, Property7, Property8, Property9, Property10>(propertyName1: string, propertyValueDisplayer1: Displayer<Property1>, propertyValue1: Property1, propertyName2: string, propertyValueDisplayer2: Displayer<Property2>, propertyValue2: Property2, propertyName3: string, propertyValueDisplayer3: Displayer<Property3>, propertyValue3: Property3, propertyName4: string, propertyValueDisplayer4: Displayer<Property4>, propertyValue4: Property4, propertyName5: string, propertyValueDisplayer5: Displayer<Property5>, propertyValue5: Property5, propertyName6: string, propertyValueDisplayer6: Displayer<Property6>, propertyValue6: Property6, propertyName7: string, propertyValueDisplayer7: Displayer<Property7>, propertyValue7: Property7, propertyName8: string, propertyValueDisplayer8: Displayer<Property8>, propertyValue8: Property8, propertyName9: string, propertyValueDisplayer9: Displayer<Property9>, propertyValue9: Property9, propertyName10: string, propertyValueDisplayer10: Displayer<Property10>, propertyValue10: Property10): Displayable;
export function asUnnamedObject(...args: unknown[]): Displayable {
  const properties: [string, Displayable][] = []
  
  for (let i = 0; i < args.length; i += 3) {
    const propertyName           = args[i] as string
    const propertyValueDisplayer = args[i + 1] as Displayer<unknown>
    const propertyValue          = args[i + 2] as unknown
    properties.push([ 
      propertyName, 
      propertyValueDisplayer.impl(propertyValue),
    ])
  }

  return {
    type: DisplayableType.UnnamedObject,
    properties,
  }
}

export function asWrappedNull(name: string): Displayable {
  return {
    type: DisplayableType.Wrapper,
    name,
    value: asNull()
  }
}

export function asWrappedNumber(name: string, number: number): Displayable {
  return {
    type: DisplayableType.Wrapper,
    name: name,
    value: asNumber(number)
  }
}

export function asWrappedString(name: string, string: string): Displayable {
  return {
    type: DisplayableType.Wrapper,
    name,
    value: asString(string)
  }
}

export function asWrappedBoolean(name: string, boolean: boolean): Displayable {
  return {
    type: DisplayableType.Wrapper,
    name,
    value: asBoolean(boolean)
  }
}

export function asWrappedArray<Element>(
  name: string,
  elementDisplayer: Displayer<Element>,
  array: Element[]
): Displayable {
  return {
    type: DisplayableType.Wrapper,
    name,
    value: asArray(elementDisplayer, array)
  }
}

export function asWrappedUsing<Value>(
  name: string, 
  valueDisplayer: Displayer<Value>,
  value: Value
): Displayable {
  return {
    type: DisplayableType.Wrapper,
    name,
    value: valueDisplayer.impl(value)
  }
}

export function asWrapped(name: string, value: Displayable): Displayable {
  return {
    type: DisplayableType.Wrapper,
    name,
    value,
  }
}
// Displayer.wrappedNamedObject()
// Displayer.wrappedUnnamedObject()

export function asEnumUnitVariant(enumName: string, enumVariantName: string): Displayable {
  return {
    type: DisplayableType.EnumUnitVariant,
    enumName,
    enumVariantName
  }
}

export function asEnumNullVariant(enumName: string, enumVariantName: string): Displayable {
  return {
    type: DisplayableType.EnumDataVariant,
    enumName,
    enumVariantName,
    enumVariantData: asNull()
  }
}

export function asEnumStringVariant(
  enumName: string, 
  enumVariantName: string,
  enumVariantData: string,
): Displayable {
  return {
    type: DisplayableType.EnumDataVariant,
    enumName,
    enumVariantName,
    enumVariantData: {
      type: DisplayableType.String,
      string: enumVariantData,
    }
  }
}

export function asEnumNumberVariant(
  enumName: string, 
  enumVariantName: string,
  enumVariantData: number,
): Displayable {
  return {
    type: DisplayableType.EnumDataVariant,
    enumName,
    enumVariantName,
    enumVariantData: {
      type: DisplayableType.Number,
      number: enumVariantData,
    }
  }
}

export function asEnumBooleanVariant(
  enumName: string, 
  enumVariantName: string,
  enumVariantData: boolean,
): Displayable {
  return {
    type: DisplayableType.EnumDataVariant,
    enumName,
    enumVariantName,
    enumVariantData: {
      type: DisplayableType.Boolean,
      boolean: enumVariantData,
    }
  }
}

export function asEnumAraryVariant<Element>(
  enumName: string, 
  enumVariantName: string,
  enumVariantDataElementDisplayer: Displayer<Element>,
  enumVariantData: Element[],
): Displayable {
  return {
    type: DisplayableType.EnumDataVariant,
    enumName,
    enumVariantName,
    enumVariantData: {
      type: DisplayableType.Array,
      array: enumVariantData.map(enumVariantDataElementDisplayer.impl),
    }
  }
}

export function asEnumDataVariantUsing<Data>(
  enumName: string, 
  enumVariantName: string,
  enumVariantDataDisplayer: Displayer<Data>,
  enumVariantData: Data,
): Displayable {
  return {
    type: DisplayableType.EnumDataVariant,
    enumName,
    enumVariantName,
    enumVariantData: enumVariantDataDisplayer.impl(enumVariantData)
  }
}

export function asEnumDataVariant(
  enumName: string, 
  enumVariantName: string,
  enumVariantData: Displayable,
): Displayable {
  return {
    type: DisplayableType.EnumDataVariant,
    enumName,
    enumVariantName,
    enumVariantData,
  }
}

function createIndentation(level: number) {
  return "  ".repeat(level)
}

export function displayNull() {
  return "Null"
}
export function displayString(string: string) {
  return `"${string
    // .replaceAll(/\s/ug, "\\s")
    // .replaceAll(/\n/ug, "\\n")
    .replaceAll(/"/ug, "\\\"")
  }"`
}
export function displayNumber(number: number) {
  return number.toString()
}
export function displayBoolean(boolean: boolean) {
  return boolean ? "True" : "False"
}
function displayArray(level: number, array: Displayable[]) {
  const length = array.length
  if (length === 0) {
    return "[]"
  }

  let index = 0
  let output = "[\n"
  const elementIndentation = createIndentation(level + 1)
  const closingIndentation = createIndentation(level)
  const elementLevel = level + 1

  while (true) {
    const element = array[index]
    output += elementIndentation
    output += display(elementLevel, element)
    
    index += 1
    if (index < length) {
      output += ",\n"
      continue
    }

    output += "\n"
    output += closingIndentation
    output += "]"
    return output
  }
}
function displayUnnamedObject(
  level: number, 
  properties: [string, Displayable][],
) {
  const length = properties.length
  if (length === 0) {
    return "{}"
  }

  let index = 0
  let output = "{\n"
  const propertyIndentation = createIndentation(level + 1)
  const closingIndentation = createIndentation(level)
  const propertyLevel = level + 1

  while (true) {
    const [ name, value ] = properties[index]
    output += propertyIndentation
    output += `${name}: ${display(propertyLevel, value)}`

    index += 1
    if (index < length) {
      output += ",\n"
      continue
    }

    output += "\n"
    output += closingIndentation
    output += "}"
    return output
  }
}
function displayNamedObject(
  level: number, 
  name: string, 
  properties: [string, Displayable][],
) {
  const length = properties.length
  if (length === 0) {
    return `${name} {}`
  }

  let index = 0
  let output = `${name} {\n`
  const propertyIndentation = createIndentation(level + 1)
  const closingIndentation = createIndentation(level)
  const propertyLevel = level + 1

  while (true) {
    const [ name, value ] = properties[index]
    output += propertyIndentation
    output += `${name}: ${display(propertyLevel, value)}`

    index += 1
    if (index < length) {
      output += ",\n"
      continue
    }

    output += "\n"
    output += closingIndentation
    output += "}"
    return output
  }
}
function displayEnumUnitVariant(
  enumName: string, 
  enumVariantName: string,
) {
  return `${enumName}.${enumVariantName}`
}
function displayEnumDataVariant(
  level: number,
  enumName: string,
  enumVariantName: string,
  enumVariantData: Displayable,
) {
  return `${enumName}.${enumVariantName}(${display(level, enumVariantData)})`
}
function displayWrapper(
  level: number,
  wrapperName: string,
  wrapperValue: Displayable,
) {
  return `${wrapperName}(${display(level, wrapperValue)})`
}
function display(level: number, value: Displayable): string {
  switch (value.type) {
    case DisplayableType.Null: {
      return displayNull()
    }
    case DisplayableType.String: {
      return displayString(value.string)
    }
    case DisplayableType.Number: {
      return displayNumber(value.number)
    }
    case DisplayableType.Boolean: {
      return displayBoolean(value.boolean)
    }
    case DisplayableType.Array: {
      return displayArray(level, value.array)
    }
    case DisplayableType.NamedObject: {
      return displayNamedObject(level, value.name, value.properties)
    }
    case DisplayableType.UnnamedObject: {
      return displayUnnamedObject(level, value.properties)
    }
    case DisplayableType.EnumUnitVariant: {
      return displayEnumUnitVariant(
        value.enumName, 
        value.enumVariantName,
      )
    }
    case DisplayableType.EnumDataVariant: {
      return displayEnumDataVariant(
        level,
        value.enumName, 
        value.enumVariantName, 
        value.enumVariantData,
      )
    }
    case DisplayableType.Wrapper: {
      return displayWrapper(
        level,
        value.name,
        value.value,
      )
    }
  }
}

export type Displayer<Me> =  {
  impl(value: Me): Displayable
  display(value: Me): string
}

export function implement<Value>(
  impl: (value: Value) => Displayable
): 
  Displayer<Value>
{
  return {
    impl,
    display(value) {
      return display(0, impl(value))
    }
  }
}

export function implementForArray<Element>(
  elementDisplayer: Displayer<Element>
):
  Displayer<Element[]>
{
  return implement<Element[]>(value => asArray(elementDisplayer, value))
}

export const nullDisplayer = implement<null>(() => 
  asNull()
)
export const stringDisplayer = implement<string>(me => 
  asString(me)
)
export const numberDisplayer = implement<number>(me => 
  asNumber(me)
)
export const booleanDisplayer = implement<boolean>(me => 
  asBoolean(me)
)
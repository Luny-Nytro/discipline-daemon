import * as Uuid from "../../ElementaryTypes/Uuid.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as Enabler from "./Enabler.ts"
import * as Activator from "./Activator.ts"
import * as DateTime from "../../ChronicTypes/DateTime.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { isErr, Ok, Tried } from "../../ElementaryTypes/Tried.ts";

export type Rule = Unique<"App.NetworkingAccess.Rule", {
  readonly id: Uuid.Uuid
  enabler: Enabler.Enabler
  activator: Activator.Activator
}>

export function constructor(
  id: Uuid.Uuid, 
  enabler: Enabler.Enabler, 
  activator: Activator.Activator,
): Rule {
  return Unique({
    id,
    enabler,
    activator,
  })
}

export function isEnabled(me: Rule, now: DateTime.DateTime): boolean {
  return Enabler.isEffective(me.enabler, now)
}

export function isEffective(me: Rule, now: DateTime.DateTime): boolean {
  return Activator.isEffective(me.activator, now) && isEnabled(me, now)
}

export const displayer = Displayer.implement<Rule>(me => {

}
  // Displayer.asNamedObject("Rule",
    // "id", Uuid.displayer, me.id,
    // "enabler", Enabler.displayer, me.enabler,
    // "activator", Activator.displayer, me.activator,
  // )
)

export const jsonSerializer = JsonSerializer.implement<Rule>(me => 
  JsonSerializer.asObject(
    "id", Uuid.jsonSerializer, me.id,
    "enabler", Enabler.jsonSerializer, me.enabler,
    "activator", Activator.jsonSerializer, me.activator,
  )
)

const JsonSerialization = null as any

export const jsonSerializer_ = JsonSerializer.implement<Rule>((writer, value) => {
  const maybeRuleWriter = JsonSerialization.Writer.createObjectWriter()
  if (isErr(maybeRuleWriter)) {
    return maybeRuleWriter
  }

  const ruleWriter = Tried.value(maybeRuleWriter)

  const maybeError1 = JsonSerialization.ObjectWriter.writeProperty(ruleWriter, "id", Uuid.jsonSerializer)
  if (isErr(maybeError1)) {
    return maybeError1
  }
  
  const maybeError2 = JsonSerialization.ObjectWriter.writeProperty(ruleWriter, "enabler", Enabler.jsonSerializer)
  if (isErr(maybeError2)) {
    return maybeError2
  }
  
  const maybeError3 = JsonSerialization.ObjectWriter.writeProperty(ruleWriter, "activator", Activator.jsonSerializer)
  if (isErr(maybeError3)) {
    return maybeError3
  }

  return Ok(null)
})

export const jsonDeserializer = JsonDeserializer.implement<Rule>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
    JsonDeserializer.propertyAs(context, "id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "enabler", Enabler.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "activator", Activator.jsonDeserializer),
    constructor
  ))
)
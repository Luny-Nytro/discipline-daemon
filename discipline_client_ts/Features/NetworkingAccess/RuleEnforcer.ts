import * as Rule from "./Rule.ts"
import * as DateTime from "../../ChronicTypes/DateTime.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as OperatingSystemUsername from "../../CommonTypes/OperatingSystemUsername.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type RuleEnforcer = Unique<"Discipline.NetworkingAccess.RuleEnforcer", {
  readonly rules: Rule.Rule[]
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly isBlocked: boolean
  readonly isEnabled: boolean
}>

export function constructor(
  rules: Rule.Rule[],
  username: OperatingSystemUsername.OperatingSystemUsername,
  isBlocked: boolean,
  isEnabled: boolean,
): RuleEnforcer {
  return Unique({
    rules,
    username,
    isBlocked,
    isEnabled,
  })
}

// TODO: Delete this as a rule enforcer is created by the service 
//       and the client only deserializes them when retrieved from
//       the service, so, "constructor", should be enough.
// function create(
//   username: OperatingSystemUsername.OperatingSystemUsername, 
// ): RuleEnforcer {
//   return Unique({
//     rules: [],
//     username,
//     isBlocked: false,
//     isEnabled: false,
//   })
// }

export function areSomeRulesEnbled(me: RuleEnforcer, now: DateTime.DateTime): boolean {
  return me.rules.some(rule => Rule.isEnabled(rule, now))
}

const ruleArrayDisplayer = Displayer.implementForArray(
  Rule.displayer,
)

export const displayer = Displayer.implement<RuleEnforcer>(me => 
  Displayer.asNamedObject("RuleEnforcer", 
    "rules", ruleArrayDisplayer, me.rules,
    "username", OperatingSystemUsername.displayer, me.username,
    "isBlocked", Displayer.booleanDisplayer, me.isBlocked,
    "isEnabled", Displayer.booleanDisplayer, me.isEnabled,
  )
)

const ruleArrayJsonSerializer = JsonSerializer.implementForArray(
  Rule.jsonSerializer
)

export const jsonSerializer = JsonSerializer.implement<RuleEnforcer>(me => 
  JsonSerializer.asObject(
    "rules", ruleArrayJsonSerializer, me.rules,
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "is_blocked", JsonSerializer.booleanSerializer, me.isBlocked, 
    "is_enabled", JsonSerializer.booleanSerializer, me.isEnabled, 
  )
)

const ruleArrayJsonDeserializer = JsonDeserializer.implementForArray(
  Rule.jsonDeserializer,
)

export const jsonDeserializer = JsonDeserializer.implement<RuleEnforcer>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map4(
    JsonDeserializer.propertyAs(context, "rules", ruleArrayJsonDeserializer),
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAsBoolean(context, "is_blocked"),
    JsonDeserializer.propertyAsBoolean(context, "is_enabled"),
    constructor
  ))
)
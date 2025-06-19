import { Unique } from "@Pkg/Unique";
import { Rule } from "./Rule.ts";

export type Blocker = Unique<"App.UserAccess.Blocker", {
  isUserAccessBlocked: boolean
  isBlockingEnabled: boolean
  rules: Rule[]  
}>

export const Blocker = {
  constructor(
    rules: Rule[],
    isUserAccessBlocked: boolean,
    isBlockingEnabled: boolean,
  ): Blocker {
    return Unique({
      rules,
      isBlockingEnabled,
      isUserAccessBlocked,
    })
  }
}

// TODO: Delete this as a rule enforcer is created by the service 
//       and the client only deserializes them when retrieved from
//       the service, so, "constructor", should be enough.
// function create(
//   username: OperatingSystemUsername.OperatingSystemUsername, 
//   password: OperatingSystemPassword.OperatingSystemPassword,
// ): RuleEnforcer {
//   return Unique({
//     rules: [],
//     username,
//     password,
//     isBlocked: false,
//     isEnabled: false,
//   })
// }

// const ruleArrayDisplayer = Displayer.implementForArray(
//   Rule.displayer,
// )

// export const displayer = Displayer.implement<RuleEnforcer>(me => 
//   Displayer.asNamedObject("RuleEnforcer", 
//     "rules", ruleArrayDisplayer, me.rules,
//     "username", OperatingSystemUsername.displayer, me.username,
//     "password", OperatingSystemPassword.displayer, me.password,
//     "isBlocked", Displayer.booleanDisplayer, me.isBlocked,
//     "isEnabled", Displayer.booleanDisplayer, me.isEnabled,
//   )
// )

// const ruleArrayJsonSerializer = JsonSerializer.implementForArray(
//   Rule.jsonSerializer
// )

// export const jsonSerializer = JsonSerializer.implement<RuleEnforcer>(me => 
//   JsonSerializer.asObject(
//     "rules", ruleArrayJsonSerializer, me.rules,
//     "username", OperatingSystemUsername.jsonSerializer, me.username,
//     "password", OperatingSystemPassword.jsonSerializer, me.password,
//     "is_blocked", JsonSerializer.booleanSerializer, me.isBlocked, 
//     "is_enabled", JsonSerializer.booleanSerializer, me.isEnabled, 
//   )
// )

// const ruleArrayJsonDeserializer = JsonDeserializer.implementForArray(
//   Rule.jsonDeserializer,
// )

// export const jsonDeserializer = JsonDeserializer.implement<RuleEnforcer>(context => 
//   Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map5(
//     JsonDeserializer.propertyAs(context, "rules", ruleArrayJsonDeserializer),
//     JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
//     JsonDeserializer.propertyAs(context, "password", OperatingSystemPassword.jsonDeserializer),
//     JsonDeserializer.propertyAsBoolean(context, "is_blocked"),
//     JsonDeserializer.propertyAsBoolean(context, "is_enabled"),
//     constructor
//   ))
// )
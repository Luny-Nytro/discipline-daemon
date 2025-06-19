import { Unique } from "@Pkg/Unique";
import { Uuid } from "@Pkg/Uuid";
import { CountdownTimer } from "@Pkg/CountdownTimer";
import { Activator } from "./Activator.ts";
import { DateTime } from "@Pkg/DateTime";

export type Rule = Unique<"App.UserAccess.Rule", {
  id: Uuid
  enablerCountdownTimer: CountdownTimer
  activator: Activator
}>

export const Rule = {
  constructor(
    id: Uuid, 
    enablerCountdownTimer: CountdownTimer, 
    activator: Activator,
  ): Rule {
    return Unique({
      id,
      enablerCountdownTimer,
      activator,
    })
  },

  isEnabled(me: Rule, now: DateTime): boolean {
    return CountdownTimer.isRunningUpdated(me.enablerCountdownTimer, now)
  },
  
  isEffective(me: Rule, now: DateTime): boolean {
    return Activator.isRuleActive(me.activator, now) && Rule.isEnabled(me, now)
  },
}

// export const displayer = Displayer.implement<Rule>(me => 
//   Displayer.asNamedObject("Rule",
//     "id", Uuid.displayer, me.id,
//     "enabler", Enabler.displayer, me.enabler,
//     "activator", Activator.displayer, me.activator,
//   )
// )

// export const jsonSerializer = JsonSerializer.implement<Rule>(me => 
//   JsonSerializer.asObject(
//     "id", Uuid.jsonSerializer, me.id,
//     "enabler", Enabler.jsonSerializer, me.enabler,
//     "activator", Activator.jsonSerializer, me.activator,
//   )
// )

// export const jsonDeserializer = JsonDeserializer.implement<Rule>(context => 
//   Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
//     JsonDeserializer.propertyAs(context, "id", Uuid.jsonDeserializer),
//     JsonDeserializer.propertyAs(context, "enabler", Enabler.jsonDeserializer),
//     JsonDeserializer.propertyAs(context, "activator", Activator.jsonDeserializer),
//     constructor
//   ))
// )
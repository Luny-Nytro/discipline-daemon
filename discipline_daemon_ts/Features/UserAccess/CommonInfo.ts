import { Unique } from "@Pkg/Unique";
import { Duration } from "@Pkg/Duration";

export type CommonInfo = Unique<"App.UserAccess.CommonInfo", {
  enforcingInterval: Duration
}>

export const CommonInfo = {
  constructor(
    enforcingInterval: Duration,
  ): CommonInfo {
    return Unique({
      enforcingInterval,
    })
  }
}

// const ruleEnforcerArrayDisplayer = Displayer.implementForArray(
//   RuleEnforcer.displayer,
// )

// export const displayer = Displayer.implement<Feature>(me => 
//   Displayer.asNamedObject("Feature", 
//     "enforcers", ruleEnforcerArrayDisplayer, me.ruleRnforcers, 
//     "enforcingInterval", Duration.displayer, me.enforcingInterval, 
//   )
// )

// const ruleEnforcerArrayJsonSerializer = JsonSerializer.implementForArray(
//   RuleEnforcer.jsonSerializer,
// )

// export const jsonSerializer = JsonSerializer.implement<Feature>(me => 
//   JsonSerializer.asObject(
//     "rule_enforcers", ruleEnforcerArrayJsonSerializer, me.ruleRnforcers,
//     "enforcing_interval", Duration.jsonSerializer, me.enforcingInterval,  
//   )
// )

// const ruleEnforcerArrayJsonDeserializer = JsonDeserializer.implementForArray(
//   RuleEnforcer.jsonDeserializer,
// )

// export const jsonDeserializer = JsonDeserializer.implement<Feature>(context => 
//   Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map2(
//     JsonDeserializer.propertyAs(context, "rule_enforcers", ruleEnforcerArrayJsonDeserializer), 
//     JsonDeserializer.propertyAs(context, "enforcing_interval", Duration.jsonDeserializer), 
//     constructor
//   ))
// )
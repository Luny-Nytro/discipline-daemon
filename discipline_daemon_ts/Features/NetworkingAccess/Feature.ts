import * as Duration from "../../ChronicTypes/Duration.ts";
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as RuleEnforcer from "./RuleEnforcer.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../../ElementaryTypes/Tried.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type Feature = Unique<"App.NetworkingAccess.Feature", {
  ruleRnforcers: RuleEnforcer.RuleEnforcer[]
  enforcingInterval: Duration.Duration
}>

export function constructor(
  ruleRnforcers: RuleEnforcer.RuleEnforcer[],
  enforcingInterval: Duration.Duration,
): Feature {
  return Unique({
    ruleRnforcers,
    enforcingInterval,
  })
}

const ruleEnforcerArrayDisplayer = Displayer.implementForArray(
  RuleEnforcer.displayer,
)

export const displayer = Displayer.implement<Feature>(me => 
  Displayer.asNamedObject("Feature", 
    "enforcers", ruleEnforcerArrayDisplayer, me.ruleRnforcers, 
    "enforcingInterval", Duration.displayer, me.enforcingInterval, 
  )
)

const ruleEnforcerArrayJsonSerializer = JsonSerializer.implementForArray(
  RuleEnforcer.jsonSerializer,
)

export const jsonSerializer = JsonSerializer.implement<Feature>(me => 
  JsonSerializer.asObject(
    "rule_enforcers", ruleEnforcerArrayJsonSerializer, me.ruleRnforcers,
    "enforcing_interval", Duration.jsonSerializer, me.enforcingInterval,  
  )
)

const ruleEnforcerArrayJsonDeserializer = JsonDeserializer.implementForArray(
  RuleEnforcer.jsonDeserializer,
)

export const jsonDeserializer = JsonDeserializer.implement<Feature>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map2(
    JsonDeserializer.propertyAs(context, "rule_enforcers", ruleEnforcerArrayJsonDeserializer), 
    JsonDeserializer.propertyAs(context, "enforcing_interval", Duration.jsonDeserializer), 
    constructor
  ))
)
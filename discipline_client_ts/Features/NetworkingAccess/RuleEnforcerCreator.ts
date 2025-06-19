import * as Displayer from "@Pkg/Display"
import * as OperatingSystemUsername from "@Pkg/OperatingSystemUsername"
import * as JsonSerializer from "@Pkg/JsonSerializer";
import * as JsonDeserializer from "@Pkg/JsonDeserializer";
import { Unique } from "@Pkg/Unique";
import { Tried } from "@Pkg/Tried";

export type RuleEnforcerCreator = Unique<"Discipline.NetworkingAccess.RuleEnforcerCreator", {
  readonly username: OperatingSystemUsername.OperatingSystemUsername
}>

export function create(
  username: OperatingSystemUsername.OperatingSystemUsername, 
): RuleEnforcerCreator {
  return Unique({
    username,
  })
}

export const displayer = Displayer.implement<RuleEnforcerCreator>(me => 
  Displayer.asNamedObject("RuleEnforcerCreator", 
    "username", OperatingSystemUsername.displayer, me.username,
  )
)

export const jsonSerializer = JsonSerializer.implement<RuleEnforcerCreator>(me => 
  JsonSerializer.asObject(
    "username", OperatingSystemUsername.jsonSerializer, me.username,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<RuleEnforcerCreator>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map(
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    create
  ))
)
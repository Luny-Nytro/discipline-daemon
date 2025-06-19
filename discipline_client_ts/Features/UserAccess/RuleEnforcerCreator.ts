import * as Displayer from "@Pkg/Display"
import * as OperatingSystemPassword from "@Pkg/OperatingSystemPassword"
import * as OperatingSystemUsername from "@Pkg/OperatingSystemUsername"
import * as JsonSerializer from "@Pkg/JsonSerializer";
import * as JsonDeserializer from "@Pkg/JsonDeserializer";
import { Unique } from "@Pkg/Unique";
import { Tried } from "@Pkg/Tried";

export type RuleEnforcerCreator = Unique<"Discipline.UserAccess.RuleEnforcerCreator", {
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly password: OperatingSystemPassword.OperatingSystemPassword
}>

export function create(
  username: OperatingSystemUsername.OperatingSystemUsername, 
  password: OperatingSystemPassword.OperatingSystemPassword,
): RuleEnforcerCreator {
  return Unique({
    username,
    password,
  })
}

export const displayer = Displayer.implement<RuleEnforcerCreator>(me => 
  Displayer.asNamedObject("RuleEnforcerCreator", 
    "username", OperatingSystemUsername.displayer, me.username,
    "password", OperatingSystemPassword.displayer, me.password,
  )
)

export const jsonSerializer = JsonSerializer.implement<RuleEnforcerCreator>(me => 
  JsonSerializer.asObject(
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "password", OperatingSystemPassword.jsonSerializer, me.password,  
  )
)

export const jsonDeserializer = JsonDeserializer.implement<RuleEnforcerCreator>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map2(
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "password", OperatingSystemPassword.jsonDeserializer),
    create
  ))
)
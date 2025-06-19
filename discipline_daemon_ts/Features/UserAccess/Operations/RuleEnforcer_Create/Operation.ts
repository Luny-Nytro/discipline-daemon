import * as Client from "../../../../Client.ts";
import * as Outcome from "./Outcome.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as OperatingSystemUsername from "../../../../CommonTypes/OperatingSystemUsername.ts";
import * as OperatingSystemPassword from "../../../../CommonTypes/OperatingSystemPassword.ts";
import { Tried } from "../../../../ElementaryTypes/Tried.ts";
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

export type Operation = Unique<"App.UserAccess.Operations.RuleEnforcer.Create", {
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly password: OperatingSystemPassword.OperatingSystemPassword
}>

export function create(
  username: OperatingSystemUsername.OperatingSystemUsername, 
  password: OperatingSystemPassword.OperatingSystemPassword,
): Operation {
  return Unique({
    username,
    password,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("CreateRuleEnforcer", 
    "username", OperatingSystemUsername.displayer, me.username,
    "password", OperatingSystemPassword.displayer, me.password,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "password", OperatingSystemPassword.jsonSerializer, me.password,  
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map2(
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "password", OperatingSystemPassword.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  [ "UserAccess", "RuleEnforcers", "Create" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
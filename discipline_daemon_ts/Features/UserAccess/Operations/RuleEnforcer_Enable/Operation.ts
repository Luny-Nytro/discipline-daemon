import * as Client from "../../../../Client.ts";
import * as Outcome from "./Outcome.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as OperatingSystemUsername from "../../../../CommonTypes/OperatingSystemUsername.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts";
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

export type Operation = Unique<"App.UserAccess.Operations.RuleEnforcer.Enable", {
  readonly username: OperatingSystemUsername.OperatingSystemUsername
}>

export function create(username: OperatingSystemUsername.OperatingSystemUsername): Operation {
  return Unique({
    username
  })
}

export const displayer = Displayer.implement<Operation>(me =>
  Displayer.asNamedObject("EnableRuleEnforcer", 
    "username", OperatingSystemUsername.displayer, me.username
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "username", OperatingSystemUsername.jsonSerializer, me.username,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map(
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  [ "UserAccess", "RuleEnforcers", "Enable" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
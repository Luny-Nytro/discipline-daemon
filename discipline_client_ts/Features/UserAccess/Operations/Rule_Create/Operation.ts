import * as Client from "../../../../Client.ts";
import * as Outcome from "./Outcome.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts";
import * as RuleCreator from "../../RuleCreator.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as OperatingSystemUsername from "../../../../CommonTypes/OperatingSystemUsername.ts";
import { Tried } from "../../../../ElementaryTypes/Tried.ts";
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

export type Operation = Unique<"Discipline.UserAccess.Operations.Rule.Create", {
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly ruleCreator: RuleCreator.RuleCreator
}>

export function create(
  username: OperatingSystemUsername.OperatingSystemUsername, 
  ruleCreator: RuleCreator.RuleCreator,
): Operation {
  return Unique({
    ruleCreator,
    username,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("CreateRule", 
    "username", OperatingSystemUsername.displayer, me.username,
    "ruleCreator", RuleCreator.displayer, me.ruleCreator,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "rule_creator", RuleCreator.jsonSerializer, me.ruleCreator,  
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map2(
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "rule_creator", RuleCreator.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  [ "UserAccess", "Rules", "Create" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
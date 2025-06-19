import * as Uuid from "../../../../ElementaryTypes/Uuid.ts"
import * as Client from "../../../../Client.ts"
import * as Outcome from "./Outcome.ts"
import * as Duration from "../../../../ChronicTypes/Duration.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as OperatingSystemUsername from "../../../../CommonTypes/OperatingSystemUsername.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

export type Operation = Unique<"Discipline.UserAccess.Operations.Enabler.ForDuration.Increment", {
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly ruleId: Uuid.Uuid
  readonly increment: Duration.Duration
}>

export function create(
  username: OperatingSystemUsername.OperatingSystemUsername,
  ruleId: Uuid.Uuid,
  increment: Duration.Duration,
): Operation {
  return Unique({
    username,
    ruleId,
    increment,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("Enabler.ForDuration.Increment",
    "username", OperatingSystemUsername.displayer, me.username,
    "ruleId", Uuid.displayer, me.ruleId,
    "increment", Duration.displayer, me.increment,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "rule_id", Uuid.jsonSerializer, me.ruleId,
    "increment", Duration.jsonSerializer, me.increment,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "rule_id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "increment", Duration.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  [ "UserAccess", "Rules", "Enabler", "ForDuration", "Increment" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
import * as Uuid from "../../../../ElementaryTypes/Uuid.ts"
import * as Client from "../../../../Client.ts"
import * as Outcome from "./Outcome.ts"
import * as Duration from "../../../../ChronicTypes/Duration.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as OperatingSystemUsername from "../../../../CommonTypes/OperatingSystemUsername.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Tried } from "../../../../ElementaryTypes/Tried.ts";

export type Operation = Unique<"Discipline.NetworkingAccess.Operations.Activator.ForDuration.Increment", {
  readonly ruleId: Uuid.Uuid
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly increment: Duration.Duration
}>

export function create(
  ruleId: Uuid.Uuid,
  username: OperatingSystemUsername.OperatingSystemUsername,
  increment: Duration.Duration,
): Operation {
  return Unique({
    ruleId,
    username,
    increment,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("Activator.ForDuration.Increment",
    "ruleId", Uuid.displayer, me.ruleId,
    "username", OperatingSystemUsername.displayer, me.username,
    "increment", Duration.displayer, me.increment,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "rule_id", Uuid.jsonSerializer, me.ruleId,
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "increment", Duration.jsonSerializer, me.increment,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
    JsonDeserializer.propertyAs(context, "rule_id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "increment", Duration.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  [ "NetworkingAccess", "Rules", "Activator", "ForDuration", "Increment"],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
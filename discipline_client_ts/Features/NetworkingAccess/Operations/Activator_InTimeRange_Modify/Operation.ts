import * as Uuid from "../../../../ElementaryTypes/Uuid.ts"
import * as Client from "../../../../Client.ts"
import * as Outcome from "./Outcome.ts"
import * as TimeRange from "../../../../ChronicTypes/TimeRange.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as OperatingSystemUsername from "../../../../CommonTypes/OperatingSystemUsername.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Tried } from "../../../../ElementaryTypes/Tried.ts";

export type Operation = Unique<"Discipline.NetworkingAccess.Operations.Activator.InTimeRange.Modify", {
  readonly ruleId: Uuid.Uuid
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly newTimeRange: TimeRange.TimeRange
}>

export function create(
  ruleId: Uuid.Uuid,
  username: OperatingSystemUsername.OperatingSystemUsername,
  newTimeRange: TimeRange.TimeRange,
): Operation {
  return Unique({
    ruleId,
    username,
    newTimeRange,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("Activator.InTimeRange.Modify",
    "ruleId", Uuid.displayer, me.ruleId,
    "username", OperatingSystemUsername.displayer, me.username,
    "newTimeRange", TimeRange.displayer, me.newTimeRange,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "rule_id", Uuid.jsonSerializer, me.ruleId,
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "new_time_range", TimeRange.jsonSerializer, me.newTimeRange,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
    JsonDeserializer.propertyAs(context, "rule_id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "new_time_range", TimeRange.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  [ "NetworkingAccess", "Rules", "Activator", "InTimeRange", "Modify" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
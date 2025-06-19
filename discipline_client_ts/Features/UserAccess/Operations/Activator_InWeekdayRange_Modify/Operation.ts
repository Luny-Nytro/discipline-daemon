import * as Uuid from "../../../../ElementaryTypes/Uuid.ts"
import * as Client from "../../../../Client.ts"
import * as Outcome from "./Outcome.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as WeekdayRange from "../../../../ChronicTypes/WeekdayRange.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as OperatingSystemUsername from "../../../../CommonTypes/OperatingSystemUsername.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

export type Operation = Unique<"Discipline.UserAccess.Operations.Activator.InWeekdayRange.Modify", {
  readonly ruleId: Uuid.Uuid
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly newWeekdayRange: WeekdayRange.WeekdayRange
}>

export function create(
  ruleId: Uuid.Uuid,
  username: OperatingSystemUsername.OperatingSystemUsername,
  newWeekdayRange: WeekdayRange.WeekdayRange,
): Operation {
  return Unique({
    ruleId,
    username,
    newWeekdayRange,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("Activator.InWeekdayRange.Modify",
    "ruleId", Uuid.displayer, me.ruleId,
    "username", OperatingSystemUsername.displayer, me.username,
    "newWeekdayRange", WeekdayRange.displayer, me.newWeekdayRange,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "rule_id", Uuid.jsonSerializer, me.ruleId,
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "new_weekday_range", WeekdayRange.jsonSerializer, me.newWeekdayRange,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
    JsonDeserializer.propertyAs(context, "rule_id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "new_weekday_range", WeekdayRange.jsonDeserializer),
    create
  ))
)


export const executer = Client.Executer.implement(
  [ "UserAccess", "Rules", "Activator", "InWeekdayRange", "Modify" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as CountdownTimer from "../../ChronicTypes/CountdownTimer.ts";
import * as Datum from "./Datum.ts";
import * as Name from "./Name.ts";
import * as Uuid from "../../ElementaryTypes/Uuid.ts";
import * as DateTime from "../../ChronicTypes/DateTime.ts";
import * as Duration from "../../ChronicTypes/Duration.ts";
import { Tried } from "../../ElementaryTypes/Tried.ts";
import { Option } from "../../ElementaryTypes/Option.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export type ShadowVault = Unique<"App.ShadowVault", {
  /** @private */
  readonly id: Uuid.Uuid
  /** @private */
  name: Name.Name
  /** @private */
  datum: Option<Datum.Datum>
  /** @private */
  protector: CountdownTimer.CountdownTimer
}>

export function constructor(
  id: Uuid.Uuid, 
  name: Name.Name, 
  datum: Option<Datum.Datum>, 
  protector: CountdownTimer.CountdownTimer,
): ShadowVault {
  return Unique({
    id,
    name,
    datum,
    protector,
  })
}

export function isProtectedUpdated(me: ShadowVault, now: DateTime.DateTime): boolean {
  return CountdownTimer.isRunningUpdated(me.protector, now)
}

export function isProtectedOutdated(me: ShadowVault): boolean {
  return CountdownTimer.isRunningOutdated(me.protector)
}

export function synchronize(me: ShadowVault, now: DateTime.DateTime): void {
  CountdownTimer.synchronize(me.protector, now)
}

export function remainingProtectionDurationUpdated(me: ShadowVault, now: DateTime.DateTime): Duration.Duration {
  return CountdownTimer.remainingDurationUpdated(me.protector, now)
}

export function remainingProtectionDurationOutdated(me: ShadowVault): Duration.Duration {
  return CountdownTimer.remainingDurationOutdated(me.protector)
}

// TODO: Maybe create "*Updated" and "*Outdated" versions of this function. 
export function datum(me: ShadowVault): Option<Datum.Datum> {
  return me.datum
}

export const display = Displayer.implement<ShadowVault>(me => 
  Displayer.asNamedObject("ShadowVault",
    "id", Uuid.displayer, me.id,
    "name", Name.displayer, me.name,
    "datum", Datum.displayerOptional, me.datum,
    "protector", CountdownTimer.displayer, me.protector,
  )
)

export const jsonSerializer = JsonSerializer.implement<ShadowVault>(me =>
  JsonSerializer.asObject(
    "id", Uuid.jsonSerializer, me.id,
    "name", Name.jsonSerializer, me.name,
    "datum", Datum.jsonSerializerOptional, me.datum,
    "protector", CountdownTimer.jsonSerializer, me.protector,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<ShadowVault>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map4(
    JsonDeserializer.propertyAs(context, "id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "name", Name.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "datum", Datum.jsonDeserializerOptional),
    JsonDeserializer.propertyAs(context, "protector", CountdownTimer.jsonDeserializer),
    constructor
  ))
)
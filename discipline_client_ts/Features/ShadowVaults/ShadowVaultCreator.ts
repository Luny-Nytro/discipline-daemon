import * as Uuid from "../../ElementaryTypes/Uuid.ts"
import * as Duration from "../../ChronicTypes/Duration.ts"
import * as Name from "./Name.ts"
import * as Datum from "./Datum.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { Option } from "../../ElementaryTypes/Option.ts";
import { Tried } from "../../ElementaryTypes/Tried.ts";

export type ShadowVaultCreator = Unique<"Discipline.ShadowVault.Creator", {
  readonly id: Option<Uuid.Uuid>
  readonly name: Name.Name
  readonly datum: Datum.Datum
  // TODO: Rename this to "protectionDuration".
  readonly protector: Duration.Duration
}>

export function create(
  id: Option<Uuid.Uuid>,
  name: Name.Name,
  datum: Datum.Datum,
  protector: Duration.Duration,  
): ShadowVaultCreator {
  return Unique({
    id,
    name,
    datum,
    protector,
  })
}

export const displayer = Displayer.implement<ShadowVaultCreator>(me => 
  Displayer.asNamedObject("ShadowVaultCreator",
    "id", Uuid.displayerOptional, me.id,
    "name", Name.displayer, me.name,
    "datum", Datum.displayer, me.datum,
    "protector", Duration.displayer, me.protector,
  )
)

export const jsonSerializer = JsonSerializer.implement<ShadowVaultCreator>(me =>
  JsonSerializer.asObject(
    "id", Uuid.jsonSerializerOptional, me.id,
    "name", Name.jsonSerializer, me.name,
    "datum", Datum.jsonSerializer, me.datum,
    "protector", Duration.jsonSerializer, me.protector,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<ShadowVaultCreator>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map4(
    JsonDeserializer.propertyAs(context, "id", Uuid.jsonDeserializerOptional),
    JsonDeserializer.propertyAs(context, "name", Name.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "datum", Datum.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "protector", Duration.jsonDeserializer),
    create
  ))
)

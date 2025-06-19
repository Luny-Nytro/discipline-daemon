import * as Displayer from "../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../ElementaryTypes/Tried.ts";
import { Unique } from "../ElementaryTypes/Unique.ts";

export type ByPasswordEnabler = Unique<"Discipline.Common.ByPasswordEnabler", {
  isEffective: boolean
}>

export function create(isEffective: boolean): ByPasswordEnabler {
  return Unique({
    isEffective
  })
}

export function isEffective(me: ByPasswordEnabler): boolean {
  return me.isEffective;
}

export function makeEffective(me: ByPasswordEnabler): void {
  me.isEffective = true;
}

export function makeIneffective(me: ByPasswordEnabler): void {
  me.isEffective = false;
}

export const displayer = Displayer.implement<ByPasswordEnabler>(me => 
  Displayer.asNamedObject("ByPasswordEnabler",
    "isEffective", Displayer.booleanDisplayer, me.isEffective
  )
)

export const jsonSerializer = JsonSerializer.implement<ByPasswordEnabler>(me => 
  JsonSerializer.asObject(
    "isEffective", JsonSerializer.booleanSerializer, me.isEffective
  )
)

export const jsonDeserializer = JsonDeserializer.implement<ByPasswordEnabler>(context =>
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map(
    JsonDeserializer.propertyAsBoolean(context, "is_effective"),
    create
  ))
)
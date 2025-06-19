import * as Uuid from "../../ElementaryTypes/Uuid.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as EnablerCreator from "./EnablerCreator.ts"
import * as ActivatorCreator from "./ActivatorCreator.ts"
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import { None, Option, Some } from "../../ElementaryTypes/Option.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { Tried } from "../../ElementaryTypes/Tried.ts";

export interface Initializer {
  readonly id?: Uuid.Uuid
  readonly enabler: EnablerCreator.EnablerCreator
  readonly activator: ActivatorCreator.ActivatorCreator
}

export type RuleCreator = Unique<"Discipline.NetworkingAccess.RuleCreator", {
  readonly id: Option<Uuid.Uuid>
  readonly enabler: EnablerCreator.EnablerCreator
  readonly activator: ActivatorCreator.ActivatorCreator
}>

export function create(
  id: Option<Uuid.Uuid>,
  enabler: EnablerCreator.EnablerCreator,
  activator: ActivatorCreator.ActivatorCreator,
): RuleCreator {
  return Unique({
    id,
    enabler,
    activator,
  })
}

export function fromInitializer(initializer: Initializer): RuleCreator {
  return Unique({
    id: initializer.id ? Some(initializer.id) : None(),
    enabler: initializer.enabler,
    activator: initializer.activator,
  })
}

export const displayer = Displayer.implement<RuleCreator>(me => 
  Displayer.asNamedObject("RuleCreator",
    "id", Uuid.displayerOptional, me.id,
    "enabler", EnablerCreator.displayer, me.enabler,
    "activator", ActivatorCreator.displayer, me.activator,
  )
)

export const jsonSerializer = JsonSerializer.implement<RuleCreator>(me => 
  JsonSerializer.asObject(
    "id", Uuid.jsonSerializerOptional, me.id,
    "enabler", EnablerCreator.jsonSerializer, me.enabler,
    "activator", ActivatorCreator.jsonSerializer, me.activator,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<RuleCreator>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
    JsonDeserializer.propertyAs(context, "id", Uuid.jsonDeserializerOptional),
    JsonDeserializer.propertyAs(context, "enabler", EnablerCreator.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "activator", ActivatorCreator.jsonDeserializer),
    create
  ))
)
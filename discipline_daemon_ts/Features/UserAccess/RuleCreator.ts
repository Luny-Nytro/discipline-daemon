import { Uuid } from "@Pkg/Uuid";
import { Activator } from "./Activator.ts";
import { Unique } from "@Pkg/Unique";
import { None, Option, Some } from "@Pkg/Option";

export interface Initializer {
  readonly id?: Uuid
  readonly activator: Activator
}

export type RuleCreator = Unique<"App.UserAccess.Rule.Creator", {
  readonly id: Option<Uuid>
  readonly activator: Activator
}>

export const RuleCreator = {
  new(
    id: Option<Uuid>,
    activator: Activator,
  ): RuleCreator {
    return Unique({
      id,
      activator,
    })
  },

  fromInitializer(initializer: Initializer): RuleCreator {
    return Unique({
      id: initializer.id ? Some(initializer.id) : None(),
      activator: initializer.activator,
    })
  }
}

// export const displayer = Displayer.implement<RuleCreator>(me => 
//   Displayer.asNamedObject("RuleCreator",
//     "id", Uuid.displayerOptional, me.id,
//     "enabler", EnablerCreator.displayer, me.enabler,
//     "activator", ActivatorCreator.displayer, me.activator,
//   )
// )

// export const jsonSerializer = JsonSerializer.implement<RuleCreator>(me => 
//   JsonSerializer.asObject(
//     "id", Uuid.jsonSerializerOptional, me.id,
//     "enabler", EnablerCreator.jsonSerializer, me.enabler,
//     "activator", ActivatorCreator.jsonSerializer, me.activator,
//   )
// )

// export const jsonDeserializer = JsonDeserializer.implement<RuleCreator>(context => 
//   Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
//     JsonDeserializer.propertyAs(context, "id", Uuid.jsonDeserializerOptional),
//     JsonDeserializer.propertyAs(context, "enabler", EnablerCreator.jsonDeserializer),
//     JsonDeserializer.propertyAs(context, "activator", ActivatorCreator.jsonDeserializer),
//     create
//   ))
// )
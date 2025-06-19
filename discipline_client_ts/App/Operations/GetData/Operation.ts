import * as Client from "../../../Client.ts"
import * as Outcome from "./Outcome.ts"
import * as Displayer from "../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../ElementaryTypes/Unique.ts";
import { Ok } from "../../../ElementaryTypes/Tried.ts";

export type Operation = Unique<"Discipline.App.Operations.GetData", null>

export function create(): Operation {
  return Unique(null)
}

export const displayer = Displayer.implement<Operation>(() => 
  // TODO: Create an "asEmptyStruct" function in "Displayer".
  Displayer.asString("GetAppData")
)

export const jsonSerializer = JsonSerializer.implement<Operation>(() => 
  // TODO: Create an "asEmptyObject" function in "JsonSerializer"
  // and use it here instead of returning an empty object.
  ({})
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(() => 
  // TODO: Create an "asEmptyObject" function in "JsonDeserializer"
  // and use it here.
  Ok(create())
)

export const executer = Client.Executer.implement(
  [ "App", "GetData" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
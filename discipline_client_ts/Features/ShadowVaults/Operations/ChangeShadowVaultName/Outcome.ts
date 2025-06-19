import * as Error from "./Error.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts";

export type Outcome = Tried<null, Error>

export const displayer = Tried.Displayer(
  Displayer.nullDisplayer,
  Error.displayer
)

export const jsonSerializer = Tried.JsonSerializer(
  JsonSerializer.nullSerializer,
  Error.jsonSerializer,
)

export const jsonDeserializer = Tried.JsonDeserializer(
  JsonDeserializer.nullDeserializer,
  Error.jsonDeserializer,
)
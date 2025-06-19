import * as Error from "./Error.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts";
import { nullDisplayer } from "../../../../ElementaryTypes/Display.ts";
import { nullSerializer } from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import { nullDeserializer } from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";

export type Outcome = Tried<null, Error.Error>

export const displayer = Tried.Displayer(
  nullDisplayer,
  Error.displayer
)

export const jsonSerializer = Tried.JsonSerializer<null, Error.Error>(
  nullSerializer,
  Error.jsonSerializer,
)

export const jsonDeserializer = Tried.JsonDeserializer<null, Error.Error>(
  nullDeserializer,
  Error.jsonDeserializer,
)


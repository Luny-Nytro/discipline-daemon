import * as Error from "./Error.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts";
import { nullSerializer } from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import { nullDeserializer } from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { nullDisplayer } from "../../../../ElementaryTypes/Display.ts";

export type Outcome = Tried<null, Error>

export const displayer = Tried.Displayer(
  nullDisplayer,
  Error.displayer
)

export const jsonSerializer = Tried.JsonSerializer(
  nullSerializer,
  Error.jsonSerializer,
)

export const jsonDeserializer = Tried.JsonDeserializer(
  nullDeserializer,
  Error.jsonDeserializer,
)
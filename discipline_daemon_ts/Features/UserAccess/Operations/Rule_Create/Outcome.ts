import * as Rule from "../../Rule.ts"
import * as Error from "./Error.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts";

export type Outcome = Tried<Rule.Rule, Error.Error>

export const displayer = Tried.Displayer(
  Rule.displayer,
  Error.displayer,
)

export const jsonSerializer = Tried.JsonSerializer<Rule.Rule, Error.Error>(
  Rule.jsonSerializer,
  Error.jsonSerializer,
)

export const jsonDeserializer = Tried.JsonDeserializer<Rule.Rule, Error.Error>(
  Rule.jsonDeserializer,
  Error.jsonDeserializer,
)

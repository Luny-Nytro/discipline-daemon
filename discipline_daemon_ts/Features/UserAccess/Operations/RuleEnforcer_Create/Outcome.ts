import * as RuleEnforcer from "../../Blocker.ts"
import * as Error from "./Error.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts";

export type Outcome = Tried<RuleEnforcer.RuleEnforcer, Error.Error>

export const displayer = Tried.Displayer(
  RuleEnforcer.displayer,
  Error.displayer,
)

export const jsonSerializer = Tried.JsonSerializer<RuleEnforcer.RuleEnforcer, Error.Error>(
  RuleEnforcer.jsonSerializer,
  Error.jsonSerializer,
)

export const jsonDeserializer = Tried.JsonDeserializer<RuleEnforcer.RuleEnforcer, Error.Error>(
  RuleEnforcer.jsonDeserializer,
  Error.jsonDeserializer,
)
import { Tried } from "../../../../ElementaryTypes/Tried.ts";
import * as Error from "./Error.ts"
import * as ShadowVault from "../../ShadowVault.ts"

export type Outcome = Tried<ShadowVault.ShadowVault, Error>

export const displayer = Tried.Displayer(
  ShadowVault.display,
  Error.displayer,
)

export const jsonSerializer = Tried.JsonSerializer(
  ShadowVault.jsonSerializer,
  Error.jsonSerializer,
)

export const jsonDeserializer = Tried.JsonDeserializer(
  ShadowVault.jsonDeserializer,
  Error.jsonDeserializer,
)
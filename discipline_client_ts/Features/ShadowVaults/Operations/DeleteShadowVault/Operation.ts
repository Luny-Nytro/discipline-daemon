import * as Uuid from "../../../../ElementaryTypes/Uuid.ts";
import * as Client from "../../../../Client.ts";
import * as Outcome from "./Outcome.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../../../../ElementaryTypes/Tried.ts";
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

export type Operation = Unique<"Discipline.ShadowVaults.Operations.DeleteShadowVault", {
  readonly shadowVaultId: Uuid.Uuid
}>

export function create(shadowVaultId: Uuid.Uuid): Operation {
  return Unique({
    shadowVaultId,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("DeleteShadowVault", 
    "shadowVaultId", Uuid.displayer, me.shadowVaultId
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "shadow_vault_id", Uuid.jsonSerializer, me.shadowVaultId
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map(
    JsonDeserializer.propertyAs(context, "shadow_vault_id", Uuid.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  ["ShadowVaults", "Delete"],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
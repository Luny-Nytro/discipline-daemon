import * as Uuid from "../../../../ElementaryTypes/Uuid.ts"
import * as Name from "../../Name.ts"
import * as Client from "../../../../Client.ts"
import * as Outcome from "./Outcome.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Tried } from "../../../../ElementaryTypes/Tried.ts";

export type Operation = Unique<"App.ShadowVaults.Operations.ChangeShadowVaultName", {
  readonly shadowVaultId: Uuid.Uuid
  readonly newName: Name.Name
}>

export function create(shadowVaultId: Uuid.Uuid, newName: Name.Name): Operation {
  return Unique({
    shadowVaultId,
    newName
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("ChangeShadowVaultName",
    "shadowVaultId", Uuid.displayer, me.shadowVaultId,
    "newName", Name.displayer, me.newName,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "shadow_vault_id", Uuid.jsonSerializer, me.shadowVaultId,
    "new_name", Name.jsonSerializer, me.newName,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map2(
    JsonDeserializer.propertyAs(context, "shadow_vault_id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "new_name", Name.jsonDeserializer),
    create
  ))
)


export const executer = Client.Executer.implement(
  [ "ShadowVaults", "ChangeName" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
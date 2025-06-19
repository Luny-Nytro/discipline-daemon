import * as Client from "../../../../Client.ts"
import * as Outcome from "./Outcome.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as ShadowVaultCreator from "../../ShadowVaultCreator.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Tried } from "../../../../ElementaryTypes/Tried.ts";

export type Operation = Unique<"Discipline.ShadowVaults.Operations.CreateShadowVault", {
  readonly shadowVaultCreator: ShadowVaultCreator.ShadowVaultCreator
}>

export function create(shadowVaultCreator: ShadowVaultCreator.ShadowVaultCreator): Operation {
  return Unique({
    shadowVaultCreator
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("CreateShadowVault", 
    "shadowVaultCreator", ShadowVaultCreator.displayer, me.shadowVaultCreator
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "shadow_vault_creator", ShadowVaultCreator.jsonSerializer, me.shadowVaultCreator
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map(
    JsonDeserializer.propertyAs(context, "shadow_vault_creator", ShadowVaultCreator.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  ["ShadowVaults", "Create"],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
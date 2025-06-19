import * as Uuid from "../../../../ElementaryTypes/Uuid.ts";
import * as Client from "../../../../Client.ts";
import * as Outcome from "./Outcome.ts"
import * as Duration from "../../../../ChronicTypes/Duration.ts";
import * as Displayer from "../../../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../../../../ElementaryTypes/Tried.ts";
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

export type Operation = Unique<"App.ShadowVaults.Operations.IncrementShadowVaultProtectionDuration", {
  readonly shadowVaultId: Uuid.Uuid
  readonly increment: Duration.Duration
}>

export function create(shadowVaultId: Uuid.Uuid, increment: Duration.Duration): Operation {
  return Unique({
    increment,
    shadowVaultId,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("ShadowVaultProtectorForDurationIncrement", 
    "shadowVaultId", Uuid.displayer, me.shadowVaultId,
    "increment", Duration.displayer, me.increment,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "shadow_vault_id", Uuid.jsonSerializer, me.shadowVaultId,
    "increment", Duration.jsonSerializer, me.increment,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map2(
    JsonDeserializer.propertyAs(context, "shadow_vault_id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "increment", Duration.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  // TODO: Rename this to "ShadowVaults/IncrementProtectionDuration"
  //       in both client and service.
  ["ShadowVaults", "Protector", "Increment"],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)
import * as DateTime from "../../../ChronicTypes/DateTime.ts"
import * as Displayer from "../../../ElementaryTypes/Display.ts"
import * as UserAccess from "../../../Features/UserAccess/mod.ts"
import * as ShadowVaults from "../../../Features/ShadowVaults/mod.ts"
import * as NetworkingAccess from "../../../Features/NetworkingAccess/mod.ts"
import * as JsonSerializer from "../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../ElementaryTypes/Unique.ts";
import { Tried } from "../../../ElementaryTypes/Tried.ts";

export type Outcome = Unique<"App.App.Operations.GetData.Outcome", {
  readonly now: DateTime.DateTime
  readonly userAccess: UserAccess.Feature.Feature
  readonly shadowVaults: ShadowVaults.Feature.Feature
  readonly networkingAccess: NetworkingAccess.Feature.Feature
}>

function constructor(
  now: DateTime.DateTime,
  userAccess: UserAccess.Feature.Feature,
  shadowVaults: ShadowVaults.Feature.Feature,
  networkingAccess: NetworkingAccess.Feature.Feature,
): Outcome {
  return Unique({
    now,
    userAccess,
    shadowVaults,
    networkingAccess,
  })
}

export const displayer = Displayer.implement<Outcome>(me => 
  Displayer.asNamedObject("AppData",
    "now", DateTime.displayer, me.now,
    "userAccess", UserAccess.Feature.displayer, me.userAccess,
    "shadowVaults", ShadowVaults.Feature.displayer, me.shadowVaults,
    "networkingAccess", NetworkingAccess.Feature.displayer, me.networkingAccess,
  )
)

export const jsonSerializer = JsonSerializer.implement<Outcome>(me=> 
  JsonSerializer.asObject(
    "now", DateTime.jsonSerializer, me.now,
    "user_access", UserAccess.Feature.jsonSerializer, me.userAccess,
    "shadow_vaults", ShadowVaults.Feature.jsonSerializer, me.shadowVaults,
    "networking_access", NetworkingAccess.Feature.jsonSerializer, me.networkingAccess,  
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Outcome>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map4(
    JsonDeserializer.propertyAs(context, "now", DateTime.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "user_access", UserAccess.Feature.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "shadow_vaults", ShadowVaults.Feature.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "networking_access", NetworkingAccess.Feature.jsonDeserializer),
    constructor
  ))
)
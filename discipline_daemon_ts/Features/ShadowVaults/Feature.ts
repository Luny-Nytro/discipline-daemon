import * as Displayer from "../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import * as ShadowVault from "./ShadowVault.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";
import { Tried } from "../../ElementaryTypes/Tried.ts";

export type Feature = Unique<"App.ShadowVaults.Feature", {
  shadowVaults: ShadowVault.ShadowVault[]
}>

export function constructor(shadowVaults: ShadowVault.ShadowVault[]): Feature {
  return Unique({
    shadowVaults,
  })
}

const shadowVaultArrayDisplayer = Displayer.implementForArray(
  ShadowVault.display
)

export const displayer = Displayer.implement<Feature>(me => 
  Displayer.asNamedObject("ShadowVaultsFeature",
    "shadowVaults", shadowVaultArrayDisplayer, me.shadowVaults,
  )
)

const shadowVaultArrayJsonSerializer = JsonSerializer.implementForArray(
  ShadowVault.jsonSerializer
)

export const jsonSerializer = JsonSerializer.implement<Feature>(me => 
  JsonSerializer.asObject(
    "shadow_vaults", shadowVaultArrayJsonSerializer, me.shadowVaults,
  )
)

const shadowVaultArrayJsonDeserializer = JsonDeserializer.implementForArray(
  ShadowVault.jsonDeserializer
)

export const jsonDeserializer = JsonDeserializer.implement<Feature>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map(
    JsonDeserializer.propertyAs(context, "shadow_vaults", shadowVaultArrayJsonDeserializer),
    constructor
  ))
)
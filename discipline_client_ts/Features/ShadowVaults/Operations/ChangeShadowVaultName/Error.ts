import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";
import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts";
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

const enum Variant {
  NoSuchShadowVault,
  InternalError,
}

const NO_SUCH_SHADOW_VAULT_AS_STRING = "NoSuchShadowVault"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export type Error = Unique<"Discipline.ShadowVaults.Operations.ChangeShadowVaultName.Error", Variant>

export function NoSuchShadowVault(): Error {
  return Unique(Variant.NoSuchShadowVault)
}

export function InternalError(): Error {
  return Unique(Variant.InternalError)
}

export interface Cases<A, B> {
  readonly NoSuchShadowVault: () => A
  readonly InternalError: () => B
}

export function  match<A, B>(
  me: Error,
  cases: Cases<A, B>,
): 
  A | B
{
  switch (me as Variant) {
    case Variant.NoSuchShadowVault: {
      return cases.NoSuchShadowVault()
    }
    case Variant.InternalError: {
      return cases.InternalError()
    }
  }
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `ChangeShadowVaultNameError.fromString: Expected ${
      Displayer.stringDisplayer.display(NO_SUCH_SHADOW_VAULT_AS_STRING)
    } or ${
      Displayer.stringDisplayer.display(INTERNAL_ERROR_AS_STRING)
    }, but found ${
      Displayer.stringDisplayer.display(variant)
    }.`
  )
}

export function fromString(variant: string): Tried<Error, GenericError.GenericError> {
  switch (variant) {
    case NO_SUCH_SHADOW_VAULT_AS_STRING: {
      return Ok(NoSuchShadowVault())
    }
    case INTERNAL_ERROR_AS_STRING: {
      return Ok(InternalError())
    }
    default: {
      return Err(UnknownVariantError(variant))
    }
  }
}

export function asString(me: Error): string {
  return match(me, {
    InternalError: () => INTERNAL_ERROR_AS_STRING,
    NoSuchShadowVault: () => NO_SUCH_SHADOW_VAULT_AS_STRING,
  })  
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("ChangeShadowVaultNameError", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Error>(me => 
  JsonSerializer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Error>(context => 
  Tried.andThen(JsonDeserializer.asString(context), string => Tried.mapErr(
    fromString(string),
    error => JsonDeserializer.err(GenericError.displayer.display(error)),
  ))
)

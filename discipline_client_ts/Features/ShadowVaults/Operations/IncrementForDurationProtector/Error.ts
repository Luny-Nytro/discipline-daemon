import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";

const enum Variant {
  NoSuchShadowVault,
  WouldBeEffectiveForTooLong,
  InternalError,
}

export interface Cases<A, B, C> {
  readonly NoSuchShadowVault: () => A
  readonly WouldBeEffectiveForTooLong: () => B
  readonly InternalError: () => C  
}

const NO_SUCH_SHADOW_VAULT_AS_STRING = "NoSuchShadowVault"
const WOULD_BLOCK_FOR_TOO_LONG_AS_STRING = "WouldBeEffectiveForTooLong"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export type Error = Unique<"Discipline.ShadowVaults.Operations.IncrementShadowVaultProtectionDuration.Error", Variant>

export function NoSuchShadowVault(): Error {
  return Unique(Variant.NoSuchShadowVault)
}
export function WouldBeEffectiveForTooLong(): Error {
  return Unique(Variant.WouldBeEffectiveForTooLong)
}
export function InternalError(): Error {
  return Unique(Variant.InternalError)
}
export function match<A, B, C>(
  me: Error,
  cases: Cases<A, B, C>
): 
  A | B | C
{
  switch (me as Variant) {
    case Variant.NoSuchShadowVault: {
      return cases.NoSuchShadowVault()
    }
    case Variant.WouldBeEffectiveForTooLong: {
      return cases.WouldBeEffectiveForTooLong()
    }
    case Variant.InternalError: {
      return cases.InternalError()
    }
  }
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `IncrementShadowVaultProtectionDurationError.fromString: Expected ${
      Displayer.stringDisplayer.display(NO_SUCH_SHADOW_VAULT_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(WOULD_BLOCK_FOR_TOO_LONG_AS_STRING)
    } or ${
      Displayer.stringDisplayer.display(INTERNAL_ERROR_AS_STRING)
    } but found ${
      Displayer.stringDisplayer.display(variant)
    }.`
  )
}

export function fromString(variant: string): Tried<Error, GenericError.GenericError> {
  switch (variant) {
    case NO_SUCH_SHADOW_VAULT_AS_STRING: {
      return Ok(NoSuchShadowVault())
    }
    case WOULD_BLOCK_FOR_TOO_LONG_AS_STRING: {
      return Ok(WouldBeEffectiveForTooLong())
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
    NoSuchShadowVault: () => NO_SUCH_SHADOW_VAULT_AS_STRING,
    WouldBeEffectiveForTooLong: () => WOULD_BLOCK_FOR_TOO_LONG_AS_STRING,
    InternalError: () => INTERNAL_ERROR_AS_STRING,
  })
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("IncrementShadowVaultProtectionDurationError", asString(me))
)

export const jsonSerializer = JsonSerializer.implement<Error>(me => 
  JsonSerializer.asString(asString(me))
)

export const jsonDeserializer = JsonDeserializer.implement<Error>(context => 
  Tried.andThen(JsonDeserializer.asString(context), string => Tried.mapErr(
    fromString(string),
    error => JsonDeserializer.err(GenericError.displayer.display(error))
  ))
)
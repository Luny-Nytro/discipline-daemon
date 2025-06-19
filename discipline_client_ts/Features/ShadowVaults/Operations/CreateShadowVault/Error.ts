import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";

const enum Variant {
  ShadowVaultCreationLimitReached,
  IdUnavailable,
  InternalError,
}

export interface Cases<A, B, C> {
  readonly ShadowVaultCreationLimitReached: () => A
  readonly IdUnavailable: () => B
  readonly InternalError: () => C 
}

export type Error = Unique<"Discipline.ShadowVaults.Operations.CreateShadowVault.Error", Variant>

const SHADOW_VAULT_CREATION_LIMIT_REACHED = "ShadowVaultCreationLimitReached"
const ID_UNAVAILABLE_AS_STRING = "ProvidedIdUsedByAnotherRule"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export function ShadowVaultCreationLimitReached(): Error {
  return Unique(Variant.ShadowVaultCreationLimitReached)
}
export function IdUnavailable(): Error {
  return Unique(Variant.IdUnavailable)
}
export function InternalError(): Error {
  return Unique(Variant.InternalError)
}
export function match<A, B, C>(
  me: Error,
  cases: Cases<A, B, C>,
): 
  A | B | C 
{
  switch (me as Variant) {
    case Variant.ShadowVaultCreationLimitReached: {
      return cases.ShadowVaultCreationLimitReached()
    }
    case Variant.IdUnavailable: {
      return cases.IdUnavailable()
    }
    case Variant.InternalError: {
      return cases.InternalError()
    }
  }
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `CreateShadowVaultError.fromString: Expected ${
      Displayer.stringDisplayer.display(SHADOW_VAULT_CREATION_LIMIT_REACHED)
    }, ${
      Displayer.stringDisplayer.display(ID_UNAVAILABLE_AS_STRING)
    }, or ${
      Displayer.stringDisplayer.display(INTERNAL_ERROR_AS_STRING)
    }, but found ${
      Displayer.stringDisplayer.display(variant)
    }.`
  )
}

export function fromString(variant: string): Tried<Error, GenericError.GenericError> {
  switch (variant) {
    case SHADOW_VAULT_CREATION_LIMIT_REACHED: {
      return Ok(ShadowVaultCreationLimitReached())
    }
    case ID_UNAVAILABLE_AS_STRING: {
      return Ok(IdUnavailable())
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
    IdUnavailable: () => ID_UNAVAILABLE_AS_STRING,
    ShadowVaultCreationLimitReached: () => SHADOW_VAULT_CREATION_LIMIT_REACHED,
  })  
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("CreateShadowVaultError", asString(me))
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
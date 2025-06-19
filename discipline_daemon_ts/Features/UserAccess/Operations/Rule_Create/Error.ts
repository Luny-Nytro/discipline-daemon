import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";

const enum Variant {
  NoSuchRuleEnforcer,
  RuleCreationLimitReached,
  IdUnavailable,
  InternalError,
}

export interface Cases<A, B, C, D> {
  readonly NoSuchRuleEnforcer: () => A
  readonly RuleCreationLimitReached: () => B
  readonly IdUnavailable: () => C
  readonly InternalError: () => D   
}

export type Error = Unique<"App.UserAccess.Operations.Rule.Create.Error", Variant>

const NO_SUCH_RULE_ENFORCER_AS_STRING = "NoSuchRuleEnforcer"
const RULE_CREATION_LIMIT_REACHED = "RuleCreationLimitReached"
const ID_UNAVAILABLE_AS_STRING = "IdUnavailable"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export function NoSuchRuleEnforcer(): Error {
  return Unique(Variant.NoSuchRuleEnforcer)
}
export function RuleCreationLimitReached(): Error {
  return Unique(Variant.RuleCreationLimitReached)
}
export function IdUnavailable(): Error {
  return Unique(Variant.IdUnavailable)
}
export function InternalError(): Error {
  return Unique(Variant.InternalError)
}

export function match<A, B, C, D>(
  me: Error,
  cases: Cases<A, B, C, D>
): 
  A | B | C | D
{
  switch (me as Variant) {
    case Variant.NoSuchRuleEnforcer: {
      return cases.NoSuchRuleEnforcer()
    }
    case Variant.RuleCreationLimitReached: {
      return cases.RuleCreationLimitReached()
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
    `UserAccess.Operations.Rule.Create.Error.fromString: Unknown variant. Expected ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_ENFORCER_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(RULE_CREATION_LIMIT_REACHED)
    }, ${
      Displayer.stringDisplayer.display(ID_UNAVAILABLE_AS_STRING)
    } or ${
      Displayer.stringDisplayer.display(INTERNAL_ERROR_AS_STRING)
    } but found ${
      Displayer.stringDisplayer.display(variant)
    }.`
  )
}

export function fromString(variant: string): Tried<Error, GenericError.GenericError> {
  switch (variant) {
    case NO_SUCH_RULE_ENFORCER_AS_STRING: {
      return Ok(NoSuchRuleEnforcer())
    }
    case RULE_CREATION_LIMIT_REACHED: {
      return Ok(RuleCreationLimitReached())
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
    NoSuchRuleEnforcer: () => NO_SUCH_RULE_ENFORCER_AS_STRING,
    RuleCreationLimitReached: () => RULE_CREATION_LIMIT_REACHED,
    IdUnavailable: () => ID_UNAVAILABLE_AS_STRING,
    InternalError: () => INTERNAL_ERROR_AS_STRING,
  })
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("CreateRuleError", asString(me))
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
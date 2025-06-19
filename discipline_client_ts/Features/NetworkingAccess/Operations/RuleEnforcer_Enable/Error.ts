import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";

const enum Variant {
  NoSuchRuleEnforcer,
  RuleEnforcerAlreadyEnabled,
  InternalError,
}

export interface Cases<A, B, C> {
  readonly NoSuchRuleEnforcer: () => A
  readonly RuleEnforcerAlreadyEnabled: () => B
  readonly InternalError: () => C
}

const NO_SUCH_RULE_ENFORCER_AS_STRING = "NoSuchRuleEnforcer"
const RULE_ENFORCER_ALREADY_ENABLED_AS_STRING = "RuleEnforcerAlreadyEnabled"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export type Error = Unique<"Discipline.NetworkingAccess.Operations.RuleEnforcer.Enable.Error", Variant>

export function NoSuchRuleEnforcer(): Error {
  return Unique(Variant.NoSuchRuleEnforcer)
}
export function RuleEnforcerAlreadyEnabled(): Error {
  return Unique(Variant.RuleEnforcerAlreadyEnabled)
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
    case Variant.NoSuchRuleEnforcer: {
      return cases.NoSuchRuleEnforcer()
    }
    case Variant.RuleEnforcerAlreadyEnabled: {
      return cases.RuleEnforcerAlreadyEnabled()
    }
    case Variant.InternalError: {
      return cases.InternalError()
    }
  }
}

export function asString(me: Error): string {
  return match(me, {
    InternalError: () => INTERNAL_ERROR_AS_STRING,
    NoSuchRuleEnforcer: () => NO_SUCH_RULE_ENFORCER_AS_STRING,
    RuleEnforcerAlreadyEnabled: () => RULE_ENFORCER_ALREADY_ENABLED_AS_STRING,
  })
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `NetworkingAccess.Operations.RuleEnforcer.Enable.Error.fromString: Unknown variant. Expected ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_ENFORCER_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(RULE_ENFORCER_ALREADY_ENABLED_AS_STRING)
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
    case RULE_ENFORCER_ALREADY_ENABLED_AS_STRING: {
      return Ok(RuleEnforcerAlreadyEnabled())
    }
    case INTERNAL_ERROR_AS_STRING: {
      return Ok(InternalError())
    }
    default: {
      return Err(UnknownVariantError(variant))
    }
  }
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("EnableRuleEnforcerError", asString(me))
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

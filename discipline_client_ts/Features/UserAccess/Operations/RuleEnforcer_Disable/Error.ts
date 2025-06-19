import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";

const enum Variant {
  NoSuchRuleEnforcer,
  RuleEnforcerAlreadyDisabled,
  SomeRulesAreEnabled,
  InternalError,
}

export interface Cases<A, B, C, D> {
  readonly NoSuchRuleEnforcer: () => A
  readonly RuleEnforcerAlreadyDisabled: () => B
  readonly SomeRulesAreEnabled: () => C
  readonly InternalError: () => D
}

const NO_SUCH_RULE_ENFORCER_AS_STRING = "NoSuchRuleEnforcer"
const RULE_ENFORCER_ALREADY_DISABLED_AS_STRING = "RuleEnforcerAlreadyDisabled"
const SOME_RULES_ARE_ENABLED_AS_STRING = "SomeRulesAreEnabled"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export type Error = Unique<"Discipline.UserAccess.Opreations.RuleEnforcer.Disable.Error", Variant>

export function NoSuchRuleEnforcer(): Error {
  return Unique(Variant.NoSuchRuleEnforcer)
}
export function RuleEnforcerAlreadyDisabled(): Error {
  return Unique(Variant.RuleEnforcerAlreadyDisabled)
}
export function SomeRulesAreEnabled(): Error {
  return Unique(Variant.SomeRulesAreEnabled)
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
    case Variant.RuleEnforcerAlreadyDisabled: {
      return cases.RuleEnforcerAlreadyDisabled()
    }
    case Variant.SomeRulesAreEnabled: {
      return cases.SomeRulesAreEnabled()
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
    SomeRulesAreEnabled: () => SOME_RULES_ARE_ENABLED_AS_STRING,
    RuleEnforcerAlreadyDisabled: () => RULE_ENFORCER_ALREADY_DISABLED_AS_STRING,
  })
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `UserAccess.Operations.RuleEnforcer.Disable.Error.fromString: Unknown variant. Expected ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_ENFORCER_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(RULE_ENFORCER_ALREADY_DISABLED_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(SOME_RULES_ARE_ENABLED_AS_STRING)
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
    case RULE_ENFORCER_ALREADY_DISABLED_AS_STRING: {
      return Ok(RuleEnforcerAlreadyDisabled())
    }
    case SOME_RULES_ARE_ENABLED_AS_STRING: {
      return Ok(SomeRulesAreEnabled())
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
  Displayer.asWrappedString("DisableRuleEnforcerError", asString(me))
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
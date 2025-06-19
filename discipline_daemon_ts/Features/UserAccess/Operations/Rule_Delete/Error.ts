import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";

const enum Variant {
  NoSuchRuleEnforcer,
  NoSuchRule,
  WouldMakeRuleLessRestrictive,
  InternalError,
}

export interface Cases<A, B, C, D> {
  NoSuchRuleEnforcer: () => A
  NoSuchRule: () => B
  WouldMakeRuleLessRestrictive: () => C
  InternalError: () => D
}

export type Error = Unique<"App.UserAccess.Operations.Rule.Delete.Error", Variant>

const NO_SUCH_RULE_ENFORCER_AS_STRING = "NoSuchRuleEnforcer"
const NO_SUCH_RULE_AS_STRING = "NoSuchRule"
const WOULD_MAKE_RULE_LESS_RESTRICTIVE_AS_STRING = "WouldMakeRuleLessRestrictive"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export function NoSuchRuleEnforcer(): Error {
  return Unique(Variant.NoSuchRuleEnforcer)
}
export function NoSuchRule(): Error {
  return Unique(Variant.NoSuchRule)
}
export function WouldMakeRuleLessRestrictive(): Error {
  return Unique(Variant.WouldMakeRuleLessRestrictive)
}
export function InternalError(): Error {
  return Unique(Variant.InternalError)
}

export function match<A, B, C, D>(
  me: Error,
  cases: Cases<A, B, C, D>,
): 
  A | B | C | D
{
  switch (me as Variant) {
    case Variant.NoSuchRuleEnforcer: {
      return cases.NoSuchRuleEnforcer()
    }
    case Variant.NoSuchRule: {
      return cases.NoSuchRule()
    }
    case Variant.WouldMakeRuleLessRestrictive: {
      return cases.WouldMakeRuleLessRestrictive()
    }
    case Variant.InternalError: {
      return cases.InternalError()
    }
  }
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `UserAccess.Operations.Rule.Delete.Error.fromString: Unknown variant. Expected ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_ENFORCER_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(WOULD_MAKE_RULE_LESS_RESTRICTIVE_AS_STRING)
    } or ${
      Displayer.stringDisplayer.display(INTERNAL_ERROR_AS_STRING)
    } but found ${
      Displayer.stringDisplayer.display(variant)
    }`
  )
}

export function fromString(variant: string): Tried<Error, GenericError.GenericError> {
  switch (variant) {
    case NO_SUCH_RULE_ENFORCER_AS_STRING: {
      return Ok(NoSuchRuleEnforcer())
    }
    case NO_SUCH_RULE_AS_STRING: {
      return Ok(NoSuchRule())
    }
    case WOULD_MAKE_RULE_LESS_RESTRICTIVE_AS_STRING: {
      return Ok(WouldMakeRuleLessRestrictive())
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
    WouldMakeRuleLessRestrictive: () => WOULD_MAKE_RULE_LESS_RESTRICTIVE_AS_STRING,
    NoSuchRule: () => NO_SUCH_RULE_AS_STRING,
    InternalError: () => INTERNAL_ERROR_AS_STRING,
  })
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("DeleteRuleError", asString(me))
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
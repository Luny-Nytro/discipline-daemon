import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";

const enum Variant {
  NoSuchRuleEnforcer,
  NoSuchRule,
  WrongEnablerType,
  WouldMakeRuleLessRestrictive,
  WouldBeEffectiveForTooLong,
  InternalError,
}

export interface Cases<A, B, C, D, E> {
  readonly NoSuchRuleEnforcer: () => B
  readonly NoSuchRule: () => A
  readonly WrongEnablerType: () => D
  readonly WouldMakeRuleLessRestrictive: () => C
  readonly WouldBeEffectiveForTooLong: () => E
  readonly InternalError: () => E
}

export type Error = Unique<"Discipline.UserAccess.Operations.Enabler.Forduration.Increment.Error", Variant>

const NO_SUCH_RULE_ENFORCER_AS_STRING = "NoSuchRuleEnforcer"
const NO_SUCH_RULE_AS_STRING = "NoSuchRule"
const WRONG_PROTECTOR_TYPE_AS_STRING = "WrongEnablerType"
const WOULD_MAKE_RULE_LESS_RESTRICTIVE_AS_STRING = "WouldMakeRuleLessRestrictive"
const WOULD_BE_EFFECTIVE_FOR_TOO_LONG_AS_STRING = "WouldBeEffectiveForTooLong"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export function NoSuchRuleEnforcer(): Error {
  return Unique(Variant.NoSuchRuleEnforcer)
}
export function NoSuchRule(): Error {
  return Unique(Variant.NoSuchRule)
}
export function WrongEnablerType(): Error {
  return Unique(Variant.WrongEnablerType)
}
export function WouldMakeRuleLessRestrictive(): Error {
  return Unique(Variant.WouldMakeRuleLessRestrictive)
}
export function WouldBeEffectiveForTooLong(): Error {
  return Unique(Variant.WouldBeEffectiveForTooLong)
}
export function InternalError(): Error {
  return Unique(Variant.InternalError)
}

export function match<A, B, C, D, E>(
  me: Error,
  cases: Cases<A, B, C, D, E>
): 
  A | B | C | D | E
{
  switch (me as Variant) {
    case Variant.NoSuchRule: {
      return cases.NoSuchRule()
    }
    case Variant.NoSuchRuleEnforcer: {
      return cases.NoSuchRuleEnforcer()
    }
    case Variant.WouldMakeRuleLessRestrictive: {
      return cases.WouldMakeRuleLessRestrictive()
    }
    case Variant.WrongEnablerType: {
      return cases.WrongEnablerType()
    }
    case Variant.InternalError: {
      return cases.InternalError()
    }
    case Variant.WouldBeEffectiveForTooLong: {
      return cases.WouldBeEffectiveForTooLong()
    }
  }
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `UserAccess.Operations.Enabler.ForDuration.Increment.Error.fromString: Unknown variant. Expected ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_ENFORCER_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(WRONG_PROTECTOR_TYPE_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(WOULD_MAKE_RULE_LESS_RESTRICTIVE_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(WOULD_BE_EFFECTIVE_FOR_TOO_LONG_AS_STRING)
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
    case NO_SUCH_RULE_AS_STRING: {
      return Ok(NoSuchRule())
    }
    case WRONG_PROTECTOR_TYPE_AS_STRING: {
      return Ok(WrongEnablerType())
    }
    case WOULD_MAKE_RULE_LESS_RESTRICTIVE_AS_STRING: {
      return Ok(WouldMakeRuleLessRestrictive())
    }
    case WOULD_BE_EFFECTIVE_FOR_TOO_LONG_AS_STRING: {
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
    NoSuchRule: () => NO_SUCH_RULE_AS_STRING,
    NoSuchRuleEnforcer: () => NO_SUCH_RULE_ENFORCER_AS_STRING,
    WrongEnablerType: () => WRONG_PROTECTOR_TYPE_AS_STRING,
    WouldMakeRuleLessRestrictive: () => WOULD_MAKE_RULE_LESS_RESTRICTIVE_AS_STRING,
    WouldBeEffectiveForTooLong: () => WOULD_BE_EFFECTIVE_FOR_TOO_LONG_AS_STRING,
    InternalError: () => INTERNAL_ERROR_AS_STRING,
  })
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("Enabler.ForDuration.Increment.Error", asString(me))
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

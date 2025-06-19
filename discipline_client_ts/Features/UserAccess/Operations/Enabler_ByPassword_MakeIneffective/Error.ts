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
  AlreadyIneffective,
  WrongPassword,
  InternalError,
}

export interface Cases<A, B, C, D, E, F> {
  readonly NoSuchRuleEnforcer: () => B
  readonly NoSuchRule: () => A
  readonly WrongEnablerType: () => D
  readonly AlreadyIneffective: () => C
  readonly WrongPassword: () => F
  readonly InternalError: () => E
}

export type Error = Unique<"Discipline.UserAccess.Operations.Enabler.ByPassword.MakeIneffective.Error", Variant>

const NO_SUCH_RULE_ENFORCER_AS_STRING = "NoSuchRuleEnforcer"
const NO_SUCH_RULE_AS_STRING = "NoSuchRule"
const WRONG_PROTECTOR_TYPE_AS_STRING = "WrongEnablerType"
const ALREADY_INEFFECTIVE_AS_STRING = "AlreadyIneffective"
const WRONG_PASSWORD_AS_STRING = "WrongPassword"
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
export function AlreadyIneffective(): Error {
  return Unique(Variant.AlreadyIneffective)
}
export function WrongPassword(): Error {
  return Unique(Variant.WrongPassword)
}
export function InternalError(): Error {
  return Unique(Variant.InternalError)
}

export function match<A, B, C, D, E, F>(
  me: Error,
  cases: Cases<A, B, C, D, E, F>
): 
  A | B | C | D | E | F
{
  switch (me as Variant) {
    case Variant.NoSuchRuleEnforcer: {
      return cases.NoSuchRuleEnforcer()
    }
    case Variant.NoSuchRule: {
      return cases.NoSuchRule()
    }
    case Variant.WrongEnablerType: {
      return cases.WrongEnablerType()
    }
    case Variant.AlreadyIneffective: {
      return cases.WrongEnablerType()
    }
    case Variant.WrongPassword: {
      return cases.WrongPassword()
    }
    case Variant.InternalError: {
      return cases.InternalError()
    }
  }
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `UserAccess.Operations.Enabler.ByPassword.MakeIneffective.Error.fromString: Unknown variant. Expected ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_ENFORCER_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(NO_SUCH_RULE_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(WRONG_PROTECTOR_TYPE_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(ALREADY_INEFFECTIVE_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(WRONG_PASSWORD_AS_STRING)
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
    case WRONG_PROTECTOR_TYPE_AS_STRING: {
      return Ok(WrongEnablerType())
    }
    case ALREADY_INEFFECTIVE_AS_STRING: {
      return Ok(AlreadyIneffective())
    }
    case WRONG_PASSWORD_AS_STRING: {
      return Ok(WrongPassword())
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
    NoSuchRule: () => NO_SUCH_RULE_AS_STRING,
    WrongEnablerType: () => WRONG_PROTECTOR_TYPE_AS_STRING,
    AlreadyIneffective: () => ALREADY_INEFFECTIVE_AS_STRING,
    WrongPassword: () => WRONG_PASSWORD_AS_STRING,
    InternalError: () => INTERNAL_ERROR_AS_STRING,
  })
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("Enabler.ByPassword.MakeIneffective.Error", asString(me))
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

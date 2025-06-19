import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as GenericError from "../../../../ElementaryTypes/GenericError.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";
import { Err, Ok, Tried } from "../../../../ElementaryTypes/Tried.ts";

const enum Variant {
  RuleEnforcerCreationLimitReached,
  RuleEnforcerAlreadyCreatedForUser,
  InternalErrorOrNoSuchUser,
  InternalError,
}

export interface Cases<A, B, C, D> {
  readonly RuleEnforcerCreationLimitReached: () => A
  readonly RuleEnforcerAlreadyCreatedForUser: () => B
  readonly InternalErrorOrNoSuchUser: () => C
  readonly InternalError: () => D
}

const RULE_ENFORCER_CREATION_LIMIT_REACHED_AS_STRING = "RuleEnforcerCreationLimitReached"
const RULE_ENFORCER_ALREADY_CREATED_FOR_USER_AS_STRING = "RuleEnforcerAlreadyCreatedForUser"
const INTERNAL_ERROR_OR_NO_SUCH_USER_AS_STRING = "InternalErrorOrNoSuchUser"
const INTERNAL_ERROR_AS_STRING = "InternalError"

export type Error = Unique<"App.NetworkingAccess.Operations.RuleEnforcer.Create.Error", Variant>

export function EnforcerCreationLimitReached(): Error {
  return Unique(Variant.RuleEnforcerCreationLimitReached)
}
export function RuleEnforcerAlreadyCreatedForUser(): Error {
  return Unique(Variant.RuleEnforcerAlreadyCreatedForUser)
}
export function InternalErrorOrNoSuchUser(): Error {
  return Unique(Variant.InternalErrorOrNoSuchUser)
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
    case Variant.RuleEnforcerCreationLimitReached: {
      return cases.RuleEnforcerCreationLimitReached()
    }
    case Variant.RuleEnforcerAlreadyCreatedForUser: {
      return cases.RuleEnforcerAlreadyCreatedForUser()
    }
    case Variant.InternalErrorOrNoSuchUser: {
      return cases.InternalErrorOrNoSuchUser()
    }
    case Variant.InternalError: {
      return cases.InternalError()
    }
  }
}

function UnknownVariantError(variant: string): GenericError.GenericError {
  return GenericError.create(
    `NetworkingAccess.Operations.RuleEnforcer.Create.Error.fromString: Unknown variant. Expected ${
      Displayer.stringDisplayer.display(RULE_ENFORCER_CREATION_LIMIT_REACHED_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(RULE_ENFORCER_ALREADY_CREATED_FOR_USER_AS_STRING)
    }, ${
      Displayer.stringDisplayer.display(INTERNAL_ERROR_OR_NO_SUCH_USER_AS_STRING)
    } or ${
      Displayer.stringDisplayer.display(INTERNAL_ERROR_AS_STRING)
    } but found ${
      Displayer.stringDisplayer.display(variant)
    }`
  )
}

export function fromString(variant: string): Tried<Error, GenericError.GenericError> {
  switch (variant) {
    case RULE_ENFORCER_CREATION_LIMIT_REACHED_AS_STRING: {
      return Ok(EnforcerCreationLimitReached())
    }
    case RULE_ENFORCER_ALREADY_CREATED_FOR_USER_AS_STRING: {
      return Ok(RuleEnforcerAlreadyCreatedForUser())
    }
    case INTERNAL_ERROR_OR_NO_SUCH_USER_AS_STRING: {
      return Ok(InternalErrorOrNoSuchUser())
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
    RuleEnforcerCreationLimitReached: () => RULE_ENFORCER_CREATION_LIMIT_REACHED_AS_STRING,
    RuleEnforcerAlreadyCreatedForUser: () => RULE_ENFORCER_ALREADY_CREATED_FOR_USER_AS_STRING,
    InternalErrorOrNoSuchUser: () => INTERNAL_ERROR_OR_NO_SUCH_USER_AS_STRING,
    InternalError: () => INTERNAL_ERROR_AS_STRING,
  })
}

export const displayer = Displayer.implement<Error>(me => 
  Displayer.asWrappedString("CreateRuleEnforcerError", asString(me))
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


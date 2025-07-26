import { Err, isNone, Ok, Option, Tried } from "../Prelude.ts";

export type IntermediateRepr = 
  | null
  | string 
  | number 
  | boolean
  | IntermediateRepr[]
  | { [key: string]: IntermediateRepr }

export type IntoIntermediateRepr<FinalRepr> = (finalRepr: FinalRepr) => IntermediateRepr

export type FromIntermediateRepr<FinalRepr> = (intermediateRepr: IntermediateRepr) => Option<FinalRepr>

export const enum ErrorType {
  FromIR,
  IntoIR,
  Serialize,
  Deserialize,
}

export interface FromIRError {
  readonly type: ErrorType.FromIR
  readonly intermediateRepr: IntermediateRepr
}

export interface IntoIRError<Data> {
  readonly type: ErrorType.IntoIR
  readonly data: Data
}

export interface SerializeError {
  readonly type: ErrorType.Serialize
  readonly intermediateRepr: IntermediateRepr
  readonly error: unknown
}

export interface DeserializeError {
  readonly type: ErrorType.Deserialize
  readonly json: string
  readonly error: unknown
}

export type Error = (
  | FromIRError
  // | IntoIRError<Data>
  | SerializeError
  | DeserializeError
)

export const Error = {
  FromIR(intermediateRepr: IntermediateRepr): FromIRError {
    return {
      type: ErrorType.FromIR,
      intermediateRepr,
    }
  },

  Serialize(error: unknown, intermediateRepr: IntermediateRepr): SerializeError {
    return {
      type: ErrorType.Serialize,
      error,
      intermediateRepr,
    }
  },

  Deserialize(json: string, error: unknown): DeserializeError {
    return {
      type: ErrorType.Deserialize,
      json,
      error,
    }
  }
}

export function serialize<Data>(data: Data, intoIR: IntoIntermediateRepr<Data>): Tried<string, Error> {
  const intermediateRepr = intoIR(data)

  try {
    return Ok(JSON.stringify(intermediateRepr, null, 0))
  } catch (error) {
    return Err(Error.Serialize(error, intermediateRepr))
  }
}

export function deserialize<Data>(json: string, fromIR: FromIntermediateRepr<Data>): Tried<Data, Error> {
  let intermediateRepr: IntermediateRepr

  try {
    intermediateRepr = JSON.parse(json) as IntermediateRepr
  } catch (error) {
    return Err(Error.Deserialize(json, error))
  }

  const finalRepr = fromIR(intermediateRepr)
  if (isNone(finalRepr)) {
    return Err(Error.FromIR(intermediateRepr))
  }

  return Ok(finalRepr.value)
}
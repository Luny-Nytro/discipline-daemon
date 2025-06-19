import * as Displayer from "../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import { Unique } from "../ElementaryTypes/Unique.ts";

const enum Type {
  Fetch,
  SerializeOperation,
  DeserializeOperationOutcome,
  BadResponse,
}

export type T = Unique<"App.Client.ExecuteError",{
  readonly type: Type.Fetch
  readonly error: unknown
} | {
  readonly type: Type.SerializeOperation;
  readonly error: JsonSerializer.Error
} | {
  readonly type: Type.DeserializeOperationOutcome;
  readonly error: JsonDeserializer.Error;
  readonly outcomeAsJson: string;
} | {
  readonly type: Type.BadResponse
  readonly statusCode: number
  readonly statusMessage: string
}>

export function Fetch(error: unknown): T {
  return Unique({
    type: Type.Fetch,
    error,
  })
}

export function SerializeOperation(
  error: JsonSerializer.Error,
): T {
  return Unique({
    type: Type.SerializeOperation,
    error,
  })
}

export function DeserializeOperationOutcome(
  error: JsonDeserializer.Error, 
  outcomeAsJson: string,
): T {
  return Unique({
    type: Type.DeserializeOperationOutcome,
    error,
    outcomeAsJson,
  })
}

export function BadResponse(
  statusCode: number,
  statusMessage: string,
): T {
  return Unique({
    type: Type.BadResponse,
    statusCode,
    statusMessage,
  })
}

export interface Cases<A, B, C, D> {
  readonly Fetch: (error: unknown) => A
  readonly SerializeOperation: (error: JsonSerializer.Error) => B
  readonly DeserializeOperationOutcome: (error: JsonDeserializer.Error, outcome: string) => C
  readonly BadResponse: (statusCode: number, statusMessage: string) => D
}

export function match<A, B, C, D>(
  me: T,
  cases: Cases<A, B, C, D>
) {
  switch (me.type) {
    case Type.Fetch: {
      return cases.Fetch(me.error)
    }
    case Type.SerializeOperation: {
      return cases.SerializeOperation(me.error)
    }
    case Type.DeserializeOperationOutcome: {
      return cases.DeserializeOperationOutcome(me.error, me.outcomeAsJson)
    }
    case Type.BadResponse: {
      return cases.BadResponse(me.statusCode, me.statusMessage)
    }
  }
}

export const displayer = Displayer.implement<T>(me => 
  match(me, {
    Fetch: error =>
      Displayer.asEnumStringVariant("ExecuteError", "Fetch", String(error)),

    SerializeOperation: error =>
      Displayer.asEnumDataVariantUsing("ExecuteError", "SerializeOperation", 
        JsonSerializer.errorDisplayer, error,
      ),

    DeserializeOperationOutcome: (error, outcome) => 
      Displayer.asEnumDataVariant("ExecuteError", "DeserializeOperationOutcome", 
        Displayer.asUnnamedObject(
          "error", JsonDeserializer.errorDisplayer, error,
          "outcome", Displayer.stringDisplayer, outcome,
        )
      ),

    BadResponse: (statusCode, statusMessage) => 
      Displayer.asEnumDataVariant("ExecuteError", "BadResponse",
        Displayer.asUnnamedObject(
          "statusCode", Displayer.numberDisplayer, statusCode,
          "statusMessage", Displayer.stringDisplayer, statusMessage,
        )
      )
  })
);
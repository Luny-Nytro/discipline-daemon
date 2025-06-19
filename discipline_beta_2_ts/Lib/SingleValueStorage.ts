import { Err, Ok, Tried, Option, None, Some, isErr, isSome } from "../Prelude.ts";

export const enum ErrorType {
  Unknown,
  Serialize,
  Deserialize,
}

export interface UnknownError {
  readonly type: ErrorType.Unknown
  readonly error: unknown
}

export function UnknownError(error: unknown): UnknownError {
  return {
    type: ErrorType.Unknown,
    error,
  }
}

export interface SerializeError<Data, Error> {
  readonly type: ErrorType.Serialize
  readonly data: Data
  readonly error: Error
}

export function SerializeError<Data, Error>(data: Data, error: Error): SerializeError<Data, Error> {
  return {
    type: ErrorType.Serialize,
    data,
    error,
  }
}

export interface DeserializeError<Error> {
  readonly type: ErrorType.Deserialize
  readonly text: string
  readonly error: Error
}

export function DeserializeError<Error>(text: string, error: Error): DeserializeError<Error> {
  return {
    type: ErrorType.Deserialize,
    text,
    error,
  }
}

export const Error = {
  Unknown: UnknownError,
  Serialize: SerializeError,
  Deserialize: DeserializeError,
}

export type Error<Data, SerializerError, DeserializerError> = (
  | UnknownError
  | SerializeError<Data, SerializerError>
  | DeserializeError<DeserializerError>
)

async function FileSystem_read(path: string) {
  try {
    return Ok(await Deno.readTextFile(path))
  } catch (error) {
    return Err(Error.Unknown(error))
  }
}

async function FileSystem_write(path: string, newValue: string) {
  try {
    await Deno.writeTextFile(path, newValue)
    return None()
  } catch (error) {
    return Some(Error.Unknown(error))
  }
}

export interface SingleValueStorage<Value, SerializerError, DeserializerError> {
  load(): Promise<Tried<Value, Error<Value, SerializerError, DeserializerError>>>
  update(newValue: Value): Promise<Option<Error<Value, SerializerError, DeserializerError>>>
}

export const SingleValueStorage = {
  new<Value, SerializerError, DeserializerError>(
    path: string,
    fallback: () => Value,
    serializer: (value: Value) => Tried<string, SerializerError>,
    deserializer: (serializedValue: string) => Tried<Value, DeserializerError>,
  ): SingleValueStorage<Value, SerializerError, DeserializerError> {
    async function load(): Promise<Tried<Value, Error<Value, SerializerError, DeserializerError>>> {
      const maybeSerializedValue = await FileSystem_read(path)
      if (isErr(maybeSerializedValue)) {
        // console.log(maybeSerializedValue.error.error)
        if (!(maybeSerializedValue.error.error instanceof Deno.errors.NotFound)) {
          return maybeSerializedValue
        }

        const value = fallback()
        const maybeError = await write(value)
        if (isSome(maybeError)) {
          return Err(maybeError.value)
        }

        return Ok(value)
      }
  
      const maybeDeserializedValue = deserializer(maybeSerializedValue.value)
      if (isErr(maybeDeserializedValue)) {
        return Err(Error.Deserialize(maybeSerializedValue.value, maybeDeserializedValue.error))
      }
  
      return Ok(maybeDeserializedValue.value)
    }
  
    async function write(newValue: Value): Promise<Option<Error<Value, SerializerError, DeserializerError>>> {
      const maybeValue = serializer(newValue)
      if (isErr(maybeValue)) {
        return Some(SerializeError(newValue, maybeValue.error))
      } else {
        return await FileSystem_write(path, maybeValue.value)
      }
    }

    return {
      load,
      update: write,
    }
  }
}
export const enum Type {
  Err,
  Ok,
}

export interface Ok<T> {
  readonly type: Type.Ok
  readonly value: T
}

export interface Err<E> {
  readonly type: Type.Err
  readonly error: E 
}

export type Tried<T, E> = Ok<T> | Err<E>

export function Ok<T>(value: T): Ok<T> {
  return {
    type: Type.Ok,
    value,
  }
}

export function Err<E>(error: E): Err<E> {
  return {
    type: Type.Err,
    error,
  }
}

export function isOk<T, E>(me: Tried<T, E>): me is Ok<T> {
  return me.type === Type.Ok
}

export function isErr<T, E>(me: Tried<T, E>): me is Err<E> {
  return me.type === Type.Err
}

export const Tried = {
  
}
import { Err, isOk, Ok, Tried } from "./Tried.ts"
import * as Displayer from "./Display.ts"
import * as JsonSerializer from "./JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "./JsonSerde/JsonDeserializer.ts";

const enum Type {
  None,
  Some,
}

export interface None {
  readonly type: Type.None
}

export interface Some<Value> {
  readonly type: Type.Some
  readonly value: Value
}

export function None(): None {
  return {
    type: Type.None
  }
}

export function Some<Value>(value: Value): Some<Value> {
  return {
    type: Type.Some,
    value,
  }
}

export type Option<Value> = None | Some<Value>

export function isNone<Value>(me: Option<Value>): me is None {
  return me.type === Type.None
}

export function isSome<Value>(me: Option<Value>): me is Some<Value> {
  return me.type === Type.Some
}

export function value<Value>(me: Some<Value>): Value {
  return me.value
}

export function map<Value, Return>(
  me: Option<Value>, 
  fn: (value: Value) => Return,
): Option<Return> {
  return isSome(me)
    ? Some(fn(me.value))
    : None()
}

export function map2<Value1, Value2, Return>(
  me1: Option<Value1>,
  me2: Option<Value2>,
  fn: (value1: Value1, value2: Value2) => Return
): Option<Return> {
  return isSome(me1) && isSome(me2)
    ? Some(fn(me1.value, me2.value))
    : None()
}

export function map3<Value1, Value2, Value3, Return>(
  me1: Option<Value1>,
  me2: Option<Value2>,
  me3: Option<Value3>,
  fn: (value1: Value1, value2: Value2, value3: Value3) => Return
): Option<Return> {
  return isSome(me1) && isSome(me2) && isSome(me3)
    ? Some(fn(me1.value, me2.value, me3.value))
    : None()
}

export function map4<Value1, Value2, Value3, Value4, Return>(
  me1: Option<Value1>,
  me2: Option<Value2>,
  me3: Option<Value3>,
  me4: Option<Value4>,
  fn: (v1: Value1, v2: Value2, v3: Value3, v4: Value4) => Return
): Option<Return> {
  return isSome(me1) && isSome(me2) && isSome(me3) && isSome(me4)
    ? Some(fn(me1.value, me2.value, me3.value, me4.value))
    : None()
}

export function map5<V1, V2, V3, V4, V5, R>(
  m1: Option<V1>, m2: Option<V2>, m3: Option<V3>, m4: Option<V4>, m5: Option<V5>,
  fn: (v1: V1, v2: V2, v3: V3, v4: V4, v5: V5) => R
): Option<R> {
  return isSome(m1) && isSome(m2) && isSome(m3) && isSome(m4) && isSome(m5)
    ? Some(fn(m1.value, m2.value, m3.value, m4.value, m5.value))
    : None()
}

export function map6<V1, V2, V3, V4, V5, V6, R>(
  m1: Option<V1>, m2: Option<V2>, m3: Option<V3>, 
  m4: Option<V4>, m5: Option<V5>, m6: Option<V6>,
  fn: (v1: V1, v2: V2, v3: V3, v4: V4, v5: V5, v6: V6) => R
): Option<R> {
  return isSome(m1) && isSome(m2) && isSome(m3) && 
         isSome(m4) && isSome(m5) && isSome(m6)
    ? Some(fn(m1.value, m2.value, m3.value, 
              m4.value, m5.value, m6.value))
    : None()
}

export function map7<V1, V2, V3, V4, V5, V6, V7, R>(
  m1: Option<V1>, m2: Option<V2>, m3: Option<V3>, m4: Option<V4>,
  m5: Option<V5>, m6: Option<V6>, m7: Option<V7>,
  fn: (v1: V1, v2: V2, v3: V3, v4: V4, v5: V5, v6: V6, v7: V7) => R
): Option<R> {
  return isSome(m1) && isSome(m2) && isSome(m3) && isSome(m4) &&
         isSome(m5) && isSome(m6) && isSome(m7)
    ? Some(fn(m1.value, m2.value, m3.value, m4.value,
              m5.value, m6.value, m7.value))
    : None()
}

export function map8<V1, V2, V3, V4, V5, V6, V7, V8, R>(
  m1: Option<V1>, m2: Option<V2>, m3: Option<V3>, m4: Option<V4>,
  m5: Option<V5>, m6: Option<V6>, m7: Option<V7>, m8: Option<V8>,
  fn: (v1: V1, v2: V2, v3: V3, v4: V4, 
       v5: V5, v6: V6, v7: V7, v8: V8) => R
): Option<R> {
  return isSome(m1) && isSome(m2) && isSome(m3) && isSome(m4) &&
         isSome(m5) && isSome(m6) && isSome(m7) && isSome(m8)
    ? Some(fn(m1.value, m2.value, m3.value, m4.value,
              m5.value, m6.value, m7.value, m8.value))
    : None()
}

export function map9<V1, V2, V3, V4, V5, V6, V7, V8, V9, R>(
  m1: Option<V1>, m2: Option<V2>, m3: Option<V3>, m4: Option<V4>,
  m5: Option<V5>, m6: Option<V6>, m7: Option<V7>, m8: Option<V8>,
  m9: Option<V9>,
  fn: (v1: V1, v2: V2, v3: V3, v4: V4, v5: V5, 
       v6: V6, v7: V7, v8: V8, v9: V9) => R
): Option<R> {
  return isSome(m1) && isSome(m2) && isSome(m3) && isSome(m4) &&
         isSome(m5) && isSome(m6) && isSome(m7) && isSome(m8) &&
         isSome(m9)
    ? Some(fn(m1.value, m2.value, m3.value, m4.value,
              m5.value, m6.value, m7.value, m8.value,
              m9.value))
    : None()
}

export function map10<V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, R>(
  m1: Option<V1>, m2: Option<V2>, m3: Option<V3>, m4: Option<V4>,
  m5: Option<V5>, m6: Option<V6>, m7: Option<V7>, m8: Option<V8>,
  m9: Option<V9>, m10: Option<V10>,
  fn: (v1: V1, v2: V2, v3: V3, v4: V4, v5: V5, 
       v6: V6, v7: V7, v8: V8, v9: V9, v10: V10) => R
): Option<R> {
  return isSome(m1) && isSome(m2) && isSome(m3) && isSome(m4) &&
         isSome(m5) && isSome(m6) && isSome(m7) && isSome(m8) &&
         isSome(m9) && isSome(m10)
    ? Some(fn(m1.value, m2.value, m3.value, m4.value,
              m5.value, m6.value, m7.value, m8.value,
              m9.value, m10.value))
    : None()
}

export function mapOr<Value, Return>(
  me: Option<Value>,
  defaultValue: Return,
  fn: (value: Value) => Return,
): Return {
  return isSome(me) ? fn(me.value) : defaultValue
}

export function mapOrElse<Value, Return>(
  me: Option<Value>,
  defaultFn: () => Return,
  fn: (value: Value) => Return,
): Return {
  return isSome(me) ? fn(me.value) : defaultFn()
}

export function and<Value, OtherValue>(
  me: Option<Value>,
  other: Option<OtherValue>,
): Option<OtherValue> {
  return isSome(me) ? other : None()
}

export function andThen<Value, Return>(
  me: Option<Value>,
  fn: (value: Value) => Option<Return>,
): Option<Return> {
  return isSome(me) ? fn(me.value) : None()
}

export function andThen2<Value1, Value2, Return>(
  me1: Option<Value1>,
  me2: Option<Value2>,
  fn: (value1: Value1, value2: Value2) => Option<Return>,
): Option<Return> {
  return isSome(me1) && isSome(me2) ? fn(me1.value, me2.value) : None()
}

export function andThen3<Value1, Value2, Value3, Return>(
  me1: Option<Value1>,
  me2: Option<Value2>,
  me3: Option<Value3>,
  fn: (value1: Value1, value2: Value2, value3: Value3) => Option<Return>,
): Option<Return> {
  return isSome(me1) && isSome(me2) && isSome(me3) 
    ? fn(me1.value, me2.value, me3.value) 
    : None()
}

export function andThen4<Value1, Value2, Value3, Value4, Return>(
  me1: Option<Value1>,
  me2: Option<Value2>,
  me3: Option<Value3>,
  me4: Option<Value4>,
  fn: (value1: Value1, value2: Value2, value3: Value3, value: Value4) => Option<Return>,
): Option<Return> {
  return isSome(me1) && isSome(me2) && isSome(me3) && isSome(me4)
    ? fn(me1.value, me2.value, me3.value, me4.value) 
    : None()
}

export function or<Value>(
  me: Option<Value>,
  other: Option<Value>,
): Option<Value> {
  return isSome(me) ? me : other
}

export function orElse<Value>(
  me: Option<Value>,
  fn: () => Option<Value>,
): Option<Value> {
  return isSome(me) ? me : fn()
}

// TODO: Rename the "orFn*" functions to "orLazy*". 
export function orFn2<Value1, Value2>(
  fn1: () => Option<Value1>,
  fn2: () => Option<Value2>,
): Option<Value1 | Value2> {
  const option1 = fn1()
  if (isSome(option1)) {
    return option1
  }

  return fn2()
}

export function orFn9<
  Value1,
  Value2,
  Value3,
  Value4,
  Value5,
  Value6,
  Value7,
  Value8,
  Value9
>(
  fn1: () => Option<Value1>,
  fn2: () => Option<Value2>,
  fn3: () => Option<Value3>,
  fn4: () => Option<Value4>,
  fn5: () => Option<Value5>,
  fn6: () => Option<Value6>,
  fn7: () => Option<Value7>,
  fn8: () => Option<Value8>,
  fn9: () => Option<Value9>
): Option<
  | Value1
  | Value2
  | Value3
  | Value4
  | Value5
  | Value6
  | Value7
  | Value8
  | Value9
> {
  const option1 = fn1()
  if (isSome(option1)) return option1

  const option2 = fn2()
  if (isSome(option2)) return option2

  const option3 = fn3()
  if (isSome(option3)) return option3

  const option4 = fn4()
  if (isSome(option4)) return option4

  const option5 = fn5()
  if (isSome(option5)) return option5

  const option6 = fn6()
  if (isSome(option6)) return option6

  const option7 = fn7()
  if (isSome(option7)) return option7

  const option8 = fn8()
  if (isSome(option8)) return option8

  return fn9()
}


export function xor<Value>(
  me: Option<Value>,
  other: Option<Value>,
): Option<Value> {
  return (isSome(me) && isNone(other)) 
    ? me 
    : (isNone(me) && isSome(other)) 
      ? other 
      : None()
}

export function filter<Value>(
  me: Option<Value>,
  predicate: (value: Value) => boolean,
): Option<Value> {
  return isSome(me) && predicate(me.value) ? me : None()
}

// ========== Boolean Operations ==========

export function contains<Value>(
  me: Option<Value>,
  value: Value,
): boolean {
  return isSome(me) && me.value === value
}

// ========== Extraction Methods ==========

export function unwrap<Value>(me: Option<Value>): Value {
  if (isSome(me)) {
    return me.value
  } else {
    throw new Error("Called `unwrap()` on a `None` value")
  }
}

export function unwrapOr<Value>(
  me: Option<Value>,
  defaultValue: Value,
): Value {
  return isSome(me) ? me.value : defaultValue
}

export function unwrapOrElse<Value>(
  me: Option<Value>,
  fn: () => Value,
): Value {
  return isSome(me) ? me.value : fn()
}

export function expect<Value>(
  me: Option<Value>,
  msg: string,
): Value {
  if (isSome(me)) return me.value
  throw new Error(msg)
}

// ========== Conversion Methods ==========

export function okOr<Value, Error>(
  me: Option<Value>,
  error: Error,
): Tried<Value, Error> {
  return isSome(me) ? Ok(me.value) : Err(error)
}

export function okOrElse<Value, Error>(
  me: Option<Value>,
  errorFn: () => Error,
): Tried<Value, Error> {
  return isSome(me) ? Ok(me.value) : Err(errorFn())
}

// ========== Utility Methods ==========

export interface OptionCases<Value, A, B> {
  readonly Some: (value: Value) => A
  readonly None: () => B
}

export function match<Value, A, B>(
  me: Option<Value>,
  cases: OptionCases<Value, A, B>
): A | B {
  return isSome(me) ? cases.Some(me.value) : cases.None()
}

export function zip<Value1, Value2>(
  a: Option<Value1>,
  b: Option<Value2>,
): Option<[Value1, Value2]> {
  return isSome(a) && isSome(b) ? Some([a.value, b.value]) : None()
}

export function zipWith<Value1, Value2, Return>(
  a: Option<Value1>,
  b: Option<Value2>,
  fn: (a: Value1, b: Value2) => Return,
): Option<Return> {
  return isSome(a) && isSome(b) ? Some(fn(a.value, b.value)) : None()
}

// ========== Collection Methods ==========

export function transpose<Value, Error>(
  me: Option<Tried<Value, Error>>,
): Tried<Option<Value>, Error> {
  if (isNone(me)) return Ok(None())
  const inner = me.value
  return isOk(inner) ? Ok(Some(inner.value)) : Err(inner.error)
}

// ========== Boolean Checks ==========

export function isSomeAnd<Value>(
  me: Option<Value>,
  predicate: (value: Value) => boolean,
): boolean {
  return isSome(me) && predicate(me.value)
}

export function isEqualTo<Value>(
  a: Option<Value>, 
  b: Option<Value>, 
  fn: (a: Value, b: Value) => boolean
) { 
  return (
    isNone(a) && isNone(b)
  ) || (
    isSome(a) && isSome(b) && fn(value(a), value(b))
  )
}

export function fromMaybeUndefined<T>(value: T | undefined): Option<T> {
  return value === undefined
    ? None()
    : Some(value)
}

export const Option = {
  fromMaybeUndefined,
  None,
  Some,
  isNone,
  isSome,
  map,
  map2,
  map3,
  map4,
  map5,
  map6,
  map7,
  map8,
  map9,
  map10,
  mapOr,
  mapOrElse,
  and,
  andThen,
  andThen2,
  andThen3,
  andThen4,
  or,
  orElse,
  xor,
  filter,
  contains,
  unwrap,
  unwrapOr,
  unwrapOrElse,
  expect,
  okOr,
  okOrElse,
  match,
  zip,
  zipWith,
  transpose,
  isSomeAnd,
  value,
  orFn2,
  orFn9,
  isEqualTo,

  JsonSerializer: <Value>(valueSerializer: JsonSerializer.JsonSerializer<Value>) => 
    JsonSerializer.implement<Option<Value>>(me =>
      match(me, {
        None: () => 
          JsonSerializer.asNull(),
        Some: (value) => 
          JsonSerializer.as(valueSerializer, value),
      })
    ),

  JsonDeserializer: <Value>(valueDeserializer: JsonDeserializer.JsonDeserializer<Value>) =>
    JsonDeserializer.implement<Option<Value>>(context => 
      Tried.match(JsonDeserializer.asNull(context), {
        Ok: () => 
          Ok(None()),

        Err: () => 
          Tried.map(JsonDeserializer.as(context, valueDeserializer), Some)
      })
    ),

  Displayer: <Value>(valueDisplayer: Displayer.Inspector<Value>) =>
    Displayer.implement<Option<Value>>(me =>
      match(me, {
        None: () => 
          Displayer.asEnumUnitVariant(
            "Option", "None"
          ),

        Some: value => 
          Displayer.asEnumDataVariantUsing(
            "Option", "Some", valueDisplayer, value
          )
      })
    )
}
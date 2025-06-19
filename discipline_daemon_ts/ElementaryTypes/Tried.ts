import { None, Option, Some } from "./Option.ts"

const enum Type {
  Ok,
  Err,
}

export interface Ok<Value> {
  readonly type: Type.Ok
  readonly value: Value
}

export function Ok<Value>(value: Value): Ok<Value> {
  return {
    type: Type.Ok,
    value,
  }
}

export interface Err<Error> {
  readonly type: Type.Err
  readonly error: Error
}

export function Err<Error>(error: Error): Err<Error> {
  return {
    type: Type.Err,
    error,
  }
}

export type Tried<Value, Error> = Ok<Value> | Err<Error>

// export function lazyOk<Value, Args extends unknown[]>(
//   fn: (...args: Args) => Value
// ): 
//   (...args: Args) => Ok<Value>
// {
//   return (...args: Args) => Ok(fn(...args))
// }

export function isOk<Value, Error>(me: Tried<Value, Error>): me is Ok<Value> {
  return me.type === Type.Ok
}

export function isErr<Value, Error>(me: Tried<Value, Error>): me is Err<Error> {
  return me.type === Type.Err
}

export function map<Value, NewValue, Error>(
  me: Tried<Value, Error>,
  fn: (value: Value) => NewValue
): Tried<NewValue, Error> {
  return isOk(me) ? Ok(fn(me.value)) : me
}

export function mapErr<Value, Error, NewError>(
  me: Tried<Value, Error>,
  fn: (error: Error) => NewError
): Tried<Value, NewError> {
  return isErr(me) ? Err(fn(me.error)) : me
}

export function lazyAndThen<Value, NewValue, Error>(
  fn: (value: Value) => Tried<NewValue, Error>
) {
  return (me: Tried<Value, Error>) => 
    isOk(me) ? fn(me.value) : me
}

export function andThen<Value, NewValue, Error>(
  me: Tried<Value, Error>,
  fn: (value: Value) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  return isOk(me) ? fn(me.value) : me
}

export function andThen1<Value1, NewValue, Error>(
  me1: Tried<Value1, Error>,
  fn: (value1: Value1) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) {
    return me1
  }

  return fn(me1.value)
}

export function andThen2<Value1, Value2, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  fn: (value1: Value1, value2: Value2) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) {
    return me1
  }

  if (isErr(me2)) {
    return me2
  }

  return fn(me1.value, me2.value)
}

export function andThen3<Value1, Value2, Value3, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  me3: Tried<Value3, Error>,
  fn: (v1: Value1, v2: Value2, v3: Value3) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) return me1
  if (isErr(me2)) return me2
  if (isErr(me3)) return me3

  return fn(me1.value, me2.value, me3.value)
}

export function andThen4<Value1, Value2, Value3, Value4, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  me3: Tried<Value3, Error>,
  me4: Tried<Value4, Error>,
  fn: (v1: Value1, v2: Value2, v3: Value3, v4: Value4) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) return me1
  if (isErr(me2)) return me2
  if (isErr(me3)) return me3
  if (isErr(me4)) return me4

  return fn(me1.value, me2.value, me3.value, me4.value)
}

export function andThen5<Value1, Value2, Value3, Value4, Value5, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  me3: Tried<Value3, Error>,
  me4: Tried<Value4, Error>,
  me5: Tried<Value5, Error>,
  fn: (v1: Value1, v2: Value2, v3: Value3, v4: Value4, v5: Value5) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) return me1
  if (isErr(me2)) return me2
  if (isErr(me3)) return me3
  if (isErr(me4)) return me4
  if (isErr(me5)) return me5

  return fn(me1.value, me2.value, me3.value, me4.value, me5.value)
}

export function andThen6<Value1, Value2, Value3, Value4, Value5, Value6, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  me3: Tried<Value3, Error>,
  me4: Tried<Value4, Error>,
  me5: Tried<Value5, Error>,
  me6: Tried<Value6, Error>,
  fn: (v1: Value1, v2: Value2, v3: Value3, v4: Value4, v5: Value5, v6: Value6) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) return me1
  if (isErr(me2)) return me2
  if (isErr(me3)) return me3
  if (isErr(me4)) return me4
  if (isErr(me5)) return me5
  if (isErr(me6)) return me6

  return fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value)
}

export function andThen7<Value1, Value2, Value3, Value4, Value5, Value6, Value7, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  me3: Tried<Value3, Error>,
  me4: Tried<Value4, Error>,
  me5: Tried<Value5, Error>,
  me6: Tried<Value6, Error>,
  me7: Tried<Value7, Error>,
  fn: (v1: Value1, v2: Value2, v3: Value3, v4: Value4, v5: Value5, v6: Value6, v7: Value7) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) return me1
  if (isErr(me2)) return me2
  if (isErr(me3)) return me3
  if (isErr(me4)) return me4
  if (isErr(me5)) return me5
  if (isErr(me6)) return me6
  if (isErr(me7)) return me7

  return fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value, me7.value)
}

export function andThen8<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  me3: Tried<Value3, Error>,
  me4: Tried<Value4, Error>,
  me5: Tried<Value5, Error>,
  me6: Tried<Value6, Error>,
  me7: Tried<Value7, Error>,
  me8: Tried<Value8, Error>,
  fn: (v1: Value1, v2: Value2, v3: Value3, v4: Value4, v5: Value5, v6: Value6, v7: Value7, v8: Value8) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) return me1
  if (isErr(me2)) return me2
  if (isErr(me3)) return me3
  if (isErr(me4)) return me4
  if (isErr(me5)) return me5
  if (isErr(me6)) return me6
  if (isErr(me7)) return me7
  if (isErr(me8)) return me8

  return fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value, me7.value, me8.value)
}

export function andThen9<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  me3: Tried<Value3, Error>,
  me4: Tried<Value4, Error>,
  me5: Tried<Value5, Error>,
  me6: Tried<Value6, Error>,
  me7: Tried<Value7, Error>,
  me8: Tried<Value8, Error>,
  me9: Tried<Value9, Error>,
  fn: (v1: Value1, v2: Value2, v3: Value3, v4: Value4, v5: Value5, v6: Value6, v7: Value7, v8: Value8, v9: Value9) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) return me1
  if (isErr(me2)) return me2
  if (isErr(me3)) return me3
  if (isErr(me4)) return me4
  if (isErr(me5)) return me5
  if (isErr(me6)) return me6
  if (isErr(me7)) return me7
  if (isErr(me8)) return me8
  if (isErr(me9)) return me9

  return fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value, me7.value, me8.value, me9.value)
}

export function andThen10<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, NewValue, Error>(
  me1: Tried<Value1, Error>,
  me2: Tried<Value2, Error>,
  me3: Tried<Value3, Error>,
  me4: Tried<Value4, Error>,
  me5: Tried<Value5, Error>,
  me6: Tried<Value6, Error>,
  me7: Tried<Value7, Error>,
  me8: Tried<Value8, Error>,
  me9: Tried<Value9, Error>,
  me10: Tried<Value10, Error>,
  fn: (v1: Value1, v2: Value2, v3: Value3, v4: Value4, v5: Value5, v6: Value6, v7: Value7, v8: Value8, v9: Value9, v10: Value10) => Tried<NewValue, Error>
): Tried<NewValue, Error> {
  if (isErr(me1)) return me1
  if (isErr(me2)) return me2
  if (isErr(me3)) return me3
  if (isErr(me4)) return me4
  if (isErr(me5)) return me5
  if (isErr(me6)) return me6
  if (isErr(me7)) return me7
  if (isErr(me8)) return me8
  if (isErr(me9)) return me9
  if (isErr(me10)) return me10

  return fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value, me7.value, me8.value, me9.value, me10.value)
}

export function andThenFn3<Value1, Error1, Value2, Error2, Value3, Error3, Value4>(
  value1: Value1,
  fn1: (value1: Value1) => Tried<Value2, Error1>,
  fn2: (value2: Value2) => Tried<Value3, Error2>,
  fn3: (value3: Value3) => Tried<Value4, Error3>,
): Tried<Value4, Error1 | Error2 | Error3> {
  const tried1 = fn1(value1)
  if (isErr(tried1)) {
    return tried1
  }
  const tried2 = fn2(value(tried1))
  if (isErr(tried2)) {
    return tried2
  }
  return fn3(value(tried2))
}


export function map1<Value1, Error1, ReturnValue>(
  me1: Tried<Value1, Error1>,
  fn: (value1: Value1) => ReturnValue
): Tried<ReturnValue, Error1> {
  if (isErr(me1)) {
    return me1
  } 
  return Ok(fn(me1.value))
}

export function map2<Value1, Error1, Value2, Error2, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  fn: (value1: Value1, value2: Value2) => ReturnValue
): Tried<ReturnValue, Error1 | Error2> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  return Ok(fn(me1.value, me2.value))
}
export function map3<Value1, Error1, Value2, Error2, Value3, Error3, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  me3: Tried<Value3, Error3>,
  fn: (value1: Value1, value2: Value2, value3: Value3) => ReturnValue
): Tried<ReturnValue, Error1 | Error2 | Error3> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  if (isErr(me3)) {
    return me3
  }
  return Ok(fn(me1.value, me2.value, me3.value))
}

export function map4<Value1, Error1, Value2, Error2, Value3, Error3, Value4, Error4, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  me3: Tried<Value3, Error3>,
  me4: Tried<Value4, Error4>,
  fn: (value1: Value1, value2: Value2, value3: Value3, value4: Value4) => ReturnValue
): Tried<ReturnValue, Error1 | Error2 | Error3 | Error4> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  if (isErr(me3)) {
    return me3
  }
  if (isErr(me4)) {
    return me4
  }
  return Ok(fn(me1.value, me2.value, me3.value, me4.value))
}

export function map5<Value1, Error1, Value2, Error2, Value3, Error3, Value4, Error4, Value5, Error5, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  me3: Tried<Value3, Error3>,
  me4: Tried<Value4, Error4>,
  me5: Tried<Value5, Error5>,
  fn: (value1: Value1, value2: Value2, value3: Value3, value4: Value4, value5: Value5) => ReturnValue
): Tried<ReturnValue, Error1 | Error2 | Error3 | Error4 | Error5> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  if (isErr(me3)) {
    return me3
  }
  if (isErr(me4)) {
    return me4
  }
  if (isErr(me5)) {
    return me5
  }
  return Ok(fn(me1.value, me2.value, me3.value, me4.value, me5.value))
}

export function map6<Value1, Error1, Value2, Error2, Value3, Error3, Value4, Error4, Value5, Error5, Value6, Error6, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  me3: Tried<Value3, Error3>,
  me4: Tried<Value4, Error4>,
  me5: Tried<Value5, Error5>,
  me6: Tried<Value6, Error6>,
  fn: (value1: Value1, value2: Value2, value3: Value3, value4: Value4, value5: Value5, value6: Value6) => ReturnValue
): Tried<ReturnValue, Error1 | Error2 | Error3 | Error4 | Error5 | Error6> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  if (isErr(me3)) {
    return me3
  }
  if (isErr(me4)) {
    return me4
  }
  if (isErr(me5)) {
    return me5
  }
  if (isErr(me6)) {
    return me6
  }
  return Ok(fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value))
}

export function map7<Value1, Error1, Value2, Error2, Value3, Error3, Value4, Error4, Value5, Error5, Value6, Error6, Value7, Error7, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  me3: Tried<Value3, Error3>,
  me4: Tried<Value4, Error4>,
  me5: Tried<Value5, Error5>,
  me6: Tried<Value6, Error6>,
  me7: Tried<Value7, Error7>,
  fn: (value1: Value1, value2: Value2, value3: Value3, value4: Value4, value5: Value5, value6: Value6, value7: Value7) => ReturnValue
): Tried<ReturnValue, Error1 | Error2 | Error3 | Error4 | Error5 | Error6 | Error7> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  if (isErr(me3)) {
    return me3
  }
  if (isErr(me4)) {
    return me4
  }
  if (isErr(me5)) {
    return me5
  }
  if (isErr(me6)) {
    return me6
  }
  if (isErr(me7)) {
    return me7
  }
  return Ok(fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value, me7.value))
}

export function map8<Value1, Error1, Value2, Error2, Value3, Error3, Value4, Error4, Value5, Error5, Value6, Error6, Value7, Error7, Value8, Error8, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  me3: Tried<Value3, Error3>,
  me4: Tried<Value4, Error4>,
  me5: Tried<Value5, Error5>,
  me6: Tried<Value6, Error6>,
  me7: Tried<Value7, Error7>,
  me8: Tried<Value8, Error8>,
  fn: (value1: Value1, value2: Value2, value3: Value3, value4: Value4, value5: Value5, value6: Value6, value7: Value7, value8: Value8) => ReturnValue
): Tried<ReturnValue, Error1 | Error2 | Error3 | Error4 | Error5 | Error6 | Error7 | Error8> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  if (isErr(me3)) {
    return me3
  }
  if (isErr(me4)) {
    return me4
  }
  if (isErr(me5)) {
    return me5
  }
  if (isErr(me6)) {
    return me6
  }
  if (isErr(me7)) {
    return me7
  }
  if (isErr(me8)) {
    return me8
  }
  return Ok(fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value, me7.value, me8.value))
}

export function map9<Value1, Error1, Value2, Error2, Value3, Error3, Value4, Error4, Value5, Error5, Value6, Error6, Value7, Error7, Value8, Error8, Value9, Error9, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  me3: Tried<Value3, Error3>,
  me4: Tried<Value4, Error4>,
  me5: Tried<Value5, Error5>,
  me6: Tried<Value6, Error6>,
  me7: Tried<Value7, Error7>,
  me8: Tried<Value8, Error8>,
  me9: Tried<Value9, Error9>,
  fn: (value1: Value1, value2: Value2, value3: Value3, value4: Value4, value5: Value5, value6: Value6, value7: Value7, value8: Value8, value9: Value9) => ReturnValue
): Tried<ReturnValue, Error1 | Error2 | Error3 | Error4 | Error5 | Error6 | Error7 | Error8 | Error9> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  if (isErr(me3)) {
    return me3
  }
  if (isErr(me4)) {
    return me4
  }
  if (isErr(me5)) {
    return me5
  }
  if (isErr(me6)) {
    return me6
  }
  if (isErr(me7)) {
    return me7
  }
  if (isErr(me8)) {
    return me8
  }
  if (isErr(me9)) {
    return me9
  }
  return Ok(fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value, me7.value, me8.value, me9.value))
}

export function map10<Value1, Error1, Value2, Error2, Value3, Error3, Value4, Error4, Value5, Error5, Value6, Error6, Value7, Error7, Value8, Error8, Value9, Error9, Value10, Error10, ReturnValue>(
  me1: Tried<Value1, Error1>,
  me2: Tried<Value2, Error2>,
  me3: Tried<Value3, Error3>,
  me4: Tried<Value4, Error4>,
  me5: Tried<Value5, Error5>,
  me6: Tried<Value6, Error6>,
  me7: Tried<Value7, Error7>,
  me8: Tried<Value8, Error8>,
  me9: Tried<Value9, Error9>,
  me10: Tried<Value10, Error10>,
  fn: (value1: Value1, value2: Value2, value3: Value3, value4: Value4, value5: Value5, value6: Value6, value7: Value7, value8: Value8, value9: Value9, value10: Value10) => ReturnValue
): Tried<ReturnValue, Error1 | Error2 | Error3 | Error4 | Error5 | Error6 | Error7 | Error8 | Error9 | Error10> {
  if (isErr(me1)) {
    return me1
  }
  if (isErr(me2)) {
    return me2
  }
  if (isErr(me3)) {
    return me3
  }
  if (isErr(me4)) {
    return me4
  }
  if (isErr(me5)) {
    return me5
  }
  if (isErr(me6)) {
    return me6
  }
  if (isErr(me7)) {
    return me7
  }
  if (isErr(me8)) {
    return me8
  }
  if (isErr(me9)) {
    return me9
  }
  if (isErr(me10)) {
    return me10
  }
  return Ok(fn(me1.value, me2.value, me3.value, me4.value, me5.value, me6.value, me7.value, me8.value, me9.value, me10.value))
}


export function orElse<Value, Error, NewError>(
  me: Tried<Value, Error>,
  fn: (error: Error) => Tried<Value, NewError>
): Tried<Value, NewError> {
  return isErr(me) ? fn(me.error) : me
}

export function unwrapOr<Value, Error>(
  me: Tried<Value, Error>,
  fallbackValue: Value
): Value {
  return isOk(me) ? me.value : fallbackValue
}

export interface TriedCases<Value, Error, A, B> {
  readonly Ok: (value: Value) => A
  readonly Err: (error: Error) => B
}

export function match<Value, Error, A, B>(
  me: Tried<Value, Error>,
  cases: TriedCases<Value, Error, A, B>
): A | B {
  return isOk(me) ? cases.Ok(me.value) : cases.Err(me.error)
}

export function ok<Value, Error>(
  me: Tried<Value, Error>
): Option<Value> {
  return isOk(me) ? Some(me.value) : None()
}

export function unwrap<Value, Error>(me: Tried<Value, Error>): Value {
  return isOk(me) ? me.value : (() => { throw me.error })()
}

export function unwrapErr<Value, Error>(me: Tried<Value, Error>): Error {
  return isErr(me) ? me.error : (() => { throw me.value })()
}

export function error<Error>(me: Err<Error>): Error {
  return me.error
}

export function value<Value>(me: Ok<Value>): Value {
  return me.value
}

export function and2<A, E, B>(
  me1: Tried<A, E>,
  me2: Tried<B, E>
): Tried<B, E> {
  return isOk(me1) ? me2 : me1
}

export function and3<A, E, B, C>(
  me1: Tried<A, E>,
  me2: Tried<B, E>,
  me3: Tried<C, E>
): Tried<C, E> {
  return isOk(me1) ? (isOk(me2) ? me3 : me2) : me1
}

export function and4<A, E, B, C, D>(
  me1: Tried<A, E>,
  me2: Tried<B, E>,
  me3: Tried<C, E>,
  me4: Tried<D, E>
): Tried<D, E> {
  return isOk(me1)
    ? isOk(me2)
      ? isOk(me3)
        ? me4
        : me3
      : me2
    : me1
}

export function and5<A, E, B, C, D, F>(
  me1: Tried<A, E>,
  me2: Tried<B, E>,
  me3: Tried<C, E>,
  me4: Tried<D, E>,
  me5: Tried<F, E>
): Tried<F, E> {
  return isOk(me1)
    ? isOk(me2)
      ? isOk(me3)
        ? isOk(me4)
          ? me5
          : me4
        : me3
      : me2
    : me1
}

export function and6<A, E, B, C, D, F, G>(
  me1: Tried<A, E>,
  me2: Tried<B, E>,
  me3: Tried<C, E>,
  me4: Tried<D, E>,
  me5: Tried<F, E>,
  me6: Tried<G, E>
): Tried<G, E> {
  return isOk(me1)
    ? isOk(me2)
      ? isOk(me3)
        ? isOk(me4)
          ? isOk(me5)
            ? me6
            : me5
          : me4
        : me3
      : me2
    : me1
}

export function and7<A, E, B, C, D, F, G, H>(
  me1: Tried<A, E>,
  me2: Tried<B, E>,
  me3: Tried<C, E>,
  me4: Tried<D, E>,
  me5: Tried<F, E>,
  me6: Tried<G, E>,
  me7: Tried<H, E>
): Tried<H, E> {
  return isOk(me1)
    ? isOk(me2)
      ? isOk(me3)
        ? isOk(me4)
          ? isOk(me5)
            ? isOk(me6)
              ? me7
              : me6
            : me5
          : me4
        : me3
      : me2
    : me1
}

export function and8<A, E, B, C, D, F, G, H, I>(
  me1: Tried<A, E>,
  me2: Tried<B, E>,
  me3: Tried<C, E>,
  me4: Tried<D, E>,
  me5: Tried<F, E>,
  me6: Tried<G, E>,
  me7: Tried<H, E>,
  me8: Tried<I, E>
): Tried<I, E> {
  return isOk(me1)
    ? isOk(me2)
      ? isOk(me3)
        ? isOk(me4)
          ? isOk(me5)
            ? isOk(me6)
              ? isOk(me7)
                ? me8
                : me7
              : me6
            : me5
          : me4
        : me3
      : me2
    : me1
}

export function and9<A, E, B, C, D, F, G, H, I, J>(
  me1: Tried<A, E>,
  me2: Tried<B, E>,
  me3: Tried<C, E>,
  me4: Tried<D, E>,
  me5: Tried<F, E>,
  me6: Tried<G, E>,
  me7: Tried<H, E>,
  me8: Tried<I, E>,
  me9: Tried<J, E>
): Tried<J, E> {
  return isOk(me1)
    ? isOk(me2)
      ? isOk(me3)
        ? isOk(me4)
          ? isOk(me5)
            ? isOk(me6)
              ? isOk(me7)
                ? isOk(me8)
                  ? me9
                  : me8
                : me7
              : me6
            : me5
          : me4
        : me3
      : me2
    : me1
}

export function and10<A, E, B, C, D, F, G, H, I, J, K>(
  me1: Tried<A, E>,
  me2: Tried<B, E>,
  me3: Tried<C, E>,
  me4: Tried<D, E>,
  me5: Tried<F, E>,
  me6: Tried<G, E>,
  me7: Tried<H, E>,
  me8: Tried<I, E>,
  me9: Tried<J, E>,
  me10: Tried<K, E>
): Tried<K, E> {
  return isOk(me1)
    ? isOk(me2)
      ? isOk(me3)
        ? isOk(me4)
          ? isOk(me5)
            ? isOk(me6)
              ? isOk(me7)
                ? isOk(me8)
                  ? isOk(me9)
                    ? me10
                    : me9
                  : me8
                : me7
              : me6
            : me5
          : me4
        : me3
      : me2
    : me1
}

export const Tried = {
  Ok,
  Err,
  isOk,
  isErr,
  map,
  mapErr,
  unwrapOr,
  match,
  ok,
  unwrap,
  unwrapErr,
  error,
  value,

  // JsonSerializer: <Value, Error>(
  //   valueSerializer: JsonSerializer.JsonSerializer<Value>,
  //   errorSerializer: JsonSerializer.JsonSerializer<Error>,
  // ) => 
  //   JsonSerializer.implement<Tried<Value, Error>>(me => 
  //     match(me, {
  //       Ok: value => JsonSerializer.asEnumDataVariant(
  //         "Ok", valueSerializer, value,
  //       ),
  //       Err: error => JsonSerializer.asEnumDataVariant(
  //         "Err", errorSerializer, error,
  //       ),
  //     })
  //   ),
  
  // JsonDeserializer: <Value, Error>(
  //   valueDeserializer: JsonDeserializer.JsonDeserializer<Value>,
  //   errorDeserializer: JsonDeserializer.JsonDeserializer<Error>,
  // ) => 
  //   JsonDeserializer.implement<Tried<Value, Error>>(context => 
  //     JsonDeserializer.asEnum(context,
  //       JsonDeserializer.EnumDataVariant( 
  //         "Ok", valueDeserializer, Ok,
  //       ), 
  //       JsonDeserializer.EnumDataVariant( 
  //         "Err", errorDeserializer, Err,
  //       ), 
  //     )
  //   ),

  // Displayer: <Value, Error>(
  //   valueDisplayer: Displayer.Displayer<Value>,
  //   errorDisplayer: Displayer.Displayer<Error>,
  // ) => 
  //   Displayer.implement<Tried<Value, Error>>(me => 
  //     match(me, {
  //       Ok: value => Displayer.asEnumDataVariantUsing(
  //         "Tried", "Ok", valueDisplayer, value,
  //       ),
  //       Err: error => Displayer.asEnumDataVariantUsing(
  //         "Tried", "Err", errorDisplayer, error,
  //       ),
  //     })
  //   ),

  // lazyOk,
lazyAndThen,
andThen,
andThen1,
andThen2,
andThen3,
andThen4,
andThen5,
andThen6,
andThen7,
andThen8,
andThen9,
andThen10,
andThenFn3,
map1,
map2,
map3,
map4,
map5,
map6,
map7,
map8,
map9,
map10,
orElse,

and2,
and3,
and4,
and5,
and6,
and7,
and8,
and9,
and10,
}
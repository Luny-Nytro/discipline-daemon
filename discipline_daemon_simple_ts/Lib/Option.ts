export const enum Type {
  None,
  Some,
}

export interface Some<T> {
  readonly type: Type.Some
  readonly value: T
}

export interface None {
  readonly type: Type.None
}

export type Option<T> = None | Some<T>

export function Some<T>(value: T): Some<T> {
  return {
    type: Type.Some,
    value,
  }
}

export function None(): None {
  return {
    type: Type.None
  }
}

export function isSome<T>(me: Option<T>): me is Some<T> {
  return me.type === Type.Some
}

export function isNone<T>(me: Option<T>): me is None {
  return me.type === Type.None
}

export const Option = {
  eq<T>(a: Option<T>, b: Option<T>, eq: (a: T, b: T) => boolean): boolean {
    return (
      isNone(a) && isNone(b)
    ) || (
      isSome(a) && isSome(b) && eq(a.value, b.value)
    )
  },

  map<T, R>(me: Option<T>, fn: (value: T) => Option<R>): Option<R> {
    return isSome(me) ? fn(me.value) : None()
  },

  mapAll<T>(me: Option<T>, ...fns: ((value: T) => Option<T>)[]) {
    for (const fn of fns) {
      if (isSome(me)) {
        me = fn(me.value)
      } else {
        return None()
      }
    }
  },
  then1<Value1, Return>(
    option1: Option<Value1>, 
    fn: (value1: Value1) => Return
  ) {
    if (isSome(option1)) {
      return Some(fn(option1.value))
    } else {
      return None()
    }
  },
  then2<Value1, Value2, Return>(
    option1: Option<Value1>, 
    option2: Option<Value2>, 
    fn: (value1: Value1, value2: Value2) => Return
  ) {
    if (isSome(option1) && isSome(option2)) {
      return Some(fn(option1.value, option2.value))
    } else {
      return None()
    }
  },
  then3<Value1, Value2, Value3, Return>(
    option1: Option<Value1>, 
    option2: Option<Value2>, 
    option3: Option<Value3>, 
    fn: (value1: Value1, value2: Value2, value3: Value3) => Return
  ) {
    if (isSome(option1) && isSome(option2) && isSome(option3)) {
      return Some(fn(option1.value, option2.value, option3.value))
    } else {
      return None()
    }
  },
  then5<Value1, Value2, Value3, Value4, Value5, Return>(
    option1: Option<Value1>, 
    option2: Option<Value2>, 
    option3: Option<Value3>, 
    option4: Option<Value4>, 
    option5: Option<Value5>, 
    fn: (value1: Value1, value2: Value2, value3: Value3, value4: Value4, value5: Value5) => Return
  ) {
    if (isSome(option1) 
      && isSome(option2) 
      && isSome(option3)
      && isSome(option4)
      && isSome(option5)
    ) {
      return Some(fn(
        option1.value, 
        option2.value, 
        option3.value,
        option4.value,
        option5.value,
      ))
    } else {
      return None()
    }
  },
  then9<Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Return>(
    option1: Option<Value1>,
    option2: Option<Value2>,
    option3: Option<Value3>,
    option4: Option<Value4>,
    option5: Option<Value5>,
    option6: Option<Value6>,
    option7: Option<Value7>,
    option8: Option<Value8>,
    option9: Option<Value9>,
    fn: (
      value1: Value1,
      value2: Value2,
      value3: Value3,
      value4: Value4,
      value5: Value5,
      value6: Value6,
      value7: Value7,
      value8: Value8,
      value9: Value9,
    ) => Return
  ): Option<Return> {
    if (
      isSome(option1) &&
      isSome(option2) &&
      isSome(option3) &&
      isSome(option4) &&
      isSome(option5) &&
      isSome(option6) &&
      isSome(option7) &&
      isSome(option8) &&
      isSome(option9)
    ) {
      return Some(
        fn(
          option1.value,
          option2.value,
          option3.value,
          option4.value,
          option5.value,
          option6.value,
          option7.value,
          option8.value,
          option9.value
        )
      );
    } else {
      return None();
    }
  },
  
  flatten1<T>(me: Option<Option<T>>): Option<T> {
    if (isSome(me) && isSome(me.value)) {
      return me.value
    } else {
      return None()
    }
  },

  unwrap<T>(option: Option<T>): T {
    if (isSome(option)) {
      return option.value
    }
    throw new Error("Calling unwrap on None")
  },

  unwrapOrNull<T>(me: Option<T>): T | null {
    return isSome(me) ? me.value : null
  },

  unwrapOrUndefined<T>(me: Option<T>): T | undefined {
    return isSome(me) ? me.value : undefined
  },
  unwrap4<Value1, Value2, Value3, Value4>(
    option1: Option<Value1>,
    option2: Option<Value2>,
    option3: Option<Value3>,
    option4: Option<Value4>,
  ): [ Value1, Value2, Value3, Value4 ] {
    if (isSome(option1) 
      && isSome(option2) 
      && isSome(option3)
      && isSome(option4)
    ) {
      return [ option1.value, option2.value, option3.value, option4.value ]
    } else {
      throw new Error("Option.unwrap4: Some arguments are not Some.")
    }
  }
}

// type AndThenAll = (
//   | (<Value1, R>(
//       option1: Option<Value1>, 
//       fn: (value1: Value1) => Option<R>
//     ) => Option<R>)
//   | (<Value1, Value2, R>(
//       option1: Option<Value1>, 
//       option2: Option<Value2>, 
//       fn: (value1: Value1, value2: Value2) => Option<R>
//     ) => Option<R>)
//   | (<Value1, Value2, Value3, R>(
//       option1: Option<Value1>, 
//       option2: Option<Value2>, 
//       option3: Option<Value3>, 
//       fn: (value1: Value1, value2: Value2, value3: Value3) => Option<R>
//     ) => Option<R>)
// )
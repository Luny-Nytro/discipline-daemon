export class Ok<T, E> {
  readonly kind = "ok"

  constructor(readonly ok: T) {}
  
  mapOk<R>(fn: (ok: T) => R): Ok<R, E> {
    return new Ok(fn(this.ok))
  }
  
  mapErr<R>(_fn: (err: unknown) => R): Ok<T, E> {
    return this
  }
  
  unwrapOk() {
    return this.ok
  }

  unwrapOkOr(_ok: T): T {
    return this.ok
  }
  
  unwrapOkOrElse(_ok: () => T): T {
    return this.ok
  }
  
  unwrapErrOr(err: E): E {
    return err
  }
  
  unwrapErrOrElse<E>(err: () => E): E {
    return err()
  }

  clone(fn: (ok: T) => T): Ok<T, E> {
    return new Ok(fn(this.ok))
  }
}

export class Err<T, E> {
  readonly kind = "err"
  constructor(readonly err: E) {}
  
  mapOk<R>(_fn: (ok: T) => Tried<R, E>): Tried<T | R, E> {
    return this
  }
  
  mapErr<R>(fn: (err: E) => Tried<T, R>): Tried<T, R> {
    return fn(this.err)
  }

  unwrapOk(): T {
    throw this.err
  }

  unwrapOkOr(ok: T): T {
    return ok
  }
  
  unwrapOkOrElse(ok: () => T): T {
    return ok()
  }
  
  unwrapErr(): E {
    return this.err
  }

  unwrapErrOr(_err: E): E {
    return this.err
  }
  
  unwrapErrOrElse(err: () => E): E {
    return err()
  }

  clone(fn: (err: E) => E): Err<T, E> {
    return new Err(fn(this.err))
  }
}

export type Tried<T, E> = Ok<T, E> | Err<T, E> 
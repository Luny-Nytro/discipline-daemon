export class Some<T> {
  readonly kind = "some"
  constructor(readonly value: T) {}

  unwrap() {
    return this.value
  }
}

export class None<T> {
  readonly kind = "none"

  unwrap(): T {
    throw Error("")
  }
}

export type Option<T> = Some<T> | None<T>
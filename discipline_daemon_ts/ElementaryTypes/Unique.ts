export type Unique<Id, Type> = Type & { 
  readonly __________INTERNAL_VIRTUAL_TYPE_ID_DO_NOT_ACCESS__________: Id 
}

export function Unique<Id, Type>(value: Type) {
  return value as Unique<Id, Type>
}

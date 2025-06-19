import { None, Option } from "../Prelude.ts";
import { IntermediateRepr } from "./mod.ts";

export function isStruct(value: unknown): value is Record<any, unknown> {
  return typeof value === "object" && value !== null
}

export const Verbose = {
  fromIR3<Member1, Member2, Member3, Return>(
    ir: IntermediateRepr,
    key1: string,
    member1FromIR: (ir: IntermediateRepr) => Option<Member1>,
    key2: string,
    member2FromIR: (ir: IntermediateRepr) => Option<Member2>,
    key3: string,
    member3FromIR: (ir: IntermediateRepr) => Option<Member3>,
    then: (member1: Member1, member2: Member2, member3: Member3) => Return
  ): Option<Return> {
    if (isStruct(ir)) {
      return Option.then3(
        member1FromIR(ir[key1] ?? null),
        member2FromIR(ir[key2] ?? null),
        member3FromIR(ir[key3] ?? null),
        then,
      )
    } else {
      return None()
    }
  }
}
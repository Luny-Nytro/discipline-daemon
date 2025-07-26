import { None, Option } from "../Prelude.ts";
import { IntermediateRepr } from "./mod.ts";

export function fromIR1<Item0, Tuble>(
  ir: IntermediateRepr, 
  item0FromIR: (ir: IntermediateRepr) => Option<Item0>,
  then: (item0: Item0) => Tuble
): Option<Tuble> 
{
  if (!Array.isArray(ir)) {
    return None()
  }
  
  return Option.then1(
    item0FromIR(ir.at(0) ?? null),
    then
  )
}

export function fromIR2<Item0, Item1, Tuble>(
  ir: IntermediateRepr, 
  item0FromIR: (ir: IntermediateRepr) => Option<Item0>,
  item1FromIR: (ir: IntermediateRepr) => Option<Item1>,
  then: (item0: Item0, item1: Item1) => Tuble
): Option<Tuble> 
{
  if (!Array.isArray(ir)) {
    return None()
  }
  
  return Option.then2(
    item0FromIR(ir.at(0) ?? null),
    item1FromIR(ir.at(1) ?? null),
    then
  )
}

export function fromIR3<Item0, Item1, Item2, Tuble>(
  ir: IntermediateRepr, 
  item0FromIR: (ir: IntermediateRepr) => Option<Item0>,
  item1FromIR: (ir: IntermediateRepr) => Option<Item1>,
  item2FromIR: (ir: IntermediateRepr) => Option<Item2>,
  then: (item0: Item0, item1: Item1, item2: Item2) => Tuble
): Option<Tuble> {
  if (!Array.isArray(ir)) {
    return None()
  }
  
  return Option.then3(
    item0FromIR(ir.at(0) ?? null),
    item1FromIR(ir.at(1) ?? null),
    item2FromIR(ir.at(2) ?? null),
    then
  )
}

export function fromIR5<Item0, Item1, Item2, Item3, Item4, Tuble>(
  ir: IntermediateRepr, 
  item0FromIR: (ir: IntermediateRepr) => Option<Item0>,
  item1FromIR: (ir: IntermediateRepr) => Option<Item1>,
  item2FromIR: (ir: IntermediateRepr) => Option<Item2>,
  item3FromIR: (ir: IntermediateRepr) => Option<Item3>,
  item4FromIR: (ir: IntermediateRepr) => Option<Item4>,
  then: (item0: Item0, item1: Item1, item2: Item2, item3: Item3, item4: Item4) => Tuble
): Option<Tuble> {
  if (!Array.isArray(ir)) {
    return None()
  }
  
  return Option.then5(
    item0FromIR(ir.at(0) ?? null),
    item1FromIR(ir.at(1) ?? null),
    item2FromIR(ir.at(2) ?? null),
    item3FromIR(ir.at(3) ?? null),
    item4FromIR(ir.at(4) ?? null),
    then
  )
}
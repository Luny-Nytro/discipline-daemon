import { Discipline } from "Pkg";
import * as Serialize from "./serialize.ts"

export function discipline(discipline: Discipline) {
  return Serialize.serialize(Serialize.discipline(discipline))
}
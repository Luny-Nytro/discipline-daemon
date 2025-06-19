import { Serde, App, Option } from "../Prelude.ts";

export function intoIR(me: App): Serde.IntermediateRepr {
  return [
    Serde.Array.intoIR(me.dataHiders, Serde.DataHider.intoIR),
    Serde.MainUserAccessRegulator.intoIR(me.mainUserAccessRegulator),
    Serde.SuperUserAccessRegulator.intoIR(me.superUserAccessRegulator),
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<App> {
  return Serde.Tuble.fromIR3(
    ir,
    Serde.Array.FromIR(Serde.DataHider.fromIR),
    Serde.MainUserAccessRegulator.fromIR,
    Serde.SuperUserAccessRegulator.fromIR,
    App.constructor
  )
}

export function serialize(me: App) {
  return Serde.serialize(me, intoIR)
}

export function deserialize(json: string) {
  return Serde.deserialize(json, fromIR)
}
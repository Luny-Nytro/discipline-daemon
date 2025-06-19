import { SuperUserAccessRegulator, Serde, Option, } from "../Prelude.ts"

export function intoIR(me: SuperUserAccessRegulator): Serde.IntermediateRepr {
  return [
    Serde.String.intoIR(me.username),
    Serde.Boolean.intoIR(me.isLocked),
    Serde.String.intoIR(me.publicPassword),
    Serde.String.intoIR(me.privatePassword),
    Serde.CountdownTimer.intoIR(me.countdownTimer),
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<SuperUserAccessRegulator> {
  return Serde.Tuble.fromIR5(ir,
    Serde.String.fromIR,
    Serde.Boolean.fromIR,
    Serde.String.fromIR,
    Serde.String.fromIR,
    Serde.CountdownTimer.fromIR,
    SuperUserAccessRegulator.new
  )
}
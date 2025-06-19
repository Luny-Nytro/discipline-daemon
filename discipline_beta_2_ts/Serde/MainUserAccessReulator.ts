import { MainUserAccessRegulator, Option, Serde, } from "../Prelude.ts"

export function intoIR(me: MainUserAccessRegulator): Serde.IntermediateRepr {
  return [
    Serde.String.intoIR(me.username),
    Serde.Boolean.intoIR(me.isLocked),
    Serde.String.intoIR(me.publicPassword),
    Serde.String.intoIR(me.privatePassword),
    Serde.DailyAllowance.intoIR(me.dailyAllowance),
  ]
}

export function fromIR(ir: Serde.IntermediateRepr): Option<MainUserAccessRegulator> {
  return Serde.Tuble.fromIR5(ir,
    Serde.String.fromIR,
    Serde.Boolean.fromIR,
    Serde.String.fromIR,
    Serde.String.fromIR,
    Serde.DailyAllowance.fromIR,
    MainUserAccessRegulator.new
  )
}
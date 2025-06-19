import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as CountdownTimer from "../../ChronicTypes/CountdownTimer.ts"
import * as DateTime from "../../ChronicTypes/DateTime.ts"
import * as ByPasswordEnabler from "../../CommonTypes/ByPasswordEnabler.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Unique } from "../../ElementaryTypes/Unique.ts";

export interface EnablerCases<A, B> {
  readonly ForDuration: (timer: CountdownTimer.CountdownTimer) => A
  readonly ByPassword: (enabler: ByPasswordEnabler.ByPasswordEnabler) => B
}

export const enum EnablerType {
  ForDuration,
  ByPassword,
}

export type Enabler = Unique<"App.NetworkingAccess.Enabler", {
  readonly type: EnablerType.ForDuration
  readonly timer: CountdownTimer.CountdownTimer
} | {
  readonly type: EnablerType.ByPassword
  readonly enabler: ByPasswordEnabler.ByPasswordEnabler
}>

export function ForDuration(timer: CountdownTimer.CountdownTimer): Enabler {
  return Unique({
    type: EnablerType.ForDuration, 
    timer,
  })
}

export function ByPassword(enabler: ByPasswordEnabler.ByPasswordEnabler): Enabler {
  return Unique({
    type: EnablerType.ByPassword, 
    enabler: enabler,
  })
}

export function match<A, B>(me: Enabler, cases: EnablerCases<A, B>):
  A | B
{
  switch (me.type) {
    case EnablerType.ForDuration: {
      return cases.ForDuration(me.timer)
    }
    case EnablerType.ByPassword: {
      return cases.ByPassword(me.enabler)
    }
  }
}

export function isEffective(me: Enabler, now: DateTime.DateTime): boolean {
  return match(me, {
    ForDuration: timer => 
      CountdownTimer.isRunningUpdated(timer, now),

    ByPassword: enabler => 
      ByPasswordEnabler.isEffective(enabler),
  })
}

export const displayer = Displayer.implement<Enabler>(me => 
  match(me, {
    ForDuration: timer => 
      Displayer.asEnumDataVariantUsing(
        "Enabler", "ForDuration", CountdownTimer.displayer, timer
      ),

    ByPassword: enabler => 
      Displayer.asEnumDataVariantUsing(
        "Enabler", "ByPassword", ByPasswordEnabler.displayer, enabler
      )
  })
)

export const jsonSerializer = JsonSerializer.implement<Enabler>(me => 
  match(me, {
    ForDuration: timer =>
      JsonSerializer.asEnumDataVariant(
        "ForDuration", CountdownTimer.jsonSerializer, timer,
      ),

    ByPassword: enabler => 
      JsonSerializer.asEnumDataVariant(
        "ByPassword", ByPasswordEnabler.jsonSerializer, enabler,
      )
  })
)

export const jsonDeserializer = JsonDeserializer.implement<Enabler>(context =>
  JsonDeserializer.asEnum(context,
    JsonDeserializer.EnumDataVariant(
      "ForDuration", CountdownTimer.jsonDeserializer, ForDuration
    ), 
    JsonDeserializer.EnumDataVariant(
      "ByPassword", ByPasswordEnabler.jsonDeserializer, ByPassword,
    ),
  ),
)
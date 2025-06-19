import * as Duration from "../../ChronicTypes/Duration.ts"
import * as Password from "../../CommonTypes/Password.ts"
import * as Displayer from "../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { EnablerType } from "./Enabler.ts"
import { Unique } from "../../ElementaryTypes/Unique.ts";

export interface Cases<A, B> {
  readonly ForDuration: (duration: Duration.Duration) => A
  readonly ByPassword: (password: Password.Password) => B
}

export type EnablerCreator = Unique<"App.NetworkingAccess.EnablerCreator", {
  readonly type: EnablerType.ForDuration
  readonly duration: Duration.Duration
} | {
  readonly type: EnablerType.ByPassword
  readonly password: Password.Password
}>

export function ForDuration(duration: Duration.Duration): EnablerCreator {
  return Unique({
    type: EnablerType.ForDuration, 
    duration,
  })
}

export function ByPassword(password: Password.Password): EnablerCreator {
  return Unique({
    type: EnablerType.ByPassword, 
    password,
  })
}

export function match<A, B>(
  me: EnablerCreator,
  cases: Cases<A, B>,
):
  A | B
{
  switch (me.type) {
    case EnablerType.ForDuration: {
      return cases.ForDuration(me.duration)
    }
    case EnablerType.ByPassword: {
      return cases.ByPassword(me.password)
    }
  }
}

export const displayer = Displayer.implement<EnablerCreator>(me => 
  match(me, {
    ForDuration: duration => Displayer.asEnumDataVariantUsing(
      "Enabler", "ForDuration", Duration.displayer, duration
    ),

    ByPassword: password => Displayer.asEnumDataVariantUsing(
      "Enabler", "ByPassword", Password.displayer, password
    ),
  })
)

export const jsonSerializer = JsonSerializer.implement<EnablerCreator>(me => 
  match(me, {
    ForDuration: duration => JsonSerializer.asEnumDataVariant(
      "ForDuration", Duration.jsonSerializer, duration
    ),
    ByPassword: password => JsonSerializer.asEnumDataVariant(
      "ByPassword", Password.jsonSerializer, password,
    ),
  })
)

export const jsonDeserializer = JsonDeserializer.implement<EnablerCreator>(context => 
  JsonDeserializer.asEnum(context, 
    JsonDeserializer.EnumDataVariant(
      "ForDuration", Duration.jsonDeserializer, ForDuration,
    ),
    JsonDeserializer.EnumDataVariant(
      "ByPassword", Password.jsonDeserializer, ByPassword,
    ),
  )
)

// import { CountdownTimer, CountdownTimer.displayer, CountdownTimer.jsonDeserializer, CountdownTimer.jsonSerializer } from "@Pkg/CountdownTimer"
// import { DateTime } from "@Pkg/DateTime"
// import { Display } from "@Pkg/Display"
// import { Duration, Duration.displayer, Duration.jsonDeserializer, Duration.jsonSerializer } from "@Pkg/Duration"
// import { JsonDeserializer, JsonSerializer } from "@Pkg/JsonSerde"
// import { Option } from "@Pkg/Option"
// import { Password, Password.displayer, Password.jsonDeserializer, Password.jsonSerializer } from "@Pkg/Password"
// import { ByPasswordEnabler, ByPasswordEnablerDisplay, ByPasswordEnabler.jsonDeserializer, ByPasswordEnabler.jsonSerializer } from "@Pkg/ByPasswordEnabler"
// import { Uuid } from "@Pkg/Uuid"

// const enum ProtectorType {
//   ForDuration,
//   ByPassword,
// }

// export interface ProtectorCases<A, B> {
//   readonly ForDuration: (timer: CountdownTimer) => A
//   readonly ByPassword: (password: Password, isEffective: boolean) => B
// }

// export type Protector = {
//   readonly id: Uuid
//   readonly type: ProtectorType.ForDuration
//   readonly duration: CountdownTimer
// } | {
//   readonly id: Uuid
//   readonly type: ProtectorType.ByPassword
//   readonly password: Password
//   readonly isEffective: boolean
// }

// export const Protector = {
//   FOR_DURATION_AS_STRING: "ForDuration",
//   BY_PASSWORD_AS_STRING: "ByPassword",

//   ForDuration(id: Uuid, duration: CountdownTimer): Protector {
//     return {
//       id,
//       type: ProtectorType.ForDuration, 
//       duration: duration,
//     }
//   },

//   ByPassword(id: Uuid, password: Password, isEffective: boolean): Protector {
//     return {
//       id,
//       type: ProtectorType.ByPassword, 
//       password,
//       isEffective,
//     }
//   },
  
//   match<A, B>(
//     me: Protector,
//     cases: ProtectorCases<A, B>
//   ):
//     A | B
//   {
//     switch (me.type) {
//       case ProtectorType.ForDuration: {
//         return cases.ForDuration(me.duration)
//       }
//       case ProtectorType.ByPassword: {
//         return cases.ByPassword(me.password, me.isEffective)
//       }
//     }
//   },
  
//   isEffective(me: Protector, now: DateTime): boolean {
//     return Protector.match(me, {
//       ForDuration: duration =>
//         CountdownTimer.isRunning(duration, now),

//       ByPassword: (_, isEffective) => 
//         isEffective
//     })
//   }
// }

// export const ProtectorJsonSerializer = JsonSerializer.implement<Protector>((protector, serializer) => 
//   Protector.match(protector, {
//     ForDuration: timer => 
//       serializer.enum("ForDuration", timer, CountdownTimer.jsonSerializer),

//     ByPassword: (password, isEffective) => 
//       serializer.enumStruct("ByPassword", 
//         "password", Password.jsonSerializer, password,
//         "is_effective", serializer.Boolean, isEffective,
//       )
//   })
// )

// export const ProtectorJsonDeserializer = JsonDeserializer.implement<Protector>(context => 
//   Option.andThen(JsonDeserializer.asObjectContext(context), context => Option.orFn2(
//     () => deserializer.enum(object, "ForDuration", CountdownTimer.jsonDeserializer, Protector.ForDuration),
//     () => deserializer.enum(
//       object, 
//       "ByPassword", ByPasswordEnabler.jsonDeserializer, Protector.ByPassword),
//   ))
// )

// export const ProtectorDisplay = Display<Protector>((displayer, protector) => 
//   Protector.match(protector, {
//     ForDuration: timer => 
//       displayer.enum("ShadowVaultProtector", "ForDuration", CountdownTimer.displayer, timer),

//     ByPassword: status => 
//       displayer.enum("ShadowVaultProtector", "ByPassword", ByPasswordEnablerDisplay, status)
//   })
// )

// export interface ProtectorCreatorCases<A, B> {
//   readonly ForDuration: (duration: Duration) => A
//   readonly ByPassword: (password: Password) => B
// }

// export type ProtectorCreator = {
//   readonly type: ProtectorType.ForDuration
//   readonly duration: Duration
// } | {
//   readonly type: ProtectorType.ByPassword
//   readonly password: Password
// }

// export const ProtectorCreator = {
//   FOR_DURATION_AS_STRING: "ForDuration",
//   TILL_PASSWORD_AUTHENTICATION_AS_STRING: "ByPassword",

//   ForDuration(duration: Duration): ProtectorCreator {
//     return {
//       type: ProtectorType.ForDuration, 
//         duration,
//       }
//   },

//   ByPassword(password: Password): ProtectorCreator {
//     return {
//       type: ProtectorType.ByPassword, 
//         password,
//       }
//   },
  
//   match<A, B>(
//     me: ProtectorCreator,
//     cases: ProtectorCreatorCases<A, B>
//   ):
//     A | B
//   {
//     switch (me.type) {
//       case ProtectorType.ForDuration: {
//         return cases.ForDuration(me.duration)
//       }
//       case ProtectorType.ByPassword: {
//         return cases.ByPassword(me.password)
//       }
//     }
//   }
// }

// export const ProtectorCreatorJsonSerializer = JsonSerializer.implement<ProtectorCreator>((creator, serializer) => 
//   ProtectorCreator.match(creator, {
//     ForDuration: timer => 
//       serializer.enum("ForDuration", timer, Duration.jsonSerializer),

//     ByPassword: status => 
//       serializer.enum("ByPassword", status, Password.jsonSerializer)
//   })
// )

// export const ProtectorCreatorJsonDeserializer = JsonDeserializer.implement<ProtectorCreator>(context => 
//   Option.andThen(JsonDeserializer.asObjectContext(context), context => Option.orFn2(
//     () => deserializer.enum(object, "ForDuration", Duration.jsonDeserializer, ProtectorCreator.ForDuration),
//     () => deserializer.enum(object, "ByPassword", Password.jsonDeserializer, ProtectorCreator.ByPassword),
//   ))
// )

// export const ProtectorCreatorDisplay = Display<ProtectorCreator>((displayer, creator) => 
//   ProtectorCreator.match(creator, {
//     ForDuration: duration => 
//       displayer.enum("ShadowVaultProtectorCreator", "ForDuration", Duration.displayer, duration),

//     ByPassword: password => 
//       displayer.enum("ShadowVaultProtectorCreator", "ByPassword", Password.displayer, password)
//   })
// )

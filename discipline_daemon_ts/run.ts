import * as Client from "./Client.ts";
import * as Uuid from "./ElementaryTypes/Uuid.ts";
import { GetData } from "./App/mod.ts";
import * as ShadowVaults from "./Features/ShadowVaults/mod.ts";
import * as NetworkingAccess from "./Features/NetworkingAccess/mod.ts";
import * as UserAccess from "./Features/UserAccess/mod.ts";
import * as OperatingSystemUsername from "./CommonTypes/OperatingSystemUsername.ts";
import { None, Option } from "./ElementaryTypes/Option.ts";
import * as Duration from "./ChronicTypes/Duration.ts";
import * as TimeRange from "./ChronicTypes/TimeRange.ts";
import * as Time from "./ChronicTypes/Time.ts";
import * as Hour from "./ChronicTypes/Hour.ts";

const client = Option.unwrap(Client.create("127.0.0.1", 8080))

const lunyUsername = OperatingSystemUsername.createOrThrow("luny")
const lunynytroUsername = OperatingSystemUsername.createOrThrow("lunynytro")

// console.log(NetworkingAccess.CreateRuleEnforcer.displayer.display(
//   await NetworkingAccess.CreateRuleEnforcer.execute(
//     client,
//     lunynytroUsername,
//   )
// ))

// console.log(NetworkingAccess.CreateRule.displayer.display(
//   await NetworkingAccess.CreateRule.execute(
//     client,
//     lunynytroUsername,
//     NetworkingAccess.RuleCreator.create(
//       None(),
      
//       NetworkingAccess
//         .EnablerCreator
//         .ForDuration(Duration.fromDaysOrThrow(10)),

//       NetworkingAccess
//         .ActivatorCreator.InTimeRange(TimeRange.create(
//           Time.fromHour(Option.unwrap(Hour.fromNumberPM(7))),
//           Time.fromHour(Option.unwrap(Hour.fromNumberAM(7))),
//         ))
//     )
//   )
// ))

console.log(ShadowVaults.CreateShadowVault.displayer.display(
  await ShadowVaults.CreateShadowVault.execute(
    client,
    ShadowVaults.ShadowVaultCreator.create(
      None(),
      ShadowVaults.Name.createOrThrow("Phone Password"),
      ShadowVaults.Datum.createOrThrow("eroifdsa78jhffds"),
      Duration.fromDaysOrThrow(30),
    )
  )
))

console.log(GetData.displayer.display(
  await GetData.execute(client)
))

console.log()
// import * as NetworkingAccessUI from "./Features/NetworkingAccess/UI/mod.ts"
// import { Tried } from "@Pkg/Tried";
// import * as DateTime from "@Pkg/DateTime"


// Tried.map(
//   await GetData.execute(client), 
//   data => 
//     console.log(
//       NetworkingAccessUI
//         .display(data.networkingAccess, DateTime.now())
//     )
// )
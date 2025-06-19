import { Unique } from "@Pkg/Unique";
import { Uuid } from "@Pkg/Uuid";
import { OperatingSystemUsername } from "@Pkg/OperatingSystemUsername";
import { Option } from "@Pkg/Option";
import { OperatingSystemPassword } from "@Pkg/OperatingSystemPassword";
import { CountdownTimer } from "@Pkg/CountdownTimer";
import { TimeRange } from "@Pkg/TimeRange";
import { WeekdayRange } from "@Pkg/WeekdayRange";
import { WeeklyTimeRange } from "@Pkg/WeeklyTimeRange";

export type App = Unique<"App", {
  userAccounts: UserAccount[]
}>

export type UserAccountName = Unique<"UserAccount.Name", string>

export type UserAccount = Unique<"UserAccount", {
  id: Uuid
  name: string
  operatingSystemUsername: OperatingSystemUsername
  operatingSystemPassword: Option<OperatingSystemPassword>
}>
import { AppData, CountdownTimer, DateTime, Duration, Executer, Hour, Minute, NetworkingAccess, OperatingSystemPassword, OperatingSystemUsername, Option, Second, ShadowVaults, Some, Time, TimeRange, UserAccess, Uuid, Weekday, WeekdayRange } from "Discipline";

const now = DateTime.now()
const twoMintes = Duration.TwoMinutes()

const userAccessRules = [
  UserAccess.Rule.new(
    Uuid.generate(),
    UserAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    UserAccess.Activator.AtHour(
      Option.unwrap(Hour.fromNumberAM(1))
    ),
  ),
  UserAccess.Rule.new(
    Uuid.generate(),
    UserAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    UserAccess.Activator.AtWeekday(
      Weekday.Sunday()
    ),
  ),
  UserAccess.Rule.new(
    Uuid.generate(),
    UserAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    UserAccess.Activator.ForDuration(CountdownTimer.new(
      Option.unwrap(Duration.fromWeeks(3)),
      DateTime.now(),
    )),
  ),
  UserAccess.Rule.new(
    Uuid.generate(),
    UserAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    UserAccess.Activator.InTimeRange(
      TimeRange.new(
        Time.fromHourMinuteSecond(
          Option.unwrap(Hour.fromNumberAM(3)),
          Option.unwrap(Minute.fromNumber(8)),
          Option.unwrap(Second.fromNumber(34)),
        ),
        Time.fromHourMinuteSecond(
          Option.unwrap(Hour.fromNumberPM(3)),
          Option.unwrap(Minute.fromNumber(8)),
          Option.unwrap(Second.fromNumber(23)),
        ),
      )
    ),
  ),
  UserAccess.Rule.new(
    Uuid.generate(),
    UserAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    UserAccess.Activator.InWeekdayRange(
      WeekdayRange.new(Weekday.Sunday(), Weekday.Thursday()),
    ),
  ),
]

const userAccessRuleEnforcers = [
  UserAccess.RuleEnforcer.constructor(
    userAccessRules,
    OperatingSystemUsername.newOrThrow("luny"),
    OperatingSystemPassword.newOrThrow("i eat vending machines"),
    false,
    false,
  ),
  UserAccess.RuleEnforcer.constructor(
    userAccessRules,
    OperatingSystemUsername.newOrThrow("lunynytro"),
    OperatingSystemPassword.newOrThrow("luny is an automata"),
    false,
    false,
  ),
  UserAccess.RuleEnforcer.constructor(
    userAccessRules,
    OperatingSystemUsername.newOrThrow("automata"),
    OperatingSystemPassword.newOrThrow("lunar orbit"),
    false,
    false,
  ),
  UserAccess.RuleEnforcer.constructor(
    userAccessRules,
    OperatingSystemUsername.newOrThrow("urban_city"),
    OperatingSystemPassword.newOrThrow("gigantuc slow ritating fans"),
    false,
    false,
  ),
  UserAccess.RuleEnforcer.constructor(
    userAccessRules,
    OperatingSystemUsername.newOrThrow("venti"),
    OperatingSystemPassword.newOrThrow("venti is a vending machine"),
    false,
    false,
  ),
  UserAccess.RuleEnforcer.constructor(
    userAccessRules,
    OperatingSystemUsername.newOrThrow("gaming"),
    OperatingSystemPassword.newOrThrow("world domination"),
    false,
    false,
  ),
  UserAccess.RuleEnforcer.constructor(
    userAccessRules,
    OperatingSystemUsername.newOrThrow("fun"),
    OperatingSystemPassword.newOrThrow("luny is the funnest life form in the lunar lands"),
    false,
    false,
  ),
]

const userAccessFeature = UserAccess.Feature.new(
  userAccessRuleEnforcers,
  Duration.OneMinute(),
)

const networkingAccessRules = [
  NetworkingAccess.Rule.new(
    Uuid.generate(),
    NetworkingAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    NetworkingAccess.Activator.AtHour(
      Option.unwrap(Hour.fromNumberAM(1))
    ),
  ),
  NetworkingAccess.Rule.new(
    Uuid.generate(),
    NetworkingAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    NetworkingAccess.Activator.AtWeekday(
      Weekday.Sunday()
    ),
  ),
  NetworkingAccess.Rule.new(
    Uuid.generate(),
    NetworkingAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    NetworkingAccess.Activator.ForDuration(CountdownTimer.new(
      Option.unwrap(Duration.fromWeeks(3)),
      DateTime.now(),
    )),
  ),
  NetworkingAccess.Rule.new(
    Uuid.generate(),
    NetworkingAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    NetworkingAccess.Activator.InTimeRange(
      TimeRange.new(
        Time.fromHourMinuteSecond(
          Option.unwrap(Hour.fromNumberAM(3)),
          Option.unwrap(Minute.fromNumber(8)),
          Option.unwrap(Second.fromNumber(44)),
        ),
        Time.fromHourMinuteSecond(
          Option.unwrap(Hour.fromNumberPM(3)),
          Option.unwrap(Minute.fromNumber(8)),
          Option.unwrap(Second.fromNumber(44)),
        ),
      )
    ),
  ),
  NetworkingAccess.Rule.new(
    Uuid.generate(),
    NetworkingAccess.Enabler.ForDuration(CountdownTimer.new(
      Duration.fromDaysOrThrow(3),
      DateTime.now(),
    )),
    NetworkingAccess.Activator.InWeekdayRange(
      WeekdayRange.new(Weekday.Sunday(), Weekday.Thursday()),
    ),
  ),
]

const networkingAccessRuleEnforcers = [
  NetworkingAccess.RuleEnforcer.constructor(
    networkingAccessRules,
    OperatingSystemUsername.newOrThrow("luny"),
    false,
    false,
  ),
  NetworkingAccess.RuleEnforcer.constructor(
    networkingAccessRules,
    OperatingSystemUsername.newOrThrow("lunynytro"),
    false,
    false,
  ),
  NetworkingAccess.RuleEnforcer.constructor(
    networkingAccessRules,
    OperatingSystemUsername.newOrThrow("automata"),
    false,
    false,
  ),
  NetworkingAccess.RuleEnforcer.constructor(
    networkingAccessRules,
    OperatingSystemUsername.newOrThrow("urban_city"),
    false,
    false,
  ),
  NetworkingAccess.RuleEnforcer.constructor(
    networkingAccessRules,
    OperatingSystemUsername.newOrThrow("venti"),
    false,
    false,
  ),
  NetworkingAccess.RuleEnforcer.constructor(
    networkingAccessRules,
    OperatingSystemUsername.newOrThrow("gaming"),
    false,
    false,
  ),
  NetworkingAccess.RuleEnforcer.constructor(
    networkingAccessRules,
    OperatingSystemUsername.newOrThrow("fun"),
    false,
    false,
  ),
]

const networkingAccessFeature = NetworkingAccess.Feature.new(
  networkingAccessRuleEnforcers,
  Duration.OneMinute()
)

const shadowVaultsFeature = ShadowVaults.Feature.new([
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("Luny Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro FamilyLink"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("Luny Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro FamilyLink"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("Luny Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro FamilyLink"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("Luny Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro FamilyLink"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("Luny Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro Password"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
  ShadowVaults.ShadowVault.new(
    Uuid.generate(),
    ShadowVaults.Name.newOrThrow("LunyNytro FamilyLink"),
    Some(ShadowVaults.Datum.newOrThrow("Dis is a monolithic chocolate cookie")),
    CountdownTimer.new(
      Duration.fromDaysOrThrow(7),
      DateTime.now()
    ),
  ),
])

export const data = AppData.constructor(
  DateTime.now(),
  userAccessFeature,
  shadowVaultsFeature,
  networkingAccessFeature,
)

export const discipline = Executer.new("http://localhost:8080")
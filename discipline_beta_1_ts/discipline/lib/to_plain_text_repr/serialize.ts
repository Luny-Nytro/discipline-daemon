import { 
  Discipline,
  AndStatusIndicator,
  DateTime,
  DeviceAccessRegulator,
  Duration,
  DurationStatusIndicator,
  Hour,
  HourRangeStatusIndicator,
  Minute,
  MinuteRangeStatusIndicator,
  NetworkAccessRegulator,
  OrStatusIndicator,
  StatusIndicator,
  StatusIndicatorKind,
  Time,
  TimeRangeStatusIndicator,
  UserAccessRegulator,
  Weekday,
  WeekdayRangeStatusIndicator,
} from "Pkg"

type Value = (
  | String
  | Boolean
  | Array
  | Map
)

interface String {
  readonly kind: "string"
  readonly value: string
}

function string(value: string): String {
  return {
    kind: "string",
    value,
  }
}

interface Boolean {
  readonly kind: "boolean"
  readonly value: boolean
}

function boolean(value: boolean): Boolean {
  return {
    kind: "boolean",
    value,
  }
}

interface Field {
  readonly kind: "field"
  readonly key: string
  readonly value: Value
}

function field(key: string, value: Value): Field {
  return {
    kind: "field",
    key,
    value,
  }
}

interface Array {
  readonly kind: "array"
  readonly name: string
  readonly items: Value[]
}

function array(name: string, items: Value[]): Array {
  return {
    kind: "array",
    items,
    name,
  }
}

interface Map {
  readonly kind: "map"
  readonly name: string
  readonly fields: Field[]
}

function map(name: string, ...fields: Field[]): Map {
  return {
    kind: "map",
    fields,
    name,
  }
}

export function serialize(value: Value, depth: number = 0) {
  switch (value.kind) {
    case "string": {
      // return `"${value.value.replaceAll("\"", "\\\"")}"`
      return value.value
    }
    case "boolean": {
      return value.value ? "yes" : "no"
    }
    case "array": {
      let str = `${value.name} [\n`
      for (const item of value.items) {
        str += `${"  ".repeat(depth + 1)}${serialize(item, depth)}\n`
      }
      str += `${"  ".repeat(depth)}]`
      return str
    }
    case "map": {
      let str = `${value.name} {\n`
      for (const field of value.fields) {
        str += `${"  ".repeat(depth + 1)}${field.key}: ${serialize(field.value, depth + 1)}\n`
      }
      str += `${"  ".repeat(depth)}}`
      return str
    }
  }
}

function hour(hour: Hour) {
  return hour.value1_24() <= 12
    ? `${hour.value1_24()}am`
    : `${hour.value1_24() - 12}pm`
}
function minute(minute: Minute) {
  return minute.value1().toString()
}
function weekday(weekday: Weekday) {
  return weekday.name()
}
function time(time: Time) {
  let am: string
  let hour: number

  if (time.hour.value1_24() <= 12) {
    am = "am"
    hour = time.hour.value1_24()
  } else {
    am = "pm"
    hour = time.hour.value1_24() - 12
  }
  
  return `${hour}:${minute(time.minute)} ${am}`
}

export function durationStatusIndicator(value: DurationStatusIndicator) {
  const parts: string[] = []
  let remainingMs = value.duration.milliseconds();

  if (remainingMs >= Duration.MILLISECONDS_PER_DAY) {
    const days = Math.floor(remainingMs / Duration.MILLISECONDS_PER_DAY)
    parts.push(`${days}d`)
    remainingMs -= days * Duration.MILLISECONDS_PER_DAY
  }
  if (remainingMs >= Duration.MILLISECONDS_PER_HOUR) {
    const hours = Math.floor(remainingMs / Duration.MILLISECONDS_PER_HOUR)
    parts.push(`${hours}h`)
    remainingMs -= hours * Duration.MILLISECONDS_PER_HOUR
  }
  if (remainingMs >= Duration.MILLISECONDS_PER_MINUTE) {
    const minutes = Math.floor(remainingMs / Duration.MILLISECONDS_PER_MINUTE)
    parts.push(`${minutes}m`)
    remainingMs -= minutes * Duration.MILLISECONDS_PER_MINUTE
  }
  if (remainingMs >= Duration.MILLISECONDS_PER_SECOND) {
    const seconds = Math.floor(remainingMs / Duration.MILLISECONDS_PER_SECOND)
    parts.push(`${seconds}s`)
    remainingMs -= seconds * Duration.MILLISECONDS_PER_SECOND
  }

  if (parts.length > 0) {
    return string(`for ${parts.join(" ")}`)
  } else {
    return string("inactive")
  }
}

export function minuteRangeStatusIndicator(value: MinuteRangeStatusIndicator) {
  return string(`within minutes ${value.from.value1()} ... ${value.till.value1()}`)
}

export function hourRangeStatusIndicator(value: HourRangeStatusIndicator) {
  return string(`within hours ${hour(value.from)} ... ${hour(value.till)}`)
}

export function weekdayRangeStatusIndicator(value: WeekdayRangeStatusIndicator) {
  return string(`within weekdays ${value.from.name()} ... ${value.till.name()}`)
}

export function timeRangeStatusIndicator(value: TimeRangeStatusIndicator) {
  return string(`within ${time(value.from)} ... ${time(value.till)}`)
}

export function orStatusIndicator(value: OrStatusIndicator): Array {
  return array("either of", value.items.map(statusIndicator))
}

export function andStatusIndicator(value: AndStatusIndicator): Array {
  return array("all of", value.items.map(statusIndicator))
}

export function statusIndicator(value: StatusIndicator): String | Array {
  switch (value.kind) {
    case StatusIndicatorKind.Or: return orStatusIndicator(value)
    case StatusIndicatorKind.And: return andStatusIndicator(value)
    case StatusIndicatorKind.Duration: return durationStatusIndicator(value)
    case StatusIndicatorKind.TimeRange: return timeRangeStatusIndicator(value)
    case StatusIndicatorKind.HourRange: return hourRangeStatusIndicator(value)
    case StatusIndicatorKind.MinuteRange: return minuteRangeStatusIndicator(value)
    case StatusIndicatorKind.WeekdayRange: return weekdayRangeStatusIndicator(value)
  }
}

export function userAccessRegulator(value: UserAccessRegulator) {
  return map(
    "UserAccessRegulator", 
    field("username", string(value.username)),
    field("password", string(value.password)),
    field("is blocked", boolean(value.isBlocked)),
    field("should block", boolean(value.blockIndicator.isActive())),
    field("block indicator", statusIndicator(value.blockIndicator)),
  )
}

export function deviceAccessRegulator(value: DeviceAccessRegulator) {
  return map(
    "DeviceAccessRegulator",
    field("should block", boolean(value.blockIndicator.isActive())),
    field("block indicator", statusIndicator(value.blockIndicator)),
  )
}

export function networkAccessRegulator(value: NetworkAccessRegulator) {
  return map(
    "NetworkAccessRegulator",
    field("is blocked", boolean(!value.isAllowed)),
    field("should block", boolean(value.blockIndicator.isActive())),
    field("block indicator", statusIndicator(value.blockIndicator)),
  )
}

export function discipline(value: Discipline) {
  const now = DateTime.now()

  return map(
    "Discipline",
    field("time", string(`${weekday(now.weekday())}, ${time(now.time())}`)),
    field("user access regulator", userAccessRegulator(value.userAccessRegulator)),
    field("device access regulator", deviceAccessRegulator(value.deviceAccessRegulrator)),
    field("network access regulator", networkAccessRegulator(value.networkAccessRegulrator)),
  )
}
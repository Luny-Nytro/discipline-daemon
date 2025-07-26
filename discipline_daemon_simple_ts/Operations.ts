import { App, Array, CountdownTimer, DataHider, Duration, isNone, None, Option, Some } from "./Prelude.ts";

const fiveMinutes = Option.unwrap(Duration.fromMinutes(5))

export const enum CreateDataHiderError {
  DataHiderNameIsLongerThan40Characters,
  DataHiderIsDataLongerThan40Characters,
  DataHiderCreationLimitReached,
  DataHiderNameIsAlreadyInUse,
  GuardForIsLongerThanOneWeek,
}

export function createDataHider(
  app: App, 
  name: string, 
  data: string,
  guardFor: Duration,
): Option<CreateDataHiderError> {
  if (name.length > 40) {
    return Some(CreateDataHiderError.DataHiderNameIsLongerThan40Characters)
  }
  if (data.length > 40) {
    return Some(CreateDataHiderError.DataHiderIsDataLongerThan40Characters)
  }
  if (Duration.weeks(guardFor) > 1) {
    return Some(CreateDataHiderError.GuardForIsLongerThanOneWeek)
  }
  if (app.dataHiders.length >= 20) {
    return Some(CreateDataHiderError.DataHiderCreationLimitReached)
  }
  if (app.dataHiders.some(dataHider => dataHider.name === name)) {
    return Some(CreateDataHiderError.DataHiderNameIsAlreadyInUse)
  }

  app.dataHiders.push(DataHider.constructor(
    name,
    data,
    CountdownTimer.new(guardFor),
  ))

  return None()
}

export const enum DeleteDataHiderError {
  NoSuchDataHider
}

export function deleteDataHider(
  app: App, 
  dataHiderName: string,
): Option<DeleteDataHiderError> {
  const index = Array.findIndex(app.dataHiders, dataHider => dataHider.name === dataHiderName)
  if (isNone(index)) {
    return Some(DeleteDataHiderError.NoSuchDataHider)
  } 
  
  app.dataHiders.splice(index.value, 1)
  return None()
}

export const enum IncreaseDataHiderCountdownTimerError {
  NoSuchDataHider,
  DataWouldBeGuardedForMoreThanOneWeek,
}

export function increaseDataHiderCountdownTimer(
  app: App,
  dataHiderName: string,
): Option<IncreaseDataHiderCountdownTimerError> {
  const dataHider = app.dataHiders.find(dataHider => dataHider.name === dataHiderName)
  if (dataHider === undefined) {
    return Some(IncreaseDataHiderCountdownTimerError.NoSuchDataHider)
  }

  const newValue = Duration.add(dataHider.countdownTimer.remainingDuration, fiveMinutes)
  if (isNone(newValue)) {
    return Some(IncreaseDataHiderCountdownTimerError.DataWouldBeGuardedForMoreThanOneWeek)
  }
  if (Duration.weeks(newValue.value) > 1) {
    return Some(IncreaseDataHiderCountdownTimerError.DataWouldBeGuardedForMoreThanOneWeek)
  }

  dataHider.countdownTimer.remainingDuration = newValue.value
  return None()
}

export function decreaseMainUserDailyAllowance(app: App) {
  const dailyAllowance = app.mainUserAccessRegulator.dailyAllowance
  dailyAllowance.remainingAllowance = Duration.subOrZero(dailyAllowance.remainingAllowance, fiveMinutes)
}

export const enum IncreaseSuperUserCountdownTimerError {
  SuperUserWouldBeLockedForMoreThanOneWeek,
}

export function increaseSuperUserCountdownTimer(app: App): Option<IncreaseSuperUserCountdownTimerError> {
  const countdownTimer = app.superUserAccessRegulator.countdownTimer

  const newValue = Duration.add(countdownTimer.remainingDuration, fiveMinutes)
  if (isNone(newValue)) {
    return Some(IncreaseSuperUserCountdownTimerError.SuperUserWouldBeLockedForMoreThanOneWeek)
  }
  if (Duration.weeks(newValue.value) > 1) {
    return Some(IncreaseSuperUserCountdownTimerError.SuperUserWouldBeLockedForMoreThanOneWeek)
  }

  countdownTimer.remainingDuration = newValue.value
  return None()
}
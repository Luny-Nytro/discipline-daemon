import { DateTime, Duration, isSome, Option, SingleValueStorage, Serde, isErr, Ok, Tried, SingleValueStorageError, Async, Hour, Minute, Server, Time, TimeRange, changeUserPassword, isOk, logoutUser } from "./Prelude.ts";

const enum ConditionType {
  CountdownTimer,
  DailyAllowance,
}

export interface CountdownTimer {
  readonly type: ConditionType.CountdownTimer
  duration: Duration
  remainingDuration: Duration
  previousSynchronizationTime: DateTime
}

export const CountdownTimer = {
  constructor(
    duration: Duration, 
    remainingDuration: Duration,
    previousSynchronizationTime: DateTime,
  ): CountdownTimer {
    return {
      type: ConditionType.CountdownTimer,
      duration,
      remainingDuration,
      previousSynchronizationTime,
    }
  },

  new(duration: Duration): CountdownTimer {
    return {
      type: ConditionType.CountdownTimer,
      duration,
      remainingDuration: Duration.clone(duration),
      previousSynchronizationTime: DateTime.now()
    }
  },

  synchronize(me: CountdownTimer, now: DateTime) {
    me.remainingDuration = Duration.subOrZero(me.remainingDuration, DateTime.tillOrZero(me.previousSynchronizationTime, now))
    me.previousSynchronizationTime = now
  },

  // increaseRemainingDurationBy(me: CountdownTimer, duration: Duration) {
  //   const newValue = Duration.add(me.remainingDuration, duration)
  //   if (isSome(newValue)) {
  //     me.remainingDuration = newValue.value
  //     return Some(null)
  //   } else {
  //     return None()
  //   }
  // },

  isDone(me: CountdownTimer, now: DateTime): boolean {
    CountdownTimer.synchronize(me, now)
    return Duration.isZero(me.remainingDuration)
  },
}

export interface DataHider {
  name: string,
  data: string
  countdownTimer: CountdownTimer
}

export const DataHider = {
  constructor(name: string, data: string, countdownTimer: CountdownTimer): DataHider {
    return {
      name,
      data,
      countdownTimer,
    }
  },

  synchronize(me: DataHider, now: DateTime) {
    CountdownTimer.synchronize(me.countdownTimer, now)
  },
}

export interface DailyAllowance {
  readonly type: ConditionType.DailyAllowance
  allowance: Duration,
  remainingAllowance: Duration,
  previousSynchronizationTime: DateTime,
}

export const DailyAllowance = {
  new(allowance: Duration): DailyAllowance {
    return {
      type: ConditionType.DailyAllowance,
      allowance,
      remainingAllowance: Duration.clone(allowance),
      previousSynchronizationTime: DateTime.now(),
    }
  },

  constructor(
    allowance: Duration, 
    remainingAllowance: Duration, 
    previousSynchronizationTime: DateTime,
  ): DailyAllowance {
    return {
      type: ConditionType.DailyAllowance,
      allowance,
      remainingAllowance,
      previousSynchronizationTime,
    }
  },

  synchronize(me: DailyAllowance, now: DateTime) {
    // console.log("DateTime.timeline2(DateTime.midnight(me.previousSynchronizationTime), DateTime.midnight(now)): ", DateTime.timeline2(DateTime.midnight(me.previousSynchronizationTime), DateTime.midnight(now)))
    // console.log(
    //   "DateTime.midnight(me.previousSynchronizationTime), DateTime.midnight(now): ", 
    //   DateTime.midnight(me.previousSynchronizationTime), 
    //   DateTime.midnight(now),
    // )
    // console.log(
    //   "me.previousSynchronizationTime, now: ", 
    //   DateTime.midnight(me.previousSynchronizationTime), 
    //   DateTime.midnight(now),
    // )
    // console.log("DailyAllowance")
    // console.log("Now:", DateTime.toString(now))
    // console.log("PreviousSynchronizationTime:", DateTime.toString(me.previousSynchronizationTime))
    // console.log("Now.Midnight:", DateTime.toString(DateTime.midnight(now)))
    // console.log("PreviousSynchronizationTime.Midnight:", DateTime.toString(DateTime.midnight(me.previousSynchronizationTime)))
    
    if (DateTime.isBefore(DateTime.midnight(me.previousSynchronizationTime), DateTime.midnight(now))) {
      me.remainingAllowance = Duration.clone(me.allowance)
      me.previousSynchronizationTime = DateTime.clone(now)
    } else {
      me.remainingAllowance = Duration.subOrZero(
        me.remainingAllowance, 
        Duration.min(
          DateTime.tillOrZero(me.previousSynchronizationTime, now),
          Duration.newOneMinute(),
        ),
      )
      
      me.previousSynchronizationTime = DateTime.clone(now)
    }
  },

  isDone(me: DailyAllowance, now: DateTime): boolean {
    DailyAllowance.synchronize(me, now)
    return Duration.isZero(me.remainingAllowance)
  },
}


// interface TimeAndDateTime {
//   datetime: DateTime,
//   time: Time,
// }

// export type Condition = {
//   readonly type: ConditionType.TimeRange
//   readonly condition: TimeRange
// } | {
//   readonly type: ConditionType.DailyAllowance
//   readonly condition: DailyAllowance
// }

// export const Condition = {
//   evaluate(me: Condition, context: TimeAndDateTime): boolean {
//     switch (me.type) {
//       case ConditionType.TimeRange: {
//         return TimeRange.contains(me.condition, context.time)
//       }
//       case ConditionType.DailyAllowance: {
//         return DailyAllowance.isDone(me.condition, context.datetime)
//       }
//     }
//   }
// }


export interface SuperUserAccessRegulator {
  username: string
  isLocked: boolean
  publicPassword: string
  privatePassword: string
  countdownTimer: CountdownTimer
}

export const SuperUserAccessRegulator = {
  new(
    username: string,
    isLocked: boolean,
    publicPassword: string,
    privatePassword: string,
    countdownTimer: CountdownTimer,
  ): SuperUserAccessRegulator {
    return {
      username,
      isLocked,
      publicPassword,
      privatePassword,
      countdownTimer,
    }
  },

  synchronize(me: SuperUserAccessRegulator, now: DateTime) {
    CountdownTimer.synchronize(me.countdownTimer, now)
  },

  async onOperatingSystemBootUp(me: SuperUserAccessRegulator) {
    if (me.isLocked) {
      return
    }

    const tried = await changeUserPassword(me.username, me.privatePassword)
    if (isOk(tried)) {
      me.isLocked = true
    }
  },

  async apply(me: SuperUserAccessRegulator, now: DateTime) {
    // when the coundown timer reaches zero, we should unlock the user.
    if (CountdownTimer.isDone(me.countdownTimer, now)) {
      // if the user is already unlocked, do nothing.
      if (!me.isLocked) {
        return
      }

      // unlock user.
      const tried = await changeUserPassword(me.username, me.publicPassword)
      if (isOk(tried)) {
        me.isLocked = false
      }
    } 
    // otherwise, ensure the user is locked.
    else {
      // if it's already locked, do nothing.
      if (me.isLocked) {
        return
      }
    
      // change its password so they cannot login until we decide otherwise.
      const tried = await changeUserPassword(me.username, me.privatePassword)
      if (isOk(tried)) {
        me.isLocked = true
      }

      // logout the user
      await logoutUser(me.username)
    }
  }
}

export interface MainUserAccessRegulator {
  username: string
  isLocked: boolean
  publicPassword: string
  privatePassword: string
  dailyAllowance: DailyAllowance

  nightTime: TimeRange
  dhuherTime: TimeRange
  asrTime: TimeRange
  maghribTime: TimeRange
}

export const MainUserAccessRegulator = {
  new(
    username: string,
    isLocked: boolean,
    publicPassword: string,
    privatePassword: string,
    dailyAllowance: DailyAllowance,
  ): MainUserAccessRegulator {
    const nightTime = TimeRange.new(
      Time.fromH(Option.unwrap(Hour.newPM(7))), 
      Time.fromH(Option.unwrap(Hour.newAM(7))),
    )

    const dhuherTime = TimeRange.new(
      Time.newHM(Option.unwrap(Hour.newAM(11)), Option.unwrap(Minute.new(30))),
      Time.newHM(Option.unwrap(Hour.newPM(0)), Option.unwrap(Minute.new(30))),
    )

    const asrTime = TimeRange.new(
      Time.newHM(Option.unwrap(Hour.newPM(2)), Option.unwrap(Minute.new(30))),
      Time.newHM(Option.unwrap(Hour.newPM(3)), Option.unwrap(Minute.new(30))),
    )

    const maghribTime = TimeRange.new(
      Time.fromH(Option.unwrap(Hour.newPM(7))),
      Time.fromH(Option.unwrap(Hour.newAM(7))),
    )

    return {
      username,
      isLocked,
      publicPassword,
      privatePassword,
      dailyAllowance,
      nightTime,
      dhuherTime,
      asrTime,
      maghribTime,
    }
  },

  synchronize(me: MainUserAccessRegulator, now: DateTime) {
    DailyAllowance.synchronize(me.dailyAllowance, now)
  },

  shouldBlockUser(me: MainUserAccessRegulator, now: DateTime) {
    const time = DateTime.time(now)

    return DailyAllowance.isDone(me.dailyAllowance, now)
      || TimeRange.contains(me.asrTime, time)
      || TimeRange.contains(me.nightTime, time)
      || TimeRange.contains(me.dhuherTime, time)
      || TimeRange.contains(me.maghribTime, time)
  },

  async onOperatingSystemBootUp(me: MainUserAccessRegulator) {
    if (me.isLocked) {
      return
    }

    const tried = await changeUserPassword(me.username, me.privatePassword)
    if (isOk(tried)) {
      me.isLocked = true
    }
  },

  async apply(me: MainUserAccessRegulator, now: DateTime) {
    // ensure the user is locked.
    if (MainUserAccessRegulator.shouldBlockUser(me, now)) {
      // if it's already locked, do nothing.
      if (me.isLocked) {
        return
      }
    
      // change its password so it cannot login until we decide otherwise.
      const tried = await changeUserPassword(me.username, me.privatePassword)
      if (isOk(tried)) {
        me.isLocked = true
      } 
      
      // logout the user
      await logoutUser(me.username)     
    } 
    // ensure the user is unlocked.
    else {
      // if the user is already unlocked, do nothing.
      if (!me.isLocked) {
        return
      }

      // unlock user.
      const tried = await changeUserPassword(me.username, me.publicPassword)
      if (isOk(tried)) {
        // The user can now login with `me.publicPassword`
        me.isLocked = false
      }
    }
  }
}

export interface App {
  dataHiders: DataHider[]
  mainUserAccessRegulator: MainUserAccessRegulator
  superUserAccessRegulator: SuperUserAccessRegulator
  // screenTimeLimiter: ScreenTimeLimiter
}

export const App = {
  constructor(
    dataHiders: DataHider[], 
    mainUserAccessRegulator: MainUserAccessRegulator,
    superUserAccessRegulator: SuperUserAccessRegulator,
    // screenTimeLimiter: ScreenTimeLimiter,
  ): App {
    return {
      dataHiders,
      // screenTimeLimiter,
      mainUserAccessRegulator,
      superUserAccessRegulator,
    }
  },

  // createInitialInstance(): App {
  //   return {
  //     dataHiders: [],
  //     screenTimeLimiter: {
  //       customCondition: None(),
  //       dailyAllowance: DailyAllowance.new(Option.unwrap(Duration.fromHours(3)))
  //     }
  //   }
  // },

  synchronize(me: App, now: DateTime) {
    me.dataHiders.forEach(dataHider => DataHider.synchronize(dataHider, now))
    // ScreenTimeLimiter.synchronize(me.screenTimeLimiter, now)
    MainUserAccessRegulator.synchronize(me.mainUserAccessRegulator, now)
    SuperUserAccessRegulator.synchronize(me.superUserAccessRegulator, now)
  },

  async apply(me: App, now: DateTime) {
    await MainUserAccessRegulator.apply(me.mainUserAccessRegulator, now)
    await SuperUserAccessRegulator.apply(me.superUserAccessRegulator, now)
  },

  async onOperatinSystemBootUp(me: App) {
    await MainUserAccessRegulator.onOperatingSystemBootUp(me.mainUserAccessRegulator)
    await SuperUserAccessRegulator.onOperatingSystemBootUp(me.superUserAccessRegulator)
  },
}

export interface AppLoader {
  readonly app: App
  readonly storage: SingleValueStorage<App, Serde.Error, Serde.Error>
}

export const AppLoader = {
  async load(
    storageFilePath: string,
    createInitialInstance: () => App,
  ): Promise<Tried<
      AppLoader, 
      SingleValueStorageError<App, Serde.Error, Serde.Error>
  >> {
    const storage = SingleValueStorage.new(
      storageFilePath,
      createInitialInstance,
      Serde.App.serialize,
      Serde.App.deserialize,
    )

    const maybeApp = await storage.load()
    if (isErr(maybeApp)) {
      return maybeApp
    }

    const app = maybeApp.value
    const appLoader: AppLoader = { app, storage }

    // `AppLoader.load` should be called on operatin system boot up, 
    // so this is the proper place to call `App.onOperatinSystemBootUp`.
    await App.onOperatinSystemBootUp(app)

    Async.interval(Duration.newOneMinute(), async () => {
      const now = DateTime.now()

      App.synchronize(app, now)
      await App.apply(app, now)
      await AppLoader.save(appLoader)
    })
  
    Server.Server(app)

    return Ok(appLoader)
  },

  async save(me: AppLoader) {
    const maybeError = await me.storage.update(me.app)
    if (isSome(maybeError)) {
      console.error("AppLoader.Save: ", maybeError.value)
    }
  },
}


// export interface ScreenTimeLimiter {
//   users: string[]
//   privatePassword: string
//   dailyAllowance: DailyAllowance

//   customCondition: Option<(now: DateTime) => boolean>,
// }

// export const ScreenTimeLimiter = {
//   constructor(dailyAllowance: DailyAllowance): ScreenTimeLimiter {
//     return {
//       users: [],
//       dailyAllowance,
//       customCondition: None(),
//     }
//   },

//   synchronize(me: ScreenTimeLimiter, now: DateTime) {
//     DailyAllowance.synchronize(me.dailyAllowance, now)
//   },

//   applyRegulationsOnOperatingSystemBootUp(me: ScreenTimeLimiter) {
    
//   },

//   apply() {

//   }
// }
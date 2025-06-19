import { App, CountdownTimer, DailyAllowance, DataHider, DateTime, Duration, MainUserAccessRegulator, Server, SuperUserAccessRegulator, TimeRange } from "../Prelude.ts";

function CreateDataHiderForm() {  
  return `
    <h2>Create Data Hider</h2>
    <form action="${Server.CreateDataHiderLink()}" method="GET">
      <label for="name">Name:</label>
      <input type="text" id="name" name="name" min="1" max="10" required><br><br>

      <label for="data">Data:</label>
      <input type="text" id="data" name="data" min="1" max="30" required><br><br>

      <label for="timer">Timer in days:</label>
      <input type="number" id="timer" name="timer" min="0" max="7" required><br><br>

      <button type="submit">Create</button>
    </form>
  `
}

function HTML(content: string) {
  return `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Discipline Beta</title>
    </head>
    <body>
      ${content}
    </body>
    </html>
  `
}

export function AppToHTML(me: App, now: DateTime) {
  return HTML(`
    <main class="app">
      <h1>Discipline Beta</h1>

      <h2>Current Date Time</h2>
      <p>${DateTime.toPrettyFormat(now, { hour12: true })}</p>

      <h2>Main User Access Regulator</h2>
      ${viewMainUserAccessRegulator(me.mainUserAccessRegulator, now)}

      <h2>Super User Access Regulator</h2>
      ${viewSuperUserAccessRegulator(me.superUserAccessRegulator, now)}

      <h2>Data Hiders</h2>
      ${Array(me.dataHiders.map(DataHiderToHTML))}

      ${CreateDataHiderForm()}
    </main>
  `)
}

function viewSuperUserAccessRegulator(me: SuperUserAccessRegulator, now: DateTime) {
  return `
    <div class="super_user_access_regulator">
      <p>username: ${me.username}</p>
      <p>is locked: ${me.isLocked ? "yes" : "no"}</p>
      <p>public password: ${me.publicPassword}</p>
      ${CountdownTimer.isDone(me.countdownTimer, now)
        ? `<p><a href="${Server.IncreaseSuperUserCountdownTimer()}">lock user</a></p>`
        : `<p>user will unlock in ${Duration.toPrettyFormat(me.countdownTimer.remainingDuration)}</p>
           <p><a href="${Server.IncreaseSuperUserCountdownTimer()}">Increase by five minutes</a></p>
          `
      }
    </div>
  `
}

function viewMainUserAccessRegulator(me: MainUserAccessRegulator, now: DateTime) {
  return `
    <div class="main_user_access_regulator">
      <p>username: ${me.username}</p>
      <p>is locked: ${me.isLocked ? "yes" : "no"}</p>
      <p>public password: ${me.publicPassword}</p>
      ${DailyAllowance.isDone(me.dailyAllowance, now)
        ? `<p>daily allowance finished. This user should be logged out and locked now.</p>`
        : `<p>daily allowance ends in ${Duration.toPrettyFormat(me.dailyAllowance.remainingAllowance)}</p>
           <p><a href="${Server.DecreaseMainUserDailyAllowance()}">decrease daily allowance by five minutes</a></p>
          `
      }

      <p>Night Time: ${TimeRange.toString(me.nightTime, { hour12: true, second: true })}</p>
      <p>Night Time Active: ${TimeRange.contains(me.nightTime, DateTime.time(now))}</p>

      <p>Dhuher Time: ${TimeRange.toString(me.dhuherTime, { hour12: true, second: true })}</p>
      <p>Dhuher Time Active: ${TimeRange.contains(me.dhuherTime, DateTime.time(now))}</p>

      <p>Asr Time: ${TimeRange.toString(me.asrTime, { hour12: true, second: true })}</p>
      <p>Asr Time Active: ${TimeRange.contains(me.asrTime, DateTime.time(now))}</p>

      <p>Maghrib Time: ${TimeRange.toString(me.maghribTime, { hour12: true, second: true })}</p>
      <p>Maghrib Time Active: ${TimeRange.contains(me.maghribTime, DateTime.time(now))}</p>
    </div>
  `
}

// function ScreenTimeLimiterToHTML(me: ScreenTimeLimiter) {
//   if (DailyAllowance.isDone(me.dailyAllowance, DateTime.now())) {
//     return `
//       <h2>Screen Time Limiter</h2>
//       <h3>Remaining Daily Allowance</h3>
//       <p>Daily allowance is finished. Device is now shutdown.</p>
//     `
//   } else {
//     return `
//       <h2>Screen Time Limiter</h2>
//       <h3>Remaining Daily Allowance</h3>
//       <p>Device will shutdown in ${Duration.toPrettyFormat(me.dailyAllowance.remainingAllowance)}</p>
//       <a href="${Server.DecreaseScreenTimeLimiterAllowanceLink()}">Decrease by five minutes</a>
//     `
//   }
// }

function DataHiderToHTML(me: DataHider) {
  if (CountdownTimer.isDone(me.countdownTimer, DateTime.now())) {
    return `
      <h3>${me.name}</h3>
      <p>Data is: ${me.data}</p>
      <a href="${Server.IncreaseDataHiderCountdownTimerLink(me.name)}">Lock data</a>
      <a href="${Server.DeleteDataHiderLink(me.name)}">Delete</a>
    `
  } else {
    return `
      <h3>${me.name}</h3>
      <p>Data will be public in ${Duration.toPrettyFormat(me.countdownTimer.remainingDuration)}</p>
      <a href="${Server.IncreaseDataHiderCountdownTimerLink(me.name)}">Increase by five minutes</a>
      <a href="${Server.DeleteDataHiderLink(me.name)}">Delete</a>
    `
  }
}

function Array(array: string[]) {
  let x = ""
  for (const item of array) {
    x += item
  }
  return x
}
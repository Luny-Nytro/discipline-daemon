import { Duration, isNone, App, UI, Operations, DateTime } from "./Prelude.ts";
import * as HTTP from "./Lib/HTTP.ts"

const serverOrigin = "http://localhost:9090"
const createDataHiderPath = "/api/CreateDataHider"
const deleteDataHiderPath = "/api/DeleteDataHider"
const decreaseMainUserDailyAllowancePath = "/api/DecreaseMainUserDailyAllowance"
const increaseDataHiderCountdownTimerPath = "/api/IncreaseDataHiderCountdownTimer"
const increaseSuperUserCountdownTimerPath = "/api/IncreaseSuperUserCountdownTimerPath"

export function CreateDataHiderLink() {
  return `${serverOrigin}${createDataHiderPath}`
}
export function DeleteDataHiderLink(name: string) {
  const url = new URL(`${serverOrigin}${deleteDataHiderPath}`)
  url.searchParams.append("name", name)
  return url
}
export function DecreaseMainUserDailyAllowance() {
  return `${serverOrigin}${decreaseMainUserDailyAllowancePath}`
}
export function IncreaseDataHiderCountdownTimerLink(name: string) {
  const url = new URL(`${serverOrigin}${increaseDataHiderCountdownTimerPath}`)
  url.searchParams.append("name", name)
  return url
}
export function IncreaseSuperUserCountdownTimer() {
  return `${serverOrigin}${increaseSuperUserCountdownTimerPath}`
}

function homePage(app: App) {
  return HTTP.HTMLResponse(UI.AppToHTML(app, DateTime.now()))
}

function createDataHider(app: App, url: URL) {
  const nameOrNull = url.searchParams.get("name")
  const dataOrNull = url.searchParams.get("data")
  const guardForOrNull = url.searchParams.get("timer")
  if (nameOrNull === null) {
    return HTTP.BadRequest("Operation failed: argument 'name' was not provided.")
  }
  if (dataOrNull === null) {
    return HTTP.BadRequest("Operation failed: argument 'data' was not provided.")
  }
  if (guardForOrNull === null) {
    return HTTP.BadRequest("Operation failed: argument 'guardFor' was not provided.")
  }

  const guardForOrNaN = parseInt(guardForOrNull)
  if (Object.is(guardForOrNaN, NaN)) {
    return HTTP.BadRequest("Operation failed: argumnet `guardFor` is outside valid range, which is 1 .. 7, inclusive both.")
  }

  const maybeGuardFor = Duration.fromDays(guardForOrNaN)
  if (isNone(maybeGuardFor)) {
    return HTTP.BadRequest("Operation failed: argumnet `guardFor` is outside valid range, which is 1 .. 7, inclusive both.")
  }

  const outcome = Operations.createDataHider(
    app, 
    nameOrNull, 
    dataOrNull, 
    maybeGuardFor.value,
  )

  if (isNone(outcome)) {
    return HTTP.OkResponse("Operation succeeded: Data Hider created successfully.")
  }
  switch (outcome.value) {
    case Operations.CreateDataHiderError.DataHiderCreationLimitReached: {
      return HTTP.BadRequest("Operation failed: You have reached the maximum number of allowed Data Hiders. Please delete an existing Data Hider before creating a new one.")
    }
    case Operations.CreateDataHiderError.DataHiderNameIsAlreadyInUse: {
      return HTTP.BadRequest("Operation failed: Provided name is already used by another data hider.")
    }
    case Operations.CreateDataHiderError.DataHiderNameIsLongerThan40Characters: {
      return HTTP.BadRequest("Operation failed: argument 'name' may not be longer than 40 characters.")
    }
    case Operations.CreateDataHiderError.DataHiderIsDataLongerThan40Characters: {
      return HTTP.BadRequest("Operation failed: argument 'data' may not be longer than 40 characters.")
    }
    case Operations.CreateDataHiderError.GuardForIsLongerThanOneWeek: {
      return HTTP.BadRequest("Operation failed: argumnet `guardFor` is outside valid range, which is 1 .. 7, inclusive both.")
    }
  }
}

function increaseDataHiderCountdownTimer(app: App, url: URL) {
  const name = url.searchParams.get("name")
  if (name === null) {
    return HTTP.BadRequest("Operation failed: argument 'name' not provided.")
  }

  const outcome = Operations.increaseDataHiderCountdownTimer(app, name)
  if (isNone(outcome)) {
    return HTTP.OkResponse("Operation succeeded: Increased data hider countdown timer by five minutes.")
  }
  switch (outcome.value) {
    case Operations.IncreaseDataHiderCountdownTimerError.NoSuchDataHider: {
      return HTTP.BadRequest(`Operation failed: No data hider with the given name exists. Given name: ${name}.`)
    }
    case Operations.IncreaseDataHiderCountdownTimerError.DataWouldBeGuardedForMoreThanOneWeek: {
      return HTTP.BadRequest("Operation failed: Increasing the countdown timer would make it count for more than a week, which is prohibited for safety.")
    }
  }
}

function deleteDataHider(app: App, url: URL) {
  const name = url.searchParams.get("name")
  if (name === null) {
    return HTTP.BadRequest("Operation failed: argument 'name' not provided.")
  }

  const outcome = Operations.deleteDataHider(app, name)
  if (isNone(outcome)) {
    return HTTP.OkResponse(`Operation succeeded: Data hider "${name}" deleted.`)
  }
  switch (outcome.value) {
    case Operations.DeleteDataHiderError.NoSuchDataHider: {
      return HTTP.BadRequest(`Operation failed: No data hider named "${name}" exists.`)
    }
  }
}

export function increaseSuperUserCountdownTimer(app: App) {
  const outcome = Operations.increaseSuperUserCountdownTimer(app)
  if (isNone(outcome)) {
    return HTTP.OkResponse("Operation succeeded: Increased Super User Countdown Timer by five minutes.")
  }
  switch (outcome.value) {
    case Operations.IncreaseSuperUserCountdownTimerError.SuperUserWouldBeLockedForMoreThanOneWeek: {
      return HTTP.BadRequest("Operation failed: Increasing the countdown timer would make it count for more than a week, which is prohibited for safety.")
    }
  }
}

export function decreaseMainUserDailyAllowance(app: App) {
  Operations.decreaseMainUserDailyAllowance(app)
  return HTTP.OkResponse("Operation succeeded: Decreased Main User Daily Allowance by five minutes or less.")
}

export function Server(app: App) {
  Deno.serve({
    transport: "tcp",
    port: 9090,
  }, (incoming) => {
    const url = new URL(incoming.url)
  
    if (incoming.method === "GET" && url.pathname === "/") {
      return homePage(app)
    }
    if (incoming.method === "GET" && url.pathname === createDataHiderPath) {
      return createDataHider(app, url)
    }
    if (incoming.method === "GET" && url.pathname === deleteDataHiderPath) {
      return deleteDataHider(app, url)
    }
    if (incoming.method === "GET" && url.pathname === decreaseMainUserDailyAllowancePath) {
      return decreaseMainUserDailyAllowance(app)
    }
    if (incoming.method === "GET" && url.pathname === increaseDataHiderCountdownTimerPath) {
      return increaseDataHiderCountdownTimer(app, url)
    }
    if (incoming.method === "GET" && url.pathname === increaseSuperUserCountdownTimerPath) {
      return increaseSuperUserCountdownTimer(app)
    }
    if (incoming.method === "GET" && url.pathname === "/style.css") {
      const css = Deno.readTextFileSync("/opt/shared/dev/packages/discipline_beta/UI/style.css")
      return new Response(css, {
        status: 200,
        statusText: "OK",
        headers: {
          "Content-Length": css.length.toString(),
          "Content-Type": "text/css",
        }
      })
    }

    return HTTP.NotFound()
  })
}
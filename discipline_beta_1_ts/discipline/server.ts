import { Application, Router, Context, Next, Status } from "https://deno.land/x/oak@v12.6.1/mod.ts" 
import { Discipline, Duration, runEvery } from "Pkg";

export async function createServer(discipline: Discipline, port: number) {
  const app = new Application()
  app.use(RequestsLimiter())
  
  const router = new Router()
  app.use(router.routes())

  router.get("/", (ctx) => {
    const state = discipline.toTextRepr()
    ctx.response.body = state
    ctx.response.headers.set("Content-Length", state.length.toString())
    ctx.response.headers.set("Content-Type", "application/json")
  })
  router.get("/actions", (ctx) => {
    const page = ActionsPage()
    ctx.response.body = page
    ctx.response.status = Status.OK
    ctx.response.headers.set("Content-Length", page.length.toString())
    ctx.response.headers.set("Content-Type", "text/html")
  })
  router.get("/sync_time", async (ctx) => {
    const maybeError = await discipline.syncTime()
    ctx.response.body = JSON.stringify(maybeError, null, 2)
  })
  router.get("/sync_user_access_regulator", async (ctx) => {
    const maybeError = await discipline.syncUserAccessRegulator()
    ctx.response.body = JSON.stringify(maybeError, null, 2)
  })
  router.get("/sync_device_access_regulator", async (ctx) => {
    const maybeError = await discipline.syncDeviceAccessRegulator()
    ctx.response.body = JSON.stringify(maybeError, null, 2)
  })
  router.get("/sync_network_access_regulator", async (ctx) => {
    const maybeError = await discipline.syncNetworkAccessRegulator()
    ctx.response.body = JSON.stringify(maybeError, null, 2)
  })

  try {
    await app.listen({ port })
    console.log("Listening on port: ", port)
  } catch (error) {
    console.log("Failed to run server: ", error)
  }
}

function RequestsLimiter() {
  // Defending against Denial of Service Attack
  const MAX_REQUESTS_PER_MINUTE = 40

  let counter = 0
  runEvery(Duration.fromMinutes(60), () => {
    counter = 0
  })

  return (ctx: Context, next: Next) => {
    if (counter >= MAX_REQUESTS_PER_MINUTE) {
      ctx.response.status = Status.TooManyRequests
      ctx.response.body = 'Excceded maximum requests per minute. Please wait another minute before making a new request'
    } else {
      counter++
      return next()
    }
  }
}

function ActionsPage() {
  return `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Document</title>
    </head>
    <body>
      <a href="/sync_time">Sync Time</a>
      <a href="/sync_user_access_regulator">Sync User Access Regulator</a>
      <a href="/sync_device_access_regulator">Sync Device Access Regulator</a>
      <a href="/sync_network_access_regulator">Sync Network Access Regulator</a>
    </body>
    </html>
  `
}
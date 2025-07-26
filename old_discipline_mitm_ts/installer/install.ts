import { join, fromFileUrl } from "https://deno.land/std@0.177.0/path/mod.ts";
import { build } from "./build.ts"

const createServiceFileContent = (execute: string, stdout: string, stderr: string) => `
[Unit]
Description="Provides Basic Self-Control Functionallity"
After=network.target local-fs.target

[Service]
Type=simple
ExecStart=${execute} 
StandardError=${stderr}
StandardOutput=${stdout}

[Install]
WantedBy=multi-user.target
`
const createServiceController = (serviceId: string) => `
const action = Deno.args[0]
switch (action) {
  case "enable": {
    new Deno.Command("systemctl", { args: [ "enable", "${serviceId}" ], stdout: "inherit", stderr: "inherit" }).outputSync()
    break
  }
  case "disable": {
    new Deno.Command("systemctl", { args: [ "disable", "${serviceId}" ], stdout: "inherit", stderr: "inherit" }).outputSync()
    break
  }
  case "start": {
    new Deno.Command("systemctl", { args: [ "start", "${serviceId}" ], stdout: "inherit", stderr: "inherit" }).outputSync()
    break
  }
  case "stop": {
    new Deno.Command("systemctl", { args: [ "stop", "${serviceId}" ], stdout: "inherit", stderr: "inherit" }).outputSync()
    break
  }
  default: {
    throw new Error("Unknown argument")
  }
}`


const generateRunFile = (deno: string, discipline: string, database: string) => `
await new Deno.Command("sudo", {
  args: [ "${deno}", "${discipline}", "${database}" ]
}).output()
`

const serviceId = Deno.args.at(0)
if (serviceId === undefined) {
  throw new Error("Service id not provided")
}
const destination = Deno.args.at(1)
if (destination === undefined) {
  throw new Error("Install location directory not provided")
}

const thisDirectory = join(fromFileUrl(import.meta.url), "../")
const sourceDeno = join(thisDirectory, "deno")
const sourceDiscipline = join(thisDirectory, "../discipline/run.ts")

const destinationRun = join(destination, "run.ts")
const destinationDeno = join(destination, "deno")
const destinationStderr = join(destination, "stderr")
const destinationStdout = join(destination, "stdout")
const destinationDatabase = join(destination, "database.json")
const destinationDiscipline = join(destination, "discipline.js")
const destinationServiceController = join(destination, "service.ts")

const exeute = `${destinationDeno} run --allow-all "${destinationDiscipline}" "${destinationDatabase}"`
const service = createServiceFileContent(exeute, `file:${destinationStdout}`, `file:${destinationStderr}`)
const serviceController = createServiceController(serviceId)
const run = generateRunFile(destinationDeno, destinationDiscipline, destinationDatabase)

Deno.writeTextFileSync(`/etc/systemd/system/${serviceId}.service`, service)
Deno.mkdirSync(destination, { recursive: true })
Deno.copyFileSync(sourceDeno, destinationDeno)
Deno.writeTextFileSync(destinationServiceController, serviceController)
Deno.writeTextFileSync(destinationRun, run)
await build(sourceDiscipline, destinationDiscipline)
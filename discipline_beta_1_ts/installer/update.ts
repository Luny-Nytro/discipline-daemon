import { join, fromFileUrl } from "https://deno.land/std@0.177.0/path/mod.ts";
import { build } from "./build.ts"

const destination = Deno.args.at(0)
if (destination === undefined) {
  throw new Error("Install location directory not provided")
}

const thisDirectory = join(fromFileUrl(import.meta.url), "../")
const sourceDiscipline = join(thisDirectory, "../discipline/run.ts")
const destinationDiscipline = join(destination, "./discipline.js")
await build(sourceDiscipline, destinationDiscipline)
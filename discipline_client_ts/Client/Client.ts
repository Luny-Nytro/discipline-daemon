import { Unique } from "../ElementaryTypes/Unique.ts";
import { Option, Some } from "../ElementaryTypes/Option.ts";

export type Client = Unique<"Discipline.Client", {
  readonly host: string
  readonly port: number
}>

export function create(host: string, port: number): Option<Client> {
  // TODO: Check whther host is a valid host and port is a valid port.
  return Some(Unique({
    host,
    port,
  }))
}

export function createOrThrow(host: string, port: number): Client {
  // TODO: Throw if host or port is invalid.
  return Unique({
    host,
    port,
  })
}
import { Some, None, Option } from "Pkg";

const debugMode = true

async function execute(command: string, ...args: string[]) {
  if (debugMode) {
    console.log(`execute(${command}, ${args})`)
  } else {
    await new Deno.Command(command, { args }).output()
  }
}

export const enum ErrorKind {
  UnknownOperatingSystemError,
  BlockNetworkError,
  AllowNetworkError,
  ShutdownError,
  SyncTimeError,
  UnknownError
}

export class UnknownOperatingSystemError {
  readonly kind = ErrorKind.UnknownOperatingSystemError
}

export class ChangeUserPasswordError {
  constructor(readonly inner: UnknownError) {}
}

export class BlockNetworkError {
  readonly kind = ErrorKind.BlockNetworkError
  constructor(readonly error: UnknownError) {}
} 

export class AllowNetworkError {
  readonly kind = ErrorKind.AllowNetworkError
  constructor(readonly error: UnknownError) {}
} 

export class ShutdownError {
  readonly kind = ErrorKind.ShutdownError
  constructor(readonly inner: UnknownError) {}
}

export class SyncTimeError {
  readonly kind = ErrorKind.SyncTimeError
  constructor(readonly inner: UnknownError) {}
}

export class UnknownError {
  readonly kind = ErrorKind.UnknownError
  constructor(readonly error: unknown) {}
}

async function linuxBlockNetwork(): Promise<Option<BlockNetworkError>> {
  try {
    await execute("systemctl", "stop", "NetworkManager")
    return new None()
  } catch (error) {
    return new Some(new BlockNetworkError(new UnknownError(error)))
  }
}

async function linuxAllowNetwork(): Promise<Option<AllowNetworkError>> {
  try {
    await execute("systemctl", "start", "NetworkManager")
    return new None()
  } catch (error) {
    return new Some(new AllowNetworkError(new UnknownError(error)))
  }
}

async function linuxShutdown() {
  try {
    await execute("shutdown", "-h", "now")
    return new None()
  } catch (error) {
    return new Some(new ShutdownError(new UnknownError(error)))
  }
}

const decoder = new TextDecoder()
const encoder = new TextEncoder()

async function linuxChangeUserPassword(username: string, newPassword: string): Promise<Option<ChangeUserPasswordError>> {
  if (debugMode) {
    console.log(`linuxChangeUserPassword(${username}, ${newPassword})`)
    return new None()
  }
  
  try {
    const process = new Deno.Command("chpasswd", {
      stdin: "piped",
    }).spawn()
  
    const stdin = process.stdin.getWriter()
    await stdin.write(encoder.encode(`${username}:${newPassword}\n`));
    await stdin.close()
    return new None()
  } catch (error) {
    return new Some(new ChangeUserPasswordError(new UnknownError(error)))
  }
}

async function linuxSyncTime() {
  try {
    await execute("ntpdate", "-s", "time.nist.gov")
    return new None()
  } catch (error) {
    return new Some(new SyncTimeError(new UnknownError(error)))
  }
}

export async function blockNetwork() {
  switch (Deno.build.os) {
    case "linux": return linuxBlockNetwork()
    default: return new Some(new UnknownOperatingSystemError())
  }
}

export async function allowNetwork() {
  switch (Deno.build.os) {
    case "linux": return linuxAllowNetwork()
    default: return new Some(new UnknownOperatingSystemError())
  }
}

export async function shutdown() {
  switch (Deno.build.os) {
    case "linux": return linuxShutdown()
    default: return new Some(new UnknownOperatingSystemError())
  }
}

export async function changeUserPassword(username: string, newPassword: string) {
  switch (Deno.build.os) {
    case "linux": return linuxChangeUserPassword(username, newPassword)
    default: return new Some(new UnknownOperatingSystemError())
  }
}

export async function syncTime() {
  switch (Deno.build.os) {
    case "linux": return linuxSyncTime()
    default: return new Some(new UnknownOperatingSystemError())
  }
}

// async function windowsAllowNetwork() {
//   try {
//     await execute("netsh", "interface", "set", "interface", "Wi-Fi", "enable")
//     await execute("netsh", "interface", "set", "interface", "Ethernet", "enable")
//   } catch {
//     throw new AllowNetworkError()
//   }
// }
// async function blockNetworkWindows() {
//   try {
//     await execute("netsh", "interface", "set", "interface", "Wi-Fi", "disable")
//     await execute("netsh", "interface", "set", "interface", "Ethernet", "disable")
//   } catch (error) {
//     throw new BlockNetworkError()
//   }
// }

// async function windowsShutdown() {
//   try {
//     await execute("shutdown", "/p")
//   } catch {
//     throw new ShutdownError()
//   }
// }

// async function windowsChangeUserPassword(username: string, newPassword: string): Promise<void> {
//   await execute(`net`, `user`, username, newPassword)
// }

// async function windowsSyncTime() {
//   await execute("w32tm", "/resync")
// }
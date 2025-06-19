import { Unique } from "@Pkg/Unique";
import { Uuid } from "@Pkg/Uuid";
import { OperatingSystemUsername } from "@Pkg/OperatingSystemUsername";
import { Option } from "@Pkg/Option";
import { OperatingSystemPassword } from "@Pkg/OperatingSystemPassword";
import * as UserBlocker from "@Pkg/UserAccess"
import { Err, Ok, Tried } from "@Pkg/Tried";
import { GenericError, GenericError1 } from "@Pkg/GenericError";
import { isErr } from "@Pkg/Tried";
import { Integer } from "@Pkg/Integer";
import { String } from "@Pkg/String";
import { OperatingSystemUserId } from "../CommonTypes/OperatingSystemUserId.ts";

export type Username = Unique<"App.User.Name", string>

export const Username = {
  MIN_LENGTH: 1,
  MAX_LENGTH: 10,

  new(username: string): Tried<Username, GenericError> {
    if (username.length < Username.MIN_LENGTH) {
      const error = GenericError.new("Create username")
      GenericError.addMessage(error, "Argument 'username' is too short")
      GenericError.addNamedAttachment(error, "Minimum length", Username.MIN_LENGTH.toString())
      return Err(error)
    }

    if (username.length > Username.MAX_LENGTH) {
      const error = GenericError.new("Create username")
      GenericError.addMessage(error, "Argument 'username' is too long")
      GenericError.addNamedAttachment(error, "Maximum length", Username.MIN_LENGTH.toString())
      return Err(error)
    }

    return Ok(Unique(username))
  },

  newOrThrow(username: string): Username {
    const maybeErr = Username.new(username)
    if (isErr(maybeErr)) {
      throw GenericError.asNativeError(Tried.error(maybeErr))
    }

    return Tried.value(maybeErr)
  },
}

export type User = Unique<"App.User", {
  id: Uuid
  name: Username
  operatingSystemUsername: OperatingSystemUsername
  operatingSystemPassword: OperatingSystemPassword
  userBlocker: UserBlocker.Blocker
}>

export const User = {
  new() {

  }
}

export type UserCreator = Unique<"App.User.Creator", {
  readonly id: Option<Uuid>
  readonly name: Username
  readonly operatingSystemUsername: OperatingSystemUsername
  readonly operatingSystemPassword: OperatingSystemPassword
}>

export const UserCreator = {
  new(
    id: Option<Uuid>,
    name: Username,
    operatingSystemUsername: OperatingSystemUsername,
    operatingSystemPassword: OperatingSystemPassword,
  ): UserCreator {
    return Unique({
      id, name, operatingSystemPassword, operatingSystemUsername
    })
  },

  create(me: UserCreator): Promise<Tried<User, GenericError>> {
    
  }
}

const OperatingSystemCalls = {
  async getUserId(username: OperatingSystemUsername): 
    Promise<Tried<OperatingSystemUserId, GenericError>> 
  {
    let command: Deno.Command
    try {
      command = new Deno.Command("id", {
        args: ["-u", OperatingSystemUsername.asString(username)],
        stdout: "piped"
      });
    } catch (exception) {
      const error = GenericError.new("get operating system user id")
      GenericError.addMessage(error, "An exception was thrown when creating a Deno.Command to spawn the 'id' command")
      GenericError.addNamedAttachment(error, "exception", String.fromUnknown(exception))
      return Err(error)
    }

    let output: Deno.CommandOutput
    try {
      output = await command.output()
    } catch (exception) {
      const error = GenericError.new("get operating system user id")
      GenericError.addMessage(error, "An exception was thrown when exeucting the 'id' command")
      GenericError.addNamedAttachment(error, "exception", String.fromUnknown(exception))
      return Err(error)
    }
    
    if (output.success) {
      const maybeString = String.fromUtf8(output.stdout)
      if (isErr(maybeString)) {
        const error = Tried.error(maybeString)
        GenericError.changeContext(error, "get operating system user id")
        GenericError.addMessage(error, "Failed to parse the output of the 'id' command as string")
        return Err(error)
      }

      const maybeInteger = Integer.fromString(Tried.value(maybeString))
      if (isErr(maybeInteger)) {
        const error = Tried.error(maybeInteger)
        GenericError.changeContext(error, "get operating system user id")
        GenericError.addMessage(error, "Failed to deserialize operating system user id")
        return Err(error)
      }

      const maybeUserId = OperatingSystemUserId.new(Tried.value(maybeInteger))
      if (isErr(maybeUserId)) {
        const error = Tried.error(maybeUserId)
        GenericError.changeContext(error, "get operating system user id")
        GenericError.addMessage(error, "'id' command returned an invalid operating system user id")
        return Err(error)
      }

      return maybeUserId
    }


    const maybeStderr = String.fromUtf8(output.stderr)
    if (isErr(maybeStderr)) {
      const error = Tried.error(maybeStderr)
      GenericError.changeContext(error, "get operating system user id")
      GenericError.addMessage(error, "'id' command exited with an error status")
      GenericError.addMessage(error, "Failed to parse the stderr of the 'id' command as string")
      return Err(error)
    }

    const stderr = Tried.value(maybeStderr)
    const error = GenericError.new("get operating system user id")
    GenericError.addMessage(error, "'id' command exited with an error status")
    GenericError.addNamedAttachment(error, "'id' command stderr", stderr)
    return Err(error)
  }
}
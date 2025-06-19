import { StatusIndicator, DateTime, None, Os } from "Pkg"

export class UserAccessRegulator {
  private constructor(
    public username: string,
    public password: string,
    public isBlocked: boolean,
    public blockIndicator: StatusIndicator,
  ) {}

  static new(
    username: string,
    password: string,
    blockIndicator: StatusIndicator,    
  ) {
    return new UserAccessRegulator(
      username, 
      password, 
      false, 
      blockIndicator,
    )
  }

  async block(privatePassword: string) {
    const maybeError = await Os.changeUserPassword(this.username, privatePassword)
    if (maybeError.kind === "none") {
      this.isBlocked = true
    }
    return maybeError
  }

  async allow() {
    const maybeError = await Os.changeUserPassword(this.username, this.password)
    if (maybeError.kind === "none") {
      this.isBlocked = false
    }
    return maybeError
  }

  sync(privatePassword: string) {
    const now = DateTime.now()
    if (this.blockIndicator.isActive(now)) {
      if (!this.isBlocked) {
        return this.block(privatePassword)
      } else {
        return new None()
      }
    } else {
      if (this.isBlocked) {
        return this.allow()
      } else {
        return new None()
      }
    }
  }

  clearCache() {
    this.isBlocked = false
  }

  static generatePrivatePassword = generatePrivatePassword
}

function generateRandomInteger(min: number, max: number) {
  // Ensure min and max are integers
  min = Math.ceil(min)
  max = Math.floor(max)

  // Generate a random integer between min and max, inclusive
  return Math.floor(Math.random() * (max - min + 1)) + min
}

function generateRandomLowerCaseLetter() {
  return String.fromCharCode(generateRandomInteger(
    97  /** ASCII code for "a" */, 
    122 /** ASCII code for "z" */,
  ))
}

function generatePrivatePassword() {
  let password = ''
  for (let i = 0; i < 10; i++) {
    password += generateRandomLowerCaseLetter()
  }
  return password
}
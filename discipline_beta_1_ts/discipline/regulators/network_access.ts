import { StatusIndicator, DateTime, Os, None } from "Pkg"

export class NetworkAccessRegulator {
  private constructor(
    public blockIndicator: StatusIndicator,
    public isAllowed: boolean,
  ) {}

  static new(blockIndicator: StatusIndicator) {
    return new NetworkAccessRegulator(blockIndicator, true)
  }

  async allow() {
    const maybeError = await Os.allowNetwork()
    if (maybeError.kind === "none") {
      this.isAllowed = true
    } 
    return maybeError
  } 
  
  async block() {
    const maybeError = await Os.blockNetwork()
    if (maybeError.kind === "none") {
      this.isAllowed = false
    }
    return maybeError
  } 

  sync(now = DateTime.now()) {
    if (this.blockIndicator.isActive(now)) {
      if (this.isAllowed) {
        return this.block()
      } else {
        return new None()
      }
    } else {
      if (!this.isAllowed) {
        return this.allow()
      } else {
        return new None()
      }
    }
  }

  clearCache() {
    this.isAllowed = true
  }
}

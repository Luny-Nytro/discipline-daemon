import { StatusIndicator, DateTime, None, Os } from "Pkg"

export class DeviceAccessRegulator {
  private constructor(readonly blockIndicator: StatusIndicator) {}
  
  static new(blockIndicator: StatusIndicator) {
    return new DeviceAccessRegulator(blockIndicator)
  }

  sync(now = DateTime.now()) {
    if (this.blockIndicator.isActive(now)) {
      return Os.shutdown()
    } else {
      return new None()
    }
  }
}
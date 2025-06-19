import { DateTime, None, Os } from "Pkg"

export class TimeSyncer {
  private constructor(
    public previousSync: DateTime | null,
  ) {}

  static new() {
    return new TimeSyncer(null)
  }

  async sync(now = DateTime.now()) {
    if (this.previousSync === null || this.previousSync.until(now).minutes() > 5) {
      this.previousSync = now
      return await Os.syncTime()
    }
    return new None()
  }
}
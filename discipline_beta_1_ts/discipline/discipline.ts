import { createServer } from "./server.ts";
import { 
  DateTime,
  TimeSyncer,
  UserAccessRegulator, 
  NetworkAccessRegulator, 
  DeviceAccessRegulator, 
  Fs,
  None,
  Some,
  Duration,
  Ok,
  ToPlainText,
  HasCompactJSONRepr,
  runEvery,
} from "Pkg"


export class Discipline {
  private constructor(
    readonly path: string,
    readonly timeSyncer: TimeSyncer,
    readonly privatePassword: string,
    readonly userAccessRegulator: UserAccessRegulator,
    readonly deviceAccessRegulrator: DeviceAccessRegulator,
    readonly networkAccessRegulrator: NetworkAccessRegulator,
  ) {}

  static new(
    path: string,
    timeSyncer: TimeSyncer,
    privatePassword: string,
    userAccessRegulator: UserAccessRegulator,
    deviceAccessRegulrator: DeviceAccessRegulator,
    networkAccessRegulrator: NetworkAccessRegulator,
  ) {
    return new Discipline(
      path,
      timeSyncer,
      privatePassword,
      userAccessRegulator,
      deviceAccessRegulrator,
      networkAccessRegulrator,
    )
  }

  static async open(directoryPath: string, initialize: (path: string) => Discipline) {
    const maybeJson = await Fs.read<HasCompactJSONRepr.Discipline.JsonRepr>(
      directoryPath, 
      () => HasCompactJSONRepr.Discipline.serialize(initialize(directoryPath)),
    )

    return maybeJson.kind === "ok"
      ? new Ok(HasCompactJSONRepr.Discipline.deserialize(directoryPath, maybeJson.ok))
      : maybeJson
  }
  
  async sync() {
    const maybeError1 = await this.deviceAccessRegulrator.sync(DateTime.now())
    if (maybeError1.kind === "some") {
      return new Some(maybeError1.value)
    }

    const maybeError2 = await this.networkAccessRegulrator.sync(DateTime.now())
    if (maybeError2.kind === "some") {
      return new Some(maybeError2.value)
    }

    const maybeError3 = await this.userAccessRegulator.sync(this.privatePassword)
    if (maybeError3.kind === "some") {
      return new Some(maybeError3.value)
    }

    const maybeError4 = await this.save()
    if (maybeError4.kind === "err") {
      return new Some(maybeError4.err)
    }

    return new None()
  }

  async runSyncingLoop() {
    await this.syncDeviceAccessRegulator()
    await this.syncUserAccessRegulator()
    await this.syncNetworkAccessRegulator()

    runEvery(Duration.fromMinutes(5), () => {
      this.syncUserAccessRegulator()
    })

    runEvery(Duration.fromMinutes(1), () => {
      this.syncNetworkAccessRegulator()
    })

    runEvery(Duration.fromSeconds(5), () => {
      this.syncDeviceAccessRegulator()
    })
  }

  syncTime() {
    return this.timeSyncer.sync()
  }

  async syncUserAccessRegulator() {
    const maybeError = await this.userAccessRegulator.sync(this.privatePassword)
    if (maybeError.kind === "some") {
      console.trace(maybeError.value)
      return new Some(maybeError.value)
    }
    
    const maybeError1 = await this.save()
    if (maybeError1.kind === "err") {
      console.trace(maybeError1.err)
      return new Some(maybeError1.err)
    }

    return new None()
  }
  
  async syncDeviceAccessRegulator() {
    const maybeError = await this.deviceAccessRegulrator.sync()
    if (maybeError.kind === "some") {
      console.trace(maybeError.value)
    }
    return maybeError
  }
  
  async syncNetworkAccessRegulator() {
    const maybeError = await this.networkAccessRegulrator.sync()
    if (maybeError.kind === "some") {
      console.trace(maybeError.value)
    }
    return maybeError
  }

  async runServer(port: number) {
    return await createServer(this, port)
  }

  save() {
    return Fs.save(this.path, HasCompactJSONRepr.Discipline.serialize(this))
  }

  toTextRepr() {
    return ToPlainText.discipline(this)
  }
}

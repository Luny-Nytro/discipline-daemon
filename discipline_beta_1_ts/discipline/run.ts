import { 
  TimeSyncer,
  Discipline, 
  UserAccessRegulator, 
  DeviceAccessRegulator,
  NetworkAccessRegulator,
  Minute,
  Hour,
  Duration,
  DurationStatusIndicator,
  HourRangeStatusIndicator,
  MinuteRangeStatusIndicator,
  OrStatusIndicator,
  Time,
  TimeRangeStatusIndicator,
} from "Pkg"


function initialize(path: string) {
  return Discipline.new(
    path,
    TimeSyncer.new(),
    UserAccessRegulator.generatePrivatePassword(),

    UserAccessRegulator.new(
      "xxzendrom",
      "xxzendrom",
      DurationStatusIndicator.new(Duration.fromDays(30)),
    ),

    DeviceAccessRegulator.new(new OrStatusIndicator([
      // 1. Go to sleep!
      new HourRangeStatusIndicator(Hour.am(1), Hour.am(5)),
      new HourRangeStatusIndicator(Hour.pm(6), Hour.pm(12)),

      // 2. Take a break from the PC.
      new MinuteRangeStatusIndicator(Minute.new1(1), Minute.new1(5)),
      
      // 3. Do Fajar Salah
      new TimeRangeStatusIndicator(
        new Time(Hour.am(4), Minute.new1(30)),
        new Time(Hour.am(5), Minute.new1(30)),
      ),
      
      // 4. Do Dhuhr Salah: 12:5 pm
      //    Block from 11:30 am to 12:30 pm
      new TimeRangeStatusIndicator(
        new Time(Hour.am(11), Minute.new1(45)),
        new Time(Hour.am(12), Minute.new1(30))
      ),

      // 5. Do Asr Prayer: 3:30 pm
      //    Block from 3 pm to 4 pm
      new TimeRangeStatusIndicator(
        new Time(Hour.pm(3), Minute.new1(35)),
        new Time(Hour.pm(4), Minute.new1(10)),
      ),

      // 6. Do Maghrib Prayer: 6:2 pm
      //    Block from 6 to 7
      new TimeRangeStatusIndicator(
        new Time(Hour.pm(6), Minute.new1(30)),
        new Time(Hour.pm(7), Minute.new1(15)),
      ),

      // 7. Isha Prayer: 7:16 pm
      //    Block 7 to 8
      new TimeRangeStatusIndicator(
        new Time(Hour.pm(8), Minute.new1(1)),
        new Time(Hour.pm(9), Minute.new1(1)),
      )
    ])),

    NetworkAccessRegulator.new(new OrStatusIndicator([
      new MinuteRangeStatusIndicator(
        Minute.new1(1), 
        Minute.new1(15),
      ),
      new MinuteRangeStatusIndicator(
        Minute.new1(30), 
        Minute.new1(45),
      ),
    ])),
  )
}

const databasePath = Deno.args.at(0)
if (databasePath === undefined) {
  throw new Error("No database path provided!")
}

const discipline = await Discipline.open(databasePath, initialize)
if (discipline.kind === "ok") {
  await discipline.ok.runSyncingLoop()
  await discipline.ok.runServer(2020)
} else {
  console.log(discipline.err)
}
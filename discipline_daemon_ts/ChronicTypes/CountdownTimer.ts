import * as Displayer from "../ElementaryTypes/Display.ts";
import * as Duration from "./Duration.ts";
import * as DateTime from "./DateTime.ts";
import * as JsonSerializer from "../ElementaryTypes/JsonSerde/JsonSerializer.ts";
import * as JsonDeserializer from "../ElementaryTypes/JsonSerde/JsonDeserializer.ts";
import { Tried } from "../ElementaryTypes/Tried.ts";
import { Unique } from "../ElementaryTypes/Unique.ts";

export type CountdownTimer = Unique<"App.Chronic.CountdownTimer", {
  duration: Duration.Duration
  remainingDuration: Duration.Duration
  previousSynchronizationTime: DateTime.DateTime
}>

export function factory(
  duration: Duration.Duration,
  remainingDuration: Duration.Duration,
  previousSynchronizationTime: DateTime.DateTime,
): CountdownTimer {
  return Unique({
    duration,
    remainingDuration,
    previousSynchronizationTime,
  })
}

export function duration(me: CountdownTimer): Duration.Duration {
  return me.duration
}

export function remainingDurationUpdated(me: CountdownTimer, now: DateTime.DateTime): Duration.Duration {
  synchronize(me, now)
  return me.remainingDuration
}

export function remainingDurationOutdated(me: CountdownTimer): Duration.Duration {
  return me.remainingDuration
}

export function previousSynchronizationTime(me: CountdownTimer): DateTime.DateTime {
  return me.previousSynchronizationTime
}

export function synchronize(me: CountdownTimer, now: DateTime.DateTime): void {    
  me.remainingDuration = Duration.minusOrZero(
    me.remainingDuration,
    DateTime.tillOrZero(me.previousSynchronizationTime, now)
  )

  me.previousSynchronizationTime = now
}

export function isRunningUpdated(me: CountdownTimer, now: DateTime.DateTime): boolean {
  synchronize(me, now)
  return !Duration.isZero(me.remainingDuration)
}

export function isRunningOutdated(me: CountdownTimer): boolean {
  return !Duration.isZero(me.remainingDuration)
}

export function isFinishedUpdated(me: CountdownTimer, now: DateTime.DateTime): boolean {
  synchronize(me ,now)
  return Duration.isZero(me.remainingDuration)
}

export function isFinishedOutdated(me: CountdownTimer): boolean {
  return Duration.isZero(me.remainingDuration)
}

export function reinitialize(me: CountdownTimer, now: DateTime.DateTime): void {
  me.remainingDuration = me.duration
  me.previousSynchronizationTime = now
}

export const displayer = Displayer.implement<CountdownTimer>(me => 
  Displayer.asNamedObject("CountdownTimer", 
    "duration", Duration.displayer, me.duration,
    "remainingDuration", Duration.displayer, me.remainingDuration,
    "previousSynchronizationTime", DateTime.displayer, me.previousSynchronizationTime,
  )
)

export const jsonSerializer = JsonSerializer.implement<CountdownTimer>(me => 
  JsonSerializer.asObject(
    "duration", Duration.jsonSerializer, me.duration,
    "remaining_duration", Duration.jsonSerializer, me.remainingDuration,
    "previous_synchronization_time", DateTime.jsonSerializer, me.previousSynchronizationTime,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<CountdownTimer>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
    JsonDeserializer.propertyAs(context, "duration", Duration.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "remaining_duration", Duration.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "previous_synchronization_time", DateTime.jsonDeserializer),
    factory
  ))
)

export const CountdownTimer = {
  factory,
  duration,
  remainingDurationUpdated,
  remainingDurationOutdated,
  previousSynchronizationTime,
  synchronize,
  isRunningUpdated,
  isRunningOutdated,
  isFinishedUpdated,
  isFinishedOutdated,
  reinitialize,
}
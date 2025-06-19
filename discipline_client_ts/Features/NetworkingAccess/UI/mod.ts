import * as OperatingSystemUsername from "@Pkg/OperatingSystemUsername"
import * as Displayer from "@Pkg/Display"
import * as CountdownTimer from "@Pkg/CountdownTimer"
import * as DateTime from "@Pkg/DateTime"
import * as Hour from "@Pkg/Hour"
import * as TimeRange from "@Pkg/TimeRange"
import * as Time from "@Pkg/Time"
import * as Weekday from "@Pkg/Weekday"
import * as WeekdayRange from "@Pkg/WeekdayRange"
import * as Duration from "@Pkg/Duration"
import * as MultiLineText from "@Pkg/MultiLineText"
import * as Feature from "../Feature.ts"
import * as Activator from "../Activator.ts"
import * as Enabler from "../Enabler.ts"
import { Rule } from "@Pkg/NetworkingAccess";

export function display(feature: Feature.Feature, now: DateTime.DateTime) {
  return MultiLineText.asString(MultiLineText.create([
    "Networking Access Enforcing Enabled",
    feature.ruleRnforcers.map(ruleEnforcer => 
      `Rule enforcing is ${
        ruleEnforcer.isEnabled
          ? "enabled"
          : "disabled"
      } for user ${
        OperatingSystemUsername.asString(
          ruleEnforcer.username
        )
      }`      
    ),

    "Networking Access Blocking",
    feature.ruleRnforcers.map(ruleEnforcer => 
      `Networking access is ${
        ruleEnforcer.isBlocked
          ? "blocked"
          : "allowed"
      } for user ${
        OperatingSystemUsername.asString(
          ruleEnforcer.username
        )
      }`      
    ),

    "Networking Access Rules",
    feature.ruleRnforcers.map(ruleEnforcer => [
      `Rules for user ${
        OperatingSystemUsername.asString(ruleEnforcer.username)
      }`,
      
      ruleEnforcer.rules.map(rule => [
        `${Activator.match(rule.activator, {
          AtHour: hour =>
            `Blocks at ${Hour.asNumber12AndMeridiemString(hour)}`,

          AtWeekday: weekday => 
            `Blocks on ${Weekday.asString(weekday)}`,

          ForDuration: timer => 
            `Blocks for ${Duration.asString(CountdownTimer.remainingDurationUpdated(timer, now))}`,

          InTimeRange: range => 
            `Blocks in ${Time.asString(TimeRange.from(range))} ... ${Time.asString(TimeRange.till(range))}`,

          InWeekdayRange: range =>
            `Blocks in ${Weekday.asString(WeekdayRange.from(range))} ... ${Weekday.asString(WeekdayRange.till(range))}`,

          NotAtHour: hour => 
            `Blocks all the time except at ${Hour.asNumber12AndMeridiemString(hour)}`,

          NotAtWeekday: weekday => 
            `Blocks all the time except on ${Weekday.asString(weekday)}`,

          NotInTimeRange: range =>
            `Blocks all the time except in ${Time.asString(TimeRange.from(range))} ... ${Time.asString(TimeRange.till(range))}`,
          
          NotInWeekdayRange: range =>
            `Blocks all the time except in ${Weekday.asString(WeekdayRange.from(range))} ... ${Weekday.asString(WeekdayRange.till(range))}`,
        })}`,
        [
          `Is Enabled: ${Rule.isEnabled(rule, now)}`,
          `Is Effective: ${Rule.isEffective(rule, now)}`,
           
        ]
      ])
    ]),
  ]))
}
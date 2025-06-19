import { AppLoader } from "./App.ts";
import { App, CountdownTimer, DailyAllowance, Duration, isOk, MainUserAccessRegulator, Option, SuperUserAccessRegulator } from "./Prelude.ts";
import { generateRandomStringOf10LowerLetters } from "./Lib/Random.ts";

const appLoader = await AppLoader.load(
  "/opt/shared/dev/packages/discipline_beta/Data/data.json",
  () => 
    App.constructor(
      [], 
      MainUserAccessRegulator.new(
        "luny",
        false,
        "luny of the lunar lands",
        generateRandomStringOf10LowerLetters(),
        DailyAllowance.new(Option.unwrap(Duration.fromHours(3)))
      ),
      SuperUserAccessRegulator.new(
        "lunynytro",
        false,
        "luny of the lunar lands",
        generateRandomStringOf10LowerLetters(),
        CountdownTimer.new(Option.unwrap(Duration.fromDays(7)))
      )
    )
)

// console.log(appLoader)

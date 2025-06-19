import { Duration, Option } from "Discipline";
import { data } from "../../data";
import type { PageLoad  } from "./$types"

export const ssr = false

export const load: PageLoad = async (a) => {

  await sleep(Option.expect(
    Duration.fromSeconds(7), 
    "Create duration from 7 seconds"
  ))
  
  return {
    shadowVault: data.shadowVaults.shadowVaults[0]
  }
}

function sleep(duration: Duration) {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(null)
    }, Duration.milliseconds(duration));
  })
}
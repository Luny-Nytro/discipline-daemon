<script lang="ts">
  import { DateTime, Duration, Err, Error, isErr, isNone, None, Ok, Option, ShadowVaults, Some, Tried, Uuid } from "Discipline";
  import { discipline } from "../data";

  let rawName = $state("")
  let name = $derived(ShadowVaults.Name.new(rawName))

  let rawDatum = $state("")
  let datum = $derived(ShadowVaults.Datum.new(rawDatum))

  let rawDays = $state(3)
  let rawHours = $state(0)
  let rawMinutes = $state(0)

  function tryCreateDuration() {
    return Duration.fromDaysHoursMinutes(rawDays, rawHours, rawMinutes)
  }
  let duration = $derived(tryCreateDuration())

  function tryCreateCreator() {

  }
  // const enum ErrorType {
  //   CreateName,
  //   CreateDatum,
  //   CreateDurationFromDaysHoursMinute,
  //   CreateDateTimeFromInput,
  //   InputDateTimeIsEarilerThanNow,
  //   InvalidDomInput,
  //   DurationTooShort,
  //   DurationTooLong,
  // }

  // type Error = {
  //   readonly type: ErrorType.CreateName
  //   readonly error: ShadowVaults.CreateNameError
  // } | {
  //   readonly type: ErrorType.CreateDatum
  //   readonly error: ShadowVaults.DatumCreateError
  // } | {
  //   readonly type: ErrorType.CreateDurationFromDaysHoursMinute
  //   readonly days: number
  //   readonly hours: number
  //   readonly minutes: number
  // } | {
  //   readonly type: ErrorType.CreateDateTimeFromInput
  //   readonly input: string
  // } | {
  //   readonly type: ErrorType.InputDateTimeIsEarilerThanNow
  //   readonly inputDateTime: DateTime
  // } | {
  //   readonly type: ErrorType.InvalidDomInput
  // } | {
  //   readonly type: ErrorType.DurationTooShort
  //   readonly duration: Duration
  // } | {
  //   readonly type: ErrorType.DurationTooLong
  //   readonly duration: Duration
  // }

  // const Error = {
  //   CreateName(error: ShadowVaults.CreateNameError): Error {
  //     return {
  //       type: ErrorType.CreateName,
  //       error,
  //     }
  //   },

  //   CreateDatum(error: ShadowVaults.DatumCreateError): Error {
  //     return {
  //       type: ErrorType.CreateDatum,
  //       error,
  //     }
  //   },

  //   CreateDurationFromDaysHoursMinutes(days: number, hours: number, minutes: number): Error {
  //     return {
  //       type: ErrorType.CreateDurationFromDaysHoursMinute,
  //       days,
  //       hours,
  //       minutes,
  //     }
  //   },

  //   CreateDateTimeFromInput(input: string): Error {
  //     return {
  //       type: ErrorType.CreateDateTimeFromInput,
  //       input,
  //     }
  //   },

  //   InputDateTimeIsEarlierThanNow(inputDateTime: DateTime): Error {
  //     return {
  //       type: ErrorType.InputDateTimeIsEarilerThanNow,
  //       inputDateTime,
  //     }
  //   },

  //   InvalidDomInput(): Error {
  //     return {
  //       type: ErrorType.InvalidDomInput,
  //     }
  //   },

  //   DurationTooShort(duration: Duration): Error {
  //     return {
  //       type: ErrorType.DurationTooShort,
  //       duration,
  //     }
  //   },

  //   DurationTooLong(duration: Duration): Error {
  //     return {
  //       type: ErrorType.DurationTooLong,
  //       duration,
  //     }
  //   }
  // }

  // function match2<Value, Candidate1, Candidate2, Return>(
  //   value: Value,
  //   candidate1: Candidate1,
  //   fn1: (value: Candidate1) => Return,
  //   candidate2: Candidate2,
  //   fn2: (value: Candidate2) => Return,
  //   otherwise: () => Return,
  // ) {
  //   switch (value as any) {
  //     case candidate1: return fn1(candidate1)
  //     case candidate2: return fn2(candidate2)
  //     default: return otherwise()
  //   }
  // }

  // function when2<Value1, Value2, Otherwise>(
  //   condition1: boolean,
  //   fn1: () => Value1,
  //   condition2: boolean,
  //   fn2: () => Value2,
  //   otherwise: () => Otherwise,
  // ) {
  //   if (condition1) {
  //     return fn1()
  //   }
  //   if (condition2) {
  //     return fn2()
  //   }
  //   return otherwise()
  // }

  // function tryCreateName() {
  //   return Tried.mapErr(ShadowVaults.Name.new(rawName), Error.CreateName)
  // }
  // function tryCreateDatum() {
  //   return Tried.mapErr(ShadowVaults.Datum.new(rawDatum), Error.CreateDatum)
  // }
  // function tryCreateDuration() {
  //   return Tried.andThen(
  //     match2(rawDurationInputMethod,
  //       DurationInputMethod_DaysHoursMinutes, () => Option.okOr(
  //         Duration.fromDaysHoursMinutes(rawDays, rawHours, rawMinutes), 
  //         Error.CreateDurationFromDaysHoursMinutes(rawDays, rawHours, rawMinutes)
  //       ),
  //       DurationInputMethod_DurationTill, () => Tried.andThen(
  //         Option.okOr(
  //           DateTime.fromNativeDate(new Date(rawDateTime)),
  //           Error.CreateDateTimeFromInput(rawDateTime),
  //         ),
  //         datetime => Option.okOr(
  //           DateTime.till(DateTime.now(), datetime),
  //           Error.InputDateTimeIsEarlierThanNow(datetime)
  //         )
  //       ),
  //       () => Err(
  //         Error.InvalidDomInput()
  //       )
  //     ),
  //     validateDurationLength
  //   )
  // }

  // function validateDurationLength(duration: Duration) {
  //   return when2(
  //     Duration.isShorterThan(duration, FIVE_MINUTES), () => 
  //       Err(Error.DurationTooShort(duration)),

  //     Duration.isLongerThan(duration, TEN_DAYS), () => 
  //       Err(Error.DurationTooLong(duration)),

  //     () => 
  //       Ok(duration)
  //   )
  // }

  // const FIVE_MINUTES = Option.unwrap(Duration.fromMinutes(5))
  // const TEN_DAYS = Option.unwrap(Duration.fromDays(10))

  // let shadowVaultCreator = $derived(Tried.map3(
  //   tryCreateName(),
  //   tryCreateDatum(),
  //   tryCreateDuration(), 
  //   (name, datum, duration) => ShadowVaults.ShadowVaultCreator.new(
  //     None(),
  //     name,
  //     datum, 
  //     duration,
  //   )
  // ))


  // async function create() {
  //   const nameOrError = ShadowVaults.Name.new(rawName)
  //   if (isErr(nameOrError)) {
  //     return
  //   }

  //   const datumOrError = ShadowVaults.Datum.new(rawDatum)
  //   if (isErr(datumOrError)) {
  //     return
  //   }

  //   if (rawDurationInputMethod === FOR_DURATION) {
  //     const durationOrNone = Duration.fromMilliseconds(
  //       rawDays * Duration.MILLISECONDS_PER_DAY
  //       +
  //       rawHours * Duration.MILLISECONDS_PER_HOUR
  //       +
  //       rawMinutes * Duration.MILLISECONDS_PER_MINUTE
  //     )
  //     if (isNone(durationOrNone)) {
  //       return
  //     }

  //     const id = Uuid.generate()
  //     const creator = ShadowVaults.ShadowVaultCreator.new(
  //       Some(id),
  //       Tried.value(nameOrError),
  //       Tried.value(datumOrError),
  //       Option.value(durationOrNone),
  //     )
  //     await ShadowVaults.CreateShadowVault.execute(
  //       discipline,
  //       creator,
  //     )
  //   }

  //   if (rawDurationInputMethod === FOR_DURATION_TILL_TIME) {
  //     const tillTime = DateTime.fromNativeDate(new Date(rawDateTime))
  //     if (isNone(tillTime)) {
  //       return
  //     }

  //     const duration
  //   }

  //   // ShadowVaults.CreateShadowVault.execute()
  //   // discipline
  // }

  // $effect(() => {
  //   console.log(rawDateTime)
  // })
</script>

<div class="w-1/2 sticky top-16 h-full container">
  <p class="text-gray-500 text-lg font-medium mb-6">Create Shadow Valut</p>

  <!-- Name -->
  <div class="mb-4">
    <label for="item-name" class="block mb-1 text-gray-500 text-sm font-medium">Name</label>
    <input
      type="text"
      bind:value={rawName}
      class="text-gray-400 text-sm mt-1 block w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
      placeholder="Enter name"
    />
  </div>
  <div class="bg-white shadow-lg rounded-lg p-6">
    

    <!-- Datum -->
    <div class="mb-4">
      <label for="item-description" class="block mb-1 text-gray-500 text-sm font-medium">
        Secret
      </label>
      <textarea
        id="item-description"
        rows="4"
        class="mt-1 block w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
        placeholder="Enter Secret"
      ></textarea>
    </div>

    <p class="block text-gray-500 text-sm font-medium text-center">Secret will be public again in</p>
    <div class="grid grid-cols-3 grid-rows-2 gap-x-4 place-items-center mx-auto w-max">
      <label for="days" class="text-sm text-gray-600">Days</label>
      <label for="hours" class="text-sm text-gray-600">Hours</label>
      <label for="minutes" class="text-sm text-gray-600">Minutes</label>
    
      <input
        id="days"
        bind:value={rawDays}
        type="number"
        min="0"
        max="14"
        class="w-24 px-2 py-1 border border-gray-300 rounded-md"
      />
      <input
        id="hours"
        bind:value={rawHours}
        type="number"
        min="0"
        max="1000"
        class="w-24 px-2 py-1 border border-gray-300 rounded-md"
      />
      <input
        id="minutes"
        bind:value={rawMinutes}
        type="number"
        min="0"
        max="1000"
        class="w-24 px-2 py-1 border border-gray-300 rounded-md"
      />
    </div>
    
    <div class="mt-6">
      <button class="w-full bg-indigo-600 text-white py-2 rounded-lg hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-opacity-50">
        Create
      </button>
    </div>    
  </div>
</div>

<style>
  .container {
    /* position: sticky;
    top: var(--spacing-16); */
    
    background-color: white;
    border-radius: var(--rounded);
    overflow: hidden;
    padding: var(--spacing-2);
    box-shadow: var(--shadow-lg);
  }

  input {
    border: 0 ;
  }
</style>
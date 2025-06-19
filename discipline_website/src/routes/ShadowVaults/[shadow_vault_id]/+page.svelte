<script lang="ts">
  import { DateTime, Duration, isSome, Option, Synchronizer } from "Discipline";
  import { ShadowVault, Name, Datum } from "DisciplineDir/Features/ShadowVaults/mod"
  // import { data } from "../../data";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props()
  
  let vault = data.shadowVault

  let datum = ShadowVault.datum(vault)
  let remainingProtectionDuration = $state(vault.protector.remainingDuration)
  let remainingProtectionDurationAsString = $derived(Duration.toString(remainingProtectionDuration))

  $effect(() => {
    const SECOND = Option.unwrap(Duration.fromSeconds(1))

    const synchronizer = Synchronizer.new(SECOND, () => {
      ShadowVault.synchronize(vault, DateTime.now())
      remainingProtectionDuration = ShadowVault.remainingProtectionDuration(vault)
    })

    if (Duration.isZero(remainingProtectionDuration)) {
      Synchronizer.ensureStopped(synchronizer)
    } else {
      Synchronizer.ensureRunning(synchronizer)
    }
  })
</script>

{#snippet moon()}
  <div></div>
{/snippet}

<div class="min-h-screen bg-gray-100 p-8 flex justify-center items-center flex-col">
  <div class="text-4xl text-center text-gray-500 mt-10 mb-20">
    <h1>Shadow Vault</h1>
  </div>

  <div class="bg-white shadow-sm rounded-lg overflow-hidden mb-3 w-[35%] p-4">
    <p class="text-gray-500 text-lg text-center mb-5 font-medium">
      {Name.value(vault.name)}
    </p>

    <div class="p-3 border border-gray-200 rounded-sm">
      <p class="text-base font-medium text-gray-500 mb-1">
        Secret
      </p>
      <p class="text-gray-500 text-sm">
        {#if !Duration.isZero(remainingProtectionDuration)}
          Secret will be public in: {remainingProtectionDurationAsString}.
        {:else if isSome(datum)}
          Secret is: {Datum.value(Option.value(datum))}.
        {:else}
          Secret is public. Reload the page to see it.
        {/if}
      </p>
    </div>
    
    <div class="p-3 border border-gray-200 rounded-b-sm mt-4">
      <p class="text-base font-medium text-gray-500 mb-1">
        <label for="new-name" class="block">
          Rename
        </label>
      </p>
      <div class="flex flex-row gap-2">
        <input
          type="text"
          placeholder="Enter new name..."
          class="flex-1 px-3 py-2 border border-gray-300 rounded-md text-sm w-full"
        />
        <button
          class="px-4 py-1.5 bg-blue-500 text-white font-semibold rounded-md text-sm hover:bg-blue-600"
        >
          Rename
        </button>
      </div>
    </div>

    <div class="p-2 border border-gray-200 mt-4 flex flex-row gap-2 justify-between items-center">
      <p class="text-sm text-gray-500">Delete this shadow vault</p>

      <button class="text-sm bg-red-500 text-white font-semibold py-1.5 px-4 rounded hover:bg-red-600">
        Delete
      </button>
    </div>
    
  </div>
</div>

<style>
  
</style>
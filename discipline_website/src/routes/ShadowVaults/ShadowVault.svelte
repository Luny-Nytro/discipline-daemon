<script lang="ts">
  import { Uuid, ShadowVaults, Duration, CountdownTimer, DateTime, Option, isSome } from "Discipline";

  let { shadowVault }: {
    shadowVault: ShadowVaults.ShadowVault
  } = $props();

  let id = Uuid.toString(shadowVault.id)
  let name = ShadowVaults.Name.value(shadowVault.name)
  let remainingDuration = $state(CountdownTimer.remainingDuration(shadowVault.protector))
  let remainingDurationAsString = $derived(Duration.toString(remainingDuration))

  function intervalExeucter(duration: Duration, fn: () => void) {
    setInterval(fn, Duration.milliseconds(duration));
  }

  $effect(() => {
    const oneSecond = Option.unwrap(Duration.fromSeconds(1))

    intervalExeucter(oneSecond, () => {
      CountdownTimer.synchronize(shadowVault.protector, DateTime.now())
      remainingDuration = CountdownTimer.remainingDuration(shadowVault.protector)
    })
  })
</script>
<!-- 
<div class="w-[60%] bg-white shadow-xl rounded-2xl border border-gray-300 p-6 hover:shadow-2xl transition-all remainingDuration-300 
                hover:text-blue-400 hover:border-blue-400 hover:cursor-pointer">
  <a href="#" class="block">
    <h2 class="text-base text-gray-600 mb-2 transition-all remainingDuration-300">
      {name}
    </h2>
    <p class="text-gray-500 hover:text-lime-400 active:text-lime-500">
      3D 4H 3M
    </p>
  </a>
</div> -->

<!-- bg-white shadow-sm rounded-lg overflow-hidden mb-3 w-full -->
<div class="container">
  <a href={`/ShadowVaults/${id}`} class="block p-4">
    <p class="name text-base font-medium text-blue-500 hover:underline cursor-pointer">
      {name}
    </p>


    <p class="duration text-gray-400 text-sm mt-1">
      {#if Duration.isZero(remainingDuration)}
        {#if isSome(shadowVault.datum)}
          Secret is: {Option.value(shadowVault.datum)}.
        {:else}
          Secret is public. Reload the page to see it.
        {/if}
      {:else}
        Secret will be public in {remainingDurationAsString}.
      {/if}
    </p>
  </a>
</div>


<style>
  .container {
    background-color: white;
    box-shadow: var(--shadow-sm);
    padding: var(--spacing-2);
    /* bg-white shadow-sm rounded-lg overflow-hidden mb-3 w-full */
  }

  .container:hover {
    cursor: pointer;
  }
  
  .container:hover .name {
    color: var(--color-blue-900);
  }

  .duration {
    font-size: var(--text-sm);
    color: var(--color-gray-600);
  }

</style>
<script lang="ts">
  import ShadowVault from "./ShadowVault.svelte";
  import { data } from "../data"
  import { Uuid, ShadowVaults } from "Discipline";

  const shadowVaults = data.shadowVaults.shadowVaults
</script>

<div class="flex flex-col items-center gap-1 container">
  {#each shadowVaults as shadowVault}
    <ShadowVault {shadowVault} />
  {/each}
</div>
<!-- <div class="flex-1 ">
</div> -->

<style>
  .container {
    display: flex;
    flex-flow: column nowrap;
    gap: var(--spacing-2);
  }
</style>
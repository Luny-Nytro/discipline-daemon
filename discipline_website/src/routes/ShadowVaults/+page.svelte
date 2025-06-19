<script lang="ts">
  import CreateShadowVault from "./CreateShadowVault.svelte";
  import ShadowVaults from "./ShadowVaults.svelte";

</script>

<div class="container">
  <h1 class="title">
    Shadow Vaults
  </h1>

  <div class="sections">
    <ShadowVaults />
    <CreateShadowVault />
  </div>
</div>

<style>
  .container {
    min-height: 100vh;
    background-color: var(--color-gray-100);
    padding: var(--spacing-8);
  }

  .title {
    font-size: var(--text-4xl);
    color: var(--color-gray-500);
    text-align: center;
    margin-top: var(--spacing-10);
    margin-bottom: var(--spacing-20);
  }

  .sections {
    display: flex;
    gap: var(--spacing-8);
    flex-flow: row;
  }

  :global(.sections > :nth-child(1)) {
    width: 50%;
  }

  :global(.sections > :nth-child(2)) {
    width: 50%;
  }
</style>
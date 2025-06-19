<script lang="ts">
  import { OperatingSystemUsername, UserAccess, type AppData } from "Discipline";
  import { data } from "./data"
  
  const userAccess = data.userAccess.enforcers.map(enforcer => ({
    username: OperatingSystemUsername.value(enforcer.username),
    isBlocked: enforcer.isBlocked,
  }))

</script>

<div class="bg-gray-100 p-8 min-h-screen">
  <h1 class="text-4xl text-center text-gray-500 mt-10 mb-20">
    Discipline
  </h1>

  <div class="flex space-x-8 max-w-[85%] mx-auto">
    <div class="bg-white shadow-sm rounded-lg overflow-hidden mb-3 w-full p-4">
      <p class="text-base font-medium text-blue-500 hover:underline cursor-pointer"><a href="/UserAccess">User Access</a></p>
      <p class="text-gray-400 text-xs mt-1">Specify when specific users are blocked from accessing the device.</p>

      {#if userAccess.length > 0}
        <p class="text-gray-500 text-sm font-medium mt-4">Summary</p>
        <ul>
          {#each userAccess as user}
            <li>
              <p class="text-gray-500 text-sm mt-1">
                {user.username}: 
                {#if user.isBlocked}
                  <span class="text-red-500">blocked</span>
                {:else}
                  <span class="text-green-500">allowed</span>
                {/if}
              </p>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    <div class="bg-white shadow-sm rounded-lg overflow-hidden mb-3 w-full p-4">
      <p class="text-base font-medium text-blue-500 hover:underline cursor-pointer"><a href="/NetworkingAccess">Networking Access</a></p>
      <p class="text-gray-400 text-xs mt-1">Specify when specific users are blocked from accessing the internet.</p>

      <p class="text-gray-500 text-sm font-medium mt-4">Summary</p>
      <ul>
        <li><p class="text-gray-500 text-sm mt-1">luny: <span class="text-red-500">blocked</span></p></li>
        <li><p class="text-gray-500 text-sm mt-1">lunynytro: <span class="text-green-500">allowed</span></p></li>
      </ul>
    </div>

    <div class="bg-white shadow-sm rounded-lg overflow-hidden mb-3 w-full p-4">
      <p class="text-base font-medium text-blue-500 hover:underline cursor-pointer"><a href="/ShadowVaults">Shadow Vaults</a></p>
      <p class="text-gray-400 text-xs mt-1">Hide notes and only access them again when according to a condition</p>

      <p class="text-gray-500 text-sm font-medium mt-4">Summary</p>
      <ul>
        <li><p class="text-gray-500 text-sm mt-1">luny: <span class="text-red-500">blocked</span></p></li>
        <li><p class="text-gray-500 text-sm mt-1">lunynytro: <span class="text-green-500">allowed</span></p></li>
      </ul>
    </div>
  </div>
</div>

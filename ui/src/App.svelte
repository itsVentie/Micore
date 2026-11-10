<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import AuthScreen from "./lib/pages/AuthScreen.svelte";
  import Dashboard from "./lib/pages/Dashboard.svelte";

  let password = $state("");
  let statusMessage = $state("");
  let isUnlocked = $state(false);

  async function handleUnlock() {
    try {
      statusMessage = await invoke("create_or_open_vault", { password });
      isUnlocked = true;
    } catch (err) {
      statusMessage = `Error: ${err}`;
    }
  }

  function handleLock() {
    isUnlocked = false;
    password = "";
  }
</script>

<svelte:head>
  <style>
    body {
      margin: 0;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      background-color: #0f172a;
      color: #f8fafc;
    }
  </style>
</svelte:head>

{#if !isUnlocked}
  <AuthScreen bind:password={password} {statusMessage} onUnlock={handleUnlock} />
{:else}
  <Dashboard onLock={handleLock} />
{/if}
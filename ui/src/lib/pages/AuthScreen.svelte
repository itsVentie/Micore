<script lang="ts">
  import styles from '../styles/Auth.module.css';
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  interface Props {
    password: string;
    statusMessage: string;
    onUnlock: () => void;
  }

  let { password = $bindable(), statusMessage, onUnlock }: Props = $props();

  let vaultName = $state("vault.mic");
  let vaultPath = $state("");
  let vaultExists = $state(false);

  async function loadVaultInfo() {
    try {
      const info: any = await invoke("get_default_vault_info");
      vaultPath = info.path;
      vaultName = info.name;
      vaultExists = info.exists;
    } catch (err) {
      console.error("Failed to load vault info:", err);
    }
  }

  async function handleBrowseVault() {
    try {
      const selected = await save({
        defaultPath: "vault.mic",
        filters: [{ name: 'Micore Vault', extensions: ['mic'] }]
      });
      if (selected && typeof selected === 'string') {
        vaultPath = selected;
        const parts = selected.split(/[/\\]/);
        vaultName = parts[parts.length - 1] || "vault.mic";

      }
    } catch (err) {
      console.error("Browse error:", err);
    }
  }

  onMount(() => {
    loadVaultInfo();
  });
</script>

<div class={styles.authContainer}>
  <div class={styles.authCard}>
    <h1>Micore</h1>
    <p class={styles.subtitle}>Personal Storage</p>

    <div class={styles.dbInfoBox}>
      <div class={styles.dbHeaderRow}>
        <span class={styles.dbLabel}>Active Database:</span>
        <span class={`{styles.badge} {vaultExists ? styles.badgeExists : styles.badgeNew}`}>
          {vaultExists ? 'Existing Vault' : 'New Vault (Will be created)'}
        </span>
      </div>
      <div class={styles.dbPathDisplay} title={vaultPath}>
        📁 {vaultPath || 'Loading path...'}
      </div>

      <div class={styles.dbConfigRow}>
        <input
          type="text"
          class={styles.smallInput}
          bind:value={vaultName}
          placeholder="vault.mic"
        />
        <button class={styles.secondaryButton} onclick={handleBrowseVault}>Browse...</button>
      </div>
    </div>

    <input
      type="password"
      class={styles.input}
      bind:value={password}
      placeholder="Enter Master Password"
      onkeydown={(e) => e.key === 'Enter' && onUnlock()}
    />

    <button class={styles.button} onclick={onUnlock}>Unlock / Create Vault</button>

    {#if statusMessage}
      <p class={styles.error}>{statusMessage}</p>
    {/if}
  </div>
</div>
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import styles from '../styles/Vault.module.css';

  let secretKey = $state("");
  let secretValue = $state("");
  let saveStatus = $state("");
  
  let searchKey = $state("");
  let retrievedValue = $state("");

  async function handleSaveSecret() {
    try {
      await invoke("save_secret", {
        keyName: secretKey,
        secretValue: secretValue,
      });
      saveStatus = "Saved successfully!";
      secretKey = "";
      secretValue = "";
    } catch (err) {
      saveStatus = `Failed: ${err}`;
    }
  }

  async function handleFetchSecret() {
    try {
      retrievedValue = await invoke("get_secret", { keyName: searchKey });
    } catch (err) {
      retrievedValue = `Error: ${err}`;
    }
  }
</script>

<div class={styles.panel}>
  <h2>Secure Vault Dashboard</h2>
  
  <div class={styles.card}>
    <h3>Add New Record</h3>
    <input class={styles.input} bind:value={secretKey} placeholder="Title (e.g., github_token)" />
    <input class={styles.input} bind:value={secretValue} placeholder="Secret Value / Password" />
    <button class={styles.button} onclick={handleSaveSecret}>Encrypt & Save</button>
    {#if saveStatus}<p class={styles.success}>{saveStatus}</p>{/if}
  </div>

  <div class={styles.card}>
    <h3>Retrieve Record</h3>
    <input class={styles.input} bind:value={searchKey} placeholder="Enter Title to decrypt" />
    <button class={styles.button} onclick={handleFetchSecret}>Decrypted Value</button>
    {#if retrievedValue}
      <div class={styles.resultBox}><code>{retrievedValue}</code></div>
    {/if}
  </div>
</div>
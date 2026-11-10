<script lang="ts">
  import styles from '../styles/Auth.module.css';

  interface Props {
    password: string;
    statusMessage: string;
    onUnlock: () => void;
  }

  let { password = $bindable(), statusMessage, onUnlock }: Props = $props();
</script>

<div class={styles.authContainer}>
  <div class={styles.authCard}>
    <h1>Micore</h1>
    <p class={styles.subtitle}>Personal Storage</p>
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
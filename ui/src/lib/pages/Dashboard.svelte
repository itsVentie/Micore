<script lang="ts">
  import styles from '../styles/Dashboard.module.css';
  import Main from './Main.svelte';
  import VaultManager from './VaultManager.svelte';
  import PasswordGenerator from './Passwords.svelte';
  import Settings from './Settings.svelte';
  import Edu from './Edu.svelte';
  import Toolchain from './Toolchain.svelte';
  import Wishlist from './Wishlist.svelte';
  import IconOverview from '../../assets/icons/table-of-contents.svg';
  import IconVault from '../../assets/icons/key-round.svg';
  import IconGenerator from '../../assets/icons/square-chart-gantt.svg';
  import IconSettings from '../../assets/icons/settings.svg';

  interface Props {
    onLock: () => void;
  }

  let { onLock }: Props = $props();
  let activeTab = $state("main");
</script>

<div class={styles.dashboard}>
  <aside class={styles.sidebar}>
    <h2>Micore</h2>
    <nav>
      <button 
        class={`${styles.navButton} ${activeTab === 'main' ? styles.active : ''}`} 
        onclick={() => activeTab = 'main'}
      >
        <img src={IconOverview} alt="" class={styles.navIcon} />
        Overview
      </button>
      <button 
        class={`${styles.navButton} ${activeTab === 'vault' ? styles.active : ''}`} 
        onclick={() => activeTab = 'vault'}
      >
        <img src={IconVault} alt="" class={styles.navIcon} />
        Vault Manager
      </button>
      <button 
        class={`${styles.navButton} ${activeTab === 'generator' ? styles.active : ''}`} 
        onclick={() => activeTab = 'generator'}
      >
        <img src={IconGenerator} alt="" class={styles.navIcon} />
        Passwords
      </button>
      <button 
        class={`${styles.navButton} ${activeTab === 'edu' ? styles.active : ''}`} 
        onclick={() => activeTab = 'edu'}
      >
        
        Edu
      </button>
      <button 
        class={`${styles.navButton} ${activeTab === 'toolchain' ? styles.active : ''}`} 
        onclick={() => activeTab = 'toolchain'}
      >

        Toolchain
      </button>
      <button 
        class={`${styles.navButton} ${activeTab === 'wishlist' ? styles.active : ''}`} 
        onclick={() => activeTab = 'wishlist'}
      >
        Wishlist
      </button>
      <button 
        class={`${styles.navButton} ${activeTab === 'settings' ? styles.active : ''}`} 
        onclick={() => activeTab = 'settings'}
      >
        <img src={IconSettings} alt="" class={styles.navIcon} />
        Settings
      </button>
    </nav>
    <button class={styles.lockBtn} onclick={onLock}>Lock Vault</button>
  </aside>

  <main class={styles.mainContent}>
    {#if activeTab === 'main'}
      <Main />
    {:else if activeTab === 'vault'}
      <VaultManager />
    {:else if activeTab === 'generator'}
      <PasswordGenerator />
    {:else if activeTab === 'edu'}
      <Edu />
    {:else if activeTab === 'toolchain'}
      <Toolchain />
    {:else if activeTab === 'wishlist'}
      <Wishlist />
    {:else}
      <Settings />
    {/if}
  </main>
</div>
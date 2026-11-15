<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import styles from '../styles/Vault.module.css';

  let userName = $state("");
  let userEmail = $state("");
  let userAvatar = $state("");
  
  let isEditing = $state(false);
  let editName = $state("");
  let editEmail = $state("");
  let editAvatar = $state("");
  let saveStatus = $state("");

  async function loadProfile() {
    try {
      const profile: any = await invoke("get_profile");
      userName = profile.name;
      userEmail = profile.email;
      userAvatar = profile.avatar;

      editName = userName;
      editEmail = userEmail;
      editAvatar = userAvatar;
    } catch (err) {
      console.error("Failed to load profile:", err);
    }
  }

  async function handleSaveProfile() {
    try {
      await invoke("save_profile", {
        name: editName,
        email: editEmail,
        avatar: editAvatar || editName.slice(0, 2).toUpperCase()
      });
      userName = editName;
      userEmail = editEmail;
      userAvatar = editAvatar;
      isEditing = false;
      saveStatus = "Profile updated successfully!";
      setTimeout(() => saveStatus = "", 3000);
    } catch (err) {
      saveStatus = `Error: ${err}`;
    }
  }

  onMount(() => {
    loadProfile();
  });
</script>

<div class={styles.panel}>
  <header class={styles.headerSection}>
    <h2>Dashboard Overview</h2>
  </header>
  
  <div class={styles.card}>
    {#if !isEditing}
      <div class={styles.profileRow}>
        <div class={styles.profileInfo}>
          <div class={styles.avatar}>
            {userAvatar || 'U'}
          </div>
          <div class={styles.profileText}>
            <h3>{userName || 'Set your name'}</h3>
            <p>{userEmail || 'Set your email'}</p>
          </div>
        </div>
        <button class={styles.button} onclick={() => isEditing = true}>Edit Profile</button>
      </div>
    {:else}
      <div class={styles.editForm}>
        <h3>Edit Profile</h3>
        <div class={styles.inputGroup}>
          <input class={styles.input} bind:value={editName} placeholder="Full Name (ФИО)" />
          <input class={styles.input} bind:value={editEmail} placeholder="Email" />
          <input class={styles.input} bind:value={editAvatar} placeholder="Avatar Initials (e.g. VR)" />
        </div>
        <div class={styles.formActions}>
          <button class={styles.button} onclick={handleSaveProfile}>Save Changes</button>
          <button class={`${styles.button} ${styles.buttonSecondary}`} onclick={() => isEditing = false}>Cancel</button>
        </div>
      </div>
    {/if}

    {#if saveStatus}
      <p class={styles.statusMessage}>{saveStatus}</p>
    {/if}
  </div>
</div>
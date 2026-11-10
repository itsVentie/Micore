<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import styles from '../styles/Vault.module.css';

  let userName = $state("");
  let userEmail = $state("");
  let userAvatar = $state("");
  let vaultStatus = $state("Active & Encrypted (AES-256-GCM)");
  
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
  <h2>Dashboard Overview</h2>
  
  <div style="background: #1e293b; padding: 20px; border-radius: 8px; border: 1px solid #334155; margin-bottom: 20px;">
    {#if !isEditing}
      <div style="display: flex; align-items: center; justify-content: space-between;">
        <div style="display: flex; align-items: center; gap: 20px;">
          <div style="width: 64px; height: 64px; border-radius: 50%; background: #3b82f6; display: flex; align-items: center; justify-content: center; font-size: 24px; font-weight: bold; color: white;">
            {userAvatar || 'U'}
          </div>
          <div>
            <h3 style="margin: 0 0 5px 0; font-size: 1.2rem; color: #f8fafc;">{userName || 'Set your name'}</h3>
            <p style="margin: 0; color: #94a3b8; font-size: 0.9rem;">{userEmail || 'Set your email'}</p>
          </div>
        </div>
        <button class={styles.button} onclick={() => isEditing = true}>Edit Profile</button>
      </div>
    {:else}
      <div>
        <h3>Edit Profile</h3>
        <input class={styles.input} bind:value={editName} placeholder="Full Name (ФИО)" />
        <input class={styles.input} bind:value={editEmail} placeholder="Email" />
        <input class={styles.input} bind:value={editAvatar} placeholder="Avatar Initials (e.g. VR)" />
        <div style="display: flex; gap: 10px; margin-top: 10px;">
          <button class={styles.button} onclick={handleSaveProfile}>Save</button>
          <button style="background: #475569;" class={styles.button} onclick={() => isEditing = false}>Cancel</button>
        </div>
      </div>
    {/if}
    {#if saveStatus}
      <p class={styles.success} style="margin-top: 10px;">{saveStatus}</p>
    {/if}
  </div>

  <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 15px;">
    <div style="background: #1e293b; padding: 20px; border-radius: 8px; border: 1px solid #334155;">
      <h4 style="margin: 0 0 10px 0; color: #94a3b8; font-size: 0.9rem;">Security</h4>
      <p style="margin: 0; color: #10b981; font-weight: bold; font-size: 1rem;">🔓 {vaultStatus}</p>
    </div>
    <div style="background: #1e293b; padding: 20px; border-radius: 8px; border: 1px solid #334155;">
      <h4 style="margin: 0 0 10px 0; color: #94a3b8; font-size: 0.9rem;">Storage Engine</h4>
      <p style="margin: 0; color: #f8fafc; font-weight: bold; font-size: 1.1rem;">redb (.mic file)</p>
    </div>
  </div>
</div>
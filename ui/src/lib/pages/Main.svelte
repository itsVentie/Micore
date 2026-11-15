<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import styles from '../styles/Vault.module.css';

  let firstName = $state("");
  let lastName = $state("");
  let userEmail = $state("");
  let avatarPath = $state("");
  let avatarUrl = $state(""); 
  
  let isEditing = $state(false);
  let editFirstName = $state("");
  let editLastName = $state("");
  let editEmail = $state("");
  let editAvatarPath = $state("");
  let editAvatarUrl = $state(""); 
  let saveStatus = $state("");

  let userInitials = $derived(
    `${firstName.charAt(0)}${lastName.charAt(0)}`.toUpperCase() || 'U'
  );

  async function loadProfile() {
    try {
      const profile: any = await invoke("get_profile");
      firstName = profile.first_name || "";
      lastName = profile.last_name || "";
      userEmail = profile.email || "";
      avatarPath = profile.avatar_path || "";
      avatarUrl = profile.avatar_url || "";

      editFirstName = firstName;
      editLastName = lastName;
      editEmail = userEmail;
      editAvatarPath = avatarPath;
      editAvatarUrl = avatarUrl;
    } catch (err) {
      console.error("Failed to load profile:", err);
    }
  }

  async function handleSelectAvatar() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Images',
          extensions: ['png', 'jpg', 'jpeg', 'webp', 'svg']
        }]
      });

      if (selected && typeof selected === 'string') {
        editAvatarPath = selected;
        try {
          const response = await fetch(`asset://localhost/${selected.replace(/\\/g, '/')}`);
          const blob = await response.blob();
          const reader = new FileReader();
          reader.onload = () => {
            editAvatarUrl = reader.result as string;
          };
          reader.readAsDataURL(blob);
        } catch {
          editAvatarUrl = "";
        }
      }
    } catch (err) {
      console.error("Failed to open file dialog:", err);
    }
  }

  async function handleSaveProfile() {
    try {
      const newAvatarUrl: any = await invoke("save_profile", {
        firstName: editFirstName,
        lastName: editLastName,
        email: editEmail,
        avatarPath: editAvatarPath
      });
      
      firstName = editFirstName;
      lastName = editLastName;
      userEmail = editEmail;
      avatarPath = editAvatarPath;
      
      await loadProfile();

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
          <div class={styles.avatarContainer}>
            {#if avatarUrl}
              <img src={avatarUrl} alt="Avatar" class={styles.avatarImage} />
            {:else}
              <div class={styles.avatarFallback}>{userInitials}</div>
            {/if}
          </div>
          <div class={styles.profileText}>
            <h3>{firstName || lastName ? `${firstName} ${lastName}`.trim() : 'Set your name'}</h3>
            <p>{userEmail || 'Set your email'}</p>
          </div>
        </div>
        <button class={styles.button} onclick={() => isEditing = true}>Edit Profile</button>
      </div>
    {:else}
      <div class={styles.editForm}>
        <h3>Edit Profile</h3>
        
        <div class={styles.avatarUploadSection}>
          <div class={styles.avatarContainer}>
            {#if editAvatarUrl}
              <img src={editAvatarUrl} alt="Avatar Preview" class={styles.avatarImage} />
            {:else}
              <div class={styles.avatarFallback}>
                {`${editFirstName.charAt(0)}${editLastName.charAt(0)}`.toUpperCase() || 'U'}
              </div>
            {/if}
          </div>
          <button class={styles.button} onclick={handleSelectAvatar}>Choose Image...</button>
        </div>

        <div class={styles.inputGroup}>
          <input class={styles.input} bind:value={editFirstName} placeholder="First Name (Имя)" />
          <input class={styles.input} bind:value={editLastName} placeholder="Last Name (Фамилия)" />
          <input class={styles.input} bind:value={editEmail} placeholder="Email" />
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
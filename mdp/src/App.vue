
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Cookies from "js-cookie";
import ListMdp from "./components/list_mdp.vue";
import { listen } from '@tauri-apps/api/event';

listen('tauri://close-requested', async () => {
  try {
    await invoke("logout_backend");
  } catch (err) {
    console.error('Erreur lors du logout backend', err);
  }
});

const password = ref("");
const NewPassword = ref("");
const loading = ref(false);
const error = ref("");
const error_new_db = ref("");
const secureMessage = ref("");
const secureError = ref("");
const isLoggedIn = ref(Cookies.get("isLoggedIn") === "false")

function handleLogout() {
  // remove persisted cookie and update state
  Cookies.remove("isLoggedIn");
  isLoggedIn.value = false;
  password.value = "";
}

async function handleLogin() {
  error.value = "";
  secureMessage.value = "";
  secureError.value = "";
  loading.value = true;
  try {
    const result = await invoke<boolean>("login_backend", {
      password: password.value,
    });

    if (!result) {
      // login failed: don't persist cookie
      Cookies.remove("isLoggedIn");
      isLoggedIn.value = false;
      error.value = "Mot de passe incorrect.";
    } else {
      // login ok: persist and update state
      Cookies.set("isLoggedIn", "true", { expires: 1 });
      isLoggedIn.value = true;
      error.value = "";
    }
  } catch (err) {
    console.error(err);
    error.value = "Erreur lors de la tentative de connexion.";
  } finally {
    loading.value = false;
  }
}

async function newDB() {
  console.log("newDB");
  console.log(NewPassword.value);

  try {
    const result_new_db = await invoke<boolean>("creat_db", {
      password: NewPassword.value,
    });

    if (!result_new_db) {
      error_new_db.value = "Probleme dans la création de la nouvelle base de données.";
    } else {
      error_new_db.value = "";
    }
  } catch (err) {
    console.error(err);
    error_new_db.value = "Erreur lors de la création de la nouvelle base de données.";
  } finally {
    loading.value = false;
  }
}

</script>

<template>
  <div id="app" class="app">
    <h1 id="titre">login Tauri</h1>

    <div v-if="!isLoggedIn" class="card">
      <p class="hint">
        L’app est mono-utilisateur : entre le mot de passe.
      </p>
      <form class="form" @submit.prevent="handleLogin">
        <label>
          Mot de passe
          <input
            v-model="password"
            type="password"
            required
            autocomplete="current-password"
          />
        </label>
        <button type="submit" :disabled="loading">
          {{ loading ? "Connexion..." : "Se connecter" }}
        </button>
      </form>
      <p v-if="error" class="error">{{ error }}</p>
    </div>

    <div v-else class="card">
      <list-mdp @logout="handleLogout" />
    </div> 

    <button @click="newDB">New DB</button>
    <input v-model="NewPassword" type="text" placeholder="Nouveau mot de passe" />
  </div>
</template>

<style>
body {
  margin: 0;
  background: #000000;
}

.app {
  display: flex;
  flex-direction: column;
  gap: 24px;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: 32px;
  font-family: system-ui, sans-serif;
  background: radial-gradient(circle at top, #0c0d0f, #585944);
  color: white;
}

#titre {
  color: white;
  font-size: 32px;
  margin-bottom: 16px;
} 

.card {
  width: min(400px, 100%);
  background: white;
  padding: 24px;
  border-radius: 12px;
  box-shadow: 0 15px 35px rgba(15, 23, 42, 0.12);
  display: flex;
  flex-direction: column;
  gap: 16px;

  background: radial-gradient(circle at 50%, #0c0d0f, #676B4A);
}

.form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

label {
  display: flex;
  flex-direction: column;
  font-weight: 600;
  gap: 4px;
}

input {
  padding: 10px 12px;
  border-radius: 8px;
  border: 1px solid #cbd5f5;
  background: #f8faff;
  font-size: 14px;
}

input:focus {
  outline: 2px solid #000000;
  outline-offset: 1px;
}

button {
  padding: 10px 14px;
  border-radius: 8px;
  border: none;
  font-weight: 600;
  cursor: pointer;
  background: #47422A;
  color: white;
  transition: background 0.2s ease;
}

button:disabled {
  background: #9aa9ff;
  cursor: progress;
}

button.secondary {
  background: #e2e8f0;
  color: #ffffff;
}

.error {
  color: #c53030;
  font-size: 14px;
}

.success {
  color: #ffffff;
  font-size: 14px;
}

.hint {
  font-size: 14px;
  color: #ffffff;
}
</style>

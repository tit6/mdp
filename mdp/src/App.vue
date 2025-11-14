<template>
  <div id="app" class="app">
    <h1>login Tauri</h1>

    <div v-if="!isAuthenticated" class="card">
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
      <list_mdp />
      <h2>Bienvenue !</h2>
      <p>Authentification active dans le backend.</p>
      <button @click="callSecureAction" :disabled="secureLoading">
        {{ secureLoading ? "Chargement..." : "Appeler une action sécurisée" }}
      </button>
      <p v-if="secureMessage" class="success">{{ secureMessage }}</p>
      <p v-if="secureError" class="error">{{ secureError }}</p>
      <button class="secondary" @click="logout">Se déconnecter</button>
    </div>
  </div>
</template>

<style>
.app {
  display: flex;
  flex-direction: column;
  gap: 24px;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: 32px;
  font-family: system-ui, sans-serif;
  background: radial-gradient(circle at top, #f0f4ff, #d7e0ff);
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
  color: #1e2a4a;
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
  outline: 2px solid #4c6ef5;
  outline-offset: 1px;
}

button {
  padding: 10px 14px;
  border-radius: 8px;
  border: none;
  font-weight: 600;
  cursor: pointer;
  background: #4c6ef5;
  color: white;
  transition: background 0.2s ease;
}

button:disabled {
  background: #9aa9ff;
  cursor: progress;
}

button.secondary {
  background: #e2e8f0;
  color: #1e2a4a;
}

.error {
  color: #c53030;
  font-size: 14px;
}

.success {
  color: #2f855a;
  font-size: 14px;
}

.hint {
  font-size: 14px;
  color: #425b8a;
}
</style>

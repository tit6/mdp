<script setup lang="ts">
import { ref, onMounted, defineEmits } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import ajout from "./ajout.vue";

const emit = defineEmits<{ (e: 'logout'): void }>();

const test = ref('') // variable réactive
const secureLoading = ref(false);
const secureMessage = ref("");
const secureError = ref("");
const compteur = ref(false)

function vue_ajouter() {
  compteur.value = !compteur.value;
  call_mdp();
}

function initPage() {
  test.value = 'Vue est prêt !'
  console.log('Vue chargée !')
}

onMounted(() => {
  initPage()
  call_mdp()
})

async function call_mdp() {
 secureLoading.value = true;
  try {
    const message = await invoke<string>("secure_action");
    secureMessage.value = JSON.parse(message);
    console.log(secureMessage.value);
  } catch (err) {
    console.error(err);
    secureError.value = err instanceof Error ? err.message : "Action refusée par le backend.";
  } finally {
    secureLoading.value = false;
  }
}

async function logout() {
  // appelle le backend pour invalider la session
  try {
    await invoke("logout_backend");
  } catch (err) {
    console.error('Erreur lors du logout backend', err);
  }
  // informe le parent (App.vue) de la déconnexion pour mettre à jour l'UI / cookie
  emit('logout');
  secureMessage.value = "";
  secureError.value = "";
}
</script>


<template>
  <div class="main">
    <h1>{{ test }}</h1>

    <h2>Bienvenue !</h2>
    <p>Authentification active dans le backend.</p>

    <button @click="vue_ajouter">ajouter</button>
    <div v-if="compteur">
      <ajout @close="vue_ajouter"/>
    </div>

    <button @click="call_mdp" :disabled="secureLoading">
      {{ secureLoading ? "Chargement..." : "Appeler les mdp" }}
    </button>
    <p v-if="secureMessage" class="success">
      <div v-for="(item, i) in secureMessage" :key="i" class="box">
          <p><strong>id :</strong> {{ item.id }}</p>
          <p>{{ item.url }}</p>
          <p><strong>Mot de passe :</strong> {{ item.mdp }}</p>
      </div>
    </p>
    <p v-if="secureError" class="error">{{ secureError }}</p>
    <button @click="logout">Se déconnecter</button>
  </div>
</template>


<style scoped>
.box {
  border: 1px solid #ccc;
  padding: 10px;
  margin-bottom: 10px;
  text-align: center;
  width: 300px;
}

.main {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
  text-align: center;
}


</style>

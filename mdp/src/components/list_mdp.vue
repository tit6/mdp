<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";

const test = ref('') // ← variable réactive
const secureLoading = ref(false);
const secureMessage = ref("");
const secureError = ref("");

function initPage() {
  test.value = 'Vue est prêt !'  // ← on met à jour la valeur
  console.log('Vue chargée !')
}

onMounted(() => {
    initPage()
    callSecureAction()
})

async function callSecureAction() {

  secureError.value = "";
  secureMessage.value = "";
  secureLoading.value = true;

  try {
    const message = await invoke<string>("secure_action");
    secureMessage.value = `Réponse du backend: ${message}`;
    console.log(secureMessage.value);
  } catch (err) {
    console.error(err);
    secureError.value =
      err instanceof Error ? err.message : "Action refusée par le backend.";
  } finally {
    secureLoading.value = false;
  }
}
</script>


<template>
  <div>
    <h1>{{ test }}</h1>
    <h1>{{ secureMessage }}</h1>
  </div>
</template>

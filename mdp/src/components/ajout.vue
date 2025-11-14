<script setup lang="ts">
    import {ref} from 'vue';
    import { invoke } from "@tauri-apps/api/core";
    import { emit } from '@tauri-apps/api/event';

    const emit = defineEmits<{ (e: 'close'): void }>();


    const loading = ref(false);
    const result = ref<string | null>(null);
    const error_save = ref("");
    const url = ref("");
    const mdp = ref("");

    async function enregistrer() {
        loading.value = true;

        try{
            const result = await invoke<boolean>("insert", {
                site: url.value,
                mdp: mdp.value,
            });

            console.log(result);

            if (!result) {
                console.error("Erreur lors de l'insertion des données.");
                error_save.value = "Erreur lors de l'insertion des données.";
            } else {
                console.log("Données insérées avec succès.");
            }
            loading.value = false; 
            console.log("close");
            emit("close");

        } catch (err) {
            console.error(err);
            error_save.value = "Erreur lors de l'insertion des données.";
            loading.value = false; 
            return error_save.value; 
        }
        
    }

</script>

<template>
  <div class="overlay">
    <div class="content">
        <div v-if="error_save">
            {{ error_save }}
        </div>
        <form class="form" @submit.prevent="enregistrer">
            <input v-model="url" type="text" placeholder="site" />
            <input v-model="mdp" type="text" placeholder="mots de passe" />
            <button type="submit" :disabled="loading">
                {{ loading ? "Connexion..." : "Enregister" }}
            </button>
        </form>
      <button @click="$emit('close')">Fermer</button>
    </div>
  </div>
</template>


<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.6);
  display: flex;
  justify-content: center;
  align-items: center;
  backdrop-filter: blur(3px);
}

.content {
  padding: 30px;
  border-radius: 10px;
  width: 90%;
  max-width: 600px;
  background: radial-gradient(circle at 50%, #0c0d0f, #676B4A);
  color: white;
  text-align: center;

}

.form {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 20px;
}

input {
  padding: 10px 12px;
  border-radius: 8px;
  border: none;
  color: white;
  background: radial-gradient(circle at left, #0c0d0f, #676B4A);

}
</style>

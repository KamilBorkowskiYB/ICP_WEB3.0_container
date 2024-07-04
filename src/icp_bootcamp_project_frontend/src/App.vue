<script setup>
import { ref } from 'vue';
import { icp_bootcamp_project_backend } from 'declarations/icp_bootcamp_project_backend/index';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  await icp_bootcamp_project_backend.greet(name).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main class="bg-teal-900">
    <div class="bg-teal-950">
      <img src="/logo2.svg" alt="DFINITY logo" />
    </div>
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" class="border-2 border-blue-600 p-4"/>
      <button type="submit" class="bg-blue-600 rounded text-white p-4">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
</template>

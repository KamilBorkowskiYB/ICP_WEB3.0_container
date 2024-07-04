<script setup>
import { ref } from 'vue';
import { icp_bootcamp_project_backend } from 'declarations/icp_bootcamp_project_backend/index';
let greeting = ref('');
const mask =ref(1); //for starting quiz
const question_num = ref(1);

function start(){
  mask.value --;
}
function next_question(){
  question_num.value ++;
}
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
  <main class="bg-teal-800">
    <div class="bg-teal-900">   <!-- Top bar -->
      <img src="/logo2.svg" alt="DFINITY logo" />
    </div>
    <div class="grid grid-cols-3"> <!-- Main screen -->
      <div class="bg-teal-700"> <!-- Left side -->
            nwm, moze jakis opis
      </div>

      <div v-if="mask > 0" class="bg-teal-800"> <!-- Start menu -->
        <br />
        <br />
        <br />
        <br />
        <div class="flex justify-center">
          <button @click="start" class="bg-blue-600 rounded text-white p-20">Start</button>
        </div>
        <br />
        <br />
        <br />
        <br />
      </div>
      <div v-if="mask <= 0" class="bg-teal-800">   <!-- Middle -->
        <br />
        <div class="text-center text-3xl">
              Question {{ question_num }}
        </div>
        <br />
        <br />
        <div class="text-center">
              Equation
        </div>
        <br />
        <br />
        <form action="#" @submit="handleSubmit" class="flex flex-col items-stretch">
          <div class="text-center">
              Enter your answer
          </div>
          <input id="name" alt="Name" type="text" class="border-2 border-blue-600 p-4 mx-20 text-center"/>
          <button @click="next_question" type="submit" class="bg-blue-600 rounded text-white p-4">Answer</button>
        </form>
        <section id="greeting">{{ greeting }}</section>
      </div>

      <div class="bg-teal-700">   <!-- Right side -->
            leaderboard
      </div>
    </div>
  </main>
</template>

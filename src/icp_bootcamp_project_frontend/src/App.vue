<script>
import { ref } from 'vue';
import { icp_bootcamp_project_backend } from 'declarations/icp_bootcamp_project_backend/index';

export default {
  data() {
    return {
      greeting: ref(''),
      mask: ref(1),
      question_num: ref(0),
      equation: ref(''),
      answer: ref(null)
    }
  },
  methods: {
    start() {
      this.mask--;
      this.next_question();
    },
    isPrime(num) {
      if (num <= 1) return false;
      for (let i = 2; i < num; i++) {
        if (num % i === 0) return false;
      }
      return true;
    },
    getRandomInt(min, max) {
      return Math.floor(Math.random() * (max - min + 1)) + min;
    },
    next_question() {
      const operation = this.getRandomInt(1, 4);
      let x, y, operation_symbol;
      this.question_num++;
      if (operation === 1) {
        x = this.getRandomInt(1, 256);
        y = this.getRandomInt(0, 256);
        this.answer = x + y;
        operation_symbol = '+';
      } else if (operation === 2) {
        x = this.getRandomInt(1, 256);
        y = this.getRandomInt(0, 256);
        this.answer = x - y;
        operation_symbol = '-';
      } else if (operation === 3) {
        x = this.getRandomInt(1, 16);
        y = this.getRandomInt(0, 16);
        this.answer = x * y;
        operation_symbol = '*';
      } else {
        x = this.getRandomInt(12, 150);
        while (this.isPrime(x)) {
          x = this.getRandomInt(12, 150);
        }
        y = this.getRandomInt(2, 12);
        while (x % y !== 0) {
          y = this.getRandomInt(2, 12);
        }
        this.answer = x / y;
        operation_symbol = '/';
      }

      this.equation = `${x} ${operation_symbol} ${y}`;
    },
    async handleSubmit(e) {
      e.preventDefault();
      const target = e.target;
      const name = target.querySelector('#name').value;
      await icp_bootcamp_project_backend.greet(name).then((response) => {
        this.greeting = response;
      });
    }
  }
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
              {{ equation }}
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

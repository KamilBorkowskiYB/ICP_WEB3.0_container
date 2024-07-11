<script>
import { ref } from 'vue';
import { icp_bootcamp_project_backend } from 'declarations/icp_bootcamp_project_backend/index';


export default {
  data() {
    return {
      mask: ref(1), // 0 = game , 1 = start, 2 = end
      question_num: ref(0),
      equation: ref(''),
      answer: ref(null),
      score: ref(0),
      submit_ans: ref(null),
      time: ref(10),
      timer: null,
      highscore: ref(0),
      game_time: ref(0),
      game_timer: null,
      time_spent: ref(''),
      soundON: ref(1),
    }
  },
  methods: {
    start() {
      this.time = 60;//60 normaly , 10 for end screen debug
      this.game_time = 0;
      this.question_num = 0;
      this.score = 0;
      this.mask = 0;
      this.next_question();

      this.$nextTick(() => {
        this.$refs.ans.focus();
      });
      this.timer = setInterval(this.countDown, 1000);
      this.game_timer = setInterval(this.countUp, 1000);
    },
    handleSubmit(e) {
      e.preventDefault();
      const target = e.target;
      this.submit_ans = target.querySelector('#ans').value;
      target.querySelector('#ans').value = '';

      if(this.submit_ans == this.answer){
        this.score++;
        this.time += 2;
        if(this.soundON == 1){
        let audio = new Audio("correct.mp3");
        audio.play();          
        }
      } else{
        if(this.soundON == 1){
        let audio = new Audio("wrong.mp3");
        audio.play();          
        }
      }

      this.next_question();
    },
    countDown(){
      if (this.time > 1) {
        this.time--;
      } else {
        clearInterval(this.timer);
        this.mask = 2;
        if(this.score > this.highscore){
          this.highscore = this.score;
        }
      }
    },
    countUp(){
      this.game_time++;
      if(this.mask == 2){
        clearInterval(this.game_timer);
        const minutes = Math.floor(this.game_time / 60);
        const seconds = this.game_time % 60;
        this.time_spent = `${minutes}:${seconds.toString().padStart(2, '0')}`;
      } 
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
    send_score(e){
      e.preventDefault();
    },
    toggleAudio(){
      if(this.soundON == 1){
        this.soundON = 0;
        const audio_button = document.getElementById("audio_button");
        audio_button.innerHTML = '<img src="/audioOFF_btn.jpg" width="50" height="50" />';
      } else{
        this.soundON = 1;
        const audio_button = document.getElementById("audio_button");
        audio_button.innerHTML = '<img src="/audioON_btn.jpg" width="50" height="50" />';
      }
    },
  },
  mounted(){
    const audio_button = document.getElementById("audio_button");
    audio_button.innerHTML = '<img src="/audioON_btn.jpg" width="50" height="50" />';
  }
}
</script>

<template>
  <main class="bg-slate-800 h-screen text-white">
    <div class="bg-slate-800 h-20 flex items-center">   <!-- Top bar -->
      <div class="pl-10">
        <p class="text-4xl text-orange-600">Math Quiz Blitz</p>
        <p class="text-xs pl-4">by Kamil Borkowski and Krzysztof Chrapowicz</p>     
      </div>
      <button @click="toggleAudio" id="audio_button" class="ml-auto pr-10"></button>
    </div>

    <div class="bg-slate-600 h-16"></div> <!-- Top span panel -->

    <div class="grid grid-cols-3 h-4/6"> <!-- Main screen -->
      <div class="bg-slate-600 flex"> <!-- Left side -->
        <div v-if="mask == 1" ><!-- Left side start -->
          <br>
          <br>
          <h1 class="text-center text-4xl">How to play</h1>
          <br>
          <div class="text-center px-20 pb-20">
            <p>You will have one minute to answer as many math questions as you can.</p>
            <p class="py-4">Each good answer awards you one score point and bonus two seconds.</p>
            <p>But each wrong answer will cost you two seconds.</p>
            <br>
            <p>Click Start when ready!</p>
          </div>
        </div>
        <div v-if="mask == 0" class="flex items-center justify-center grow"><!-- Left side game -->
          <div class="text-center text-4xl">
            <p>Highscore: {{ highscore }}</p>
            <p class="py-20">Score: {{score}}</p>
            <p>Time left: {{ time }} </p>         
          </div>
        </div>
        <div v-if="mask == 2" class="flex items-center justify-center grow"><!-- Left side end screen -->
          <div class="text-center text-4xl">
            <p>Highscore: {{ highscore }}</p>
            <p class="py-10">Score: {{score}}</p>
            <p>Questions asked: {{ question_num }} </p>  
            <p class="py-10">Correct answers: {{ score }}</p>  
            <p>Wrong answers: {{ question_num - score }}</p>
            <p class="pt-10">Time {{ time_spent }}</p>
          </div>
        </div>
      </div>

      <div class="bg-slate-700 h-full drop-shadow-2xl"><!-- Middle -->
        <div v-if="mask == 1" class="h-full flex justify-center items-center" > <!-- Start menu -->
          <div class="flex justify-center">
            <button @click="start" class="bg-orange-600 rounded text-white p-20 text-5xl">Start</button>
          </div>
        </div>
        <div v-if="mask == 0" class="h-full">   <!-- Game -->
          <br />
          <div class="text-center text-4xl">
             Question {{ question_num }}
          </div>
          <br />
          <br />
          <br />
          <div class="text-center text-7xl">
                {{ equation }}
          </div>
          <br />
          <br />
          <form action="#" @submit="handleSubmit" class="flex flex-col items-stretch">
            <div class="text-center text-lg">
                Enter your answer
            </div>
            <input id="ans" ref="ans" autocomplete="off" type="text" class="border-2 border-orange-600 p-4 mx-20 text-center text-black"/>
            <button type="submit" class="bg-orange-600 rounded text-white p-4 mx-20 mt-4">Answer</button>
          </form>
        </div>
        <div v-if="mask == 2" class="h-full"> <!-- End screen -->
          <br />
          <p class="text-center text-4xl">Time's up!</p>
          <br />
          <br />
          <p class="text-center text-4xl">Share your score on the leaderboard.</p>
          <br />
          <form action="#" @submit="send_score" class="flex flex-col items-stretch">
            <div class="text-center text-lg">
                Enter your name
            </div>
            <input id="name" ref="name" autocomplete="off" type="text" class="border-2 border-orange-600 p-4 mx-20 text-center text-black"/>
            <button type="submit" class="bg-orange-600 rounded text-white p-4 mx-20">Send</button>
            <button @click="start" class="bg-orange-600 rounded text-white p-4 mx-20">Restart</button>
          </form>
        </div>
      </div>
      
      <div class="bg-slate-600">   <!-- Right side -->
            leaderboard
      </div>
    </div>

    <div class="bg-slate-600 h-16"></div> <!-- Bottom span panel -->

    <div class="bg-slate-800 h-20 flex items-center">   <!-- Bottom -->
      <div class="text-sm text-gray-400 pl-10 grid grid-cols-2 gap-10">
        <div>
          <p class="text-base">Project repository</p>
          <a href="https://github.com/KamilBorkowskiYB/ICP_WEB3.0_container">https://github.com/KamilBorkowskiYB/ICP_WEB3.0_container</a>
          <br>
        </div>
        <div>
          <p class="text-base">Authors' repositories</p>
          <a href="https://github.com/KamilBorkowskiYB">https://github.com/KamilBorkowskiYB</a>
          <br>
          <a href="https://github.com/Gemmon">https://github.com/Gemmon</a>
        </div>
      </div>
    </div>
  </main>
</template>

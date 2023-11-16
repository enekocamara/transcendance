<style scoped>

.container{
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  height: 100%;
}
.screen{
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  margin-top: 100px;
  width: 1000px;
  height: 500px;
  background-color: gray;
  position: relative;
}

.matchmakingMessage{
  position: relative;
  background-color: aqua;
  margin-top: 20px;
  width: 200px;
  height: 20px;
  color: black;
  font-size: medium;
}

.btn{
  position: relative;
  background-color: white;
}

</style>

<template>
  <div class="container">
    <div class="screen">
      <button @click="submitForm" class="btn">START</button>
      <h1 v-if=matchmakingMessage class="matchmakingMessage"> {{matchmakingMessage }}</h1>
    </div>
  </div>
</template>


<script>
  import axios from 'axios';
  import { API_URL} from '@/vue.config';

  export default {
    data(){
      return {
      matchmakingMessage: '',
      };
    },
    methods: {
    submitForm() {
      const data = {
        username: this.username,

      };
      const apiUrl = `${API_URL}/matchmaking/`;
      console.log('URL: ', apiUrl)
      console.log('Submitted:', this.username, this.password1, this.password2);
      this.matchmakingMessage = 'requesting access...'
      axios.post(apiUrl, data)
        .then(response => {
          this.matchmakingMessage = 'Port received, connecting to servers...';
          const serverIP = 'http://10.13.8.2';
          const serverPort = 4915;
          const socket = new WebSocket(`ws://${serverIP}:${serverPort}`);
          setTimeout(() => {
            if (socket.readyState === WebSocket.OPEN) {
              this.matchmakingMessage ='WebSocket connection is open.';
            } else {
              this.matchmakingMessage ='WebSocket connection failed or timed out.';
              socket.close(); // Close the connection if needed
            }
          }, 5000);
          
          if (socket.readyState != socket.OPEN)
            this.matchmakingMessage = 'Failed to connect to servers. Try again'
          else{
            this.matchmakingMessage = 'Connected to servers.'
            /*while (true){
            }*/
          }
          console.log(response.data);
        })
        .catch(error => {
          if (error.response && error.response.data && error.response.data.errors){
            const errorHtml = error.response.data.errors;
            this.errorHtml = errorHtml;
            //this.message = error.message;
          } else {
            //this.message = 'Unspecified error ocurrend.Try again.'
          }
         // console.error(message);
        });
    },
  },
};
</script>
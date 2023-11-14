<style scoped>
  /* Add any custom styles for your alert */
  .registerForm {
    text-align: center;
    margin: auto;
    width: 60%;
    padding: 10px;
  }

  .alert {
    margin: auto;
    width: 60%;
    padding: 10px;
    border: 1px solid #d9534f;
    border-radius: 4px;
    color: rgb(69, 55, 55);
    background-color: #f2dede;
  }
  .alertSuccess {
    margin: auto;
    width: 60%;
    padding: 10px;
    border: 1px solid #4fd976;
    border-radius: 4px;
    color: rgb(69, 55, 55);
    background-color: #ffffff;
  }
</style>

<template>
    <div class="registerForm" style="padding-right: 350px;padding-left: 350px;">
      <h1>Log In</h1><br><br>
      <form @submit.prevent="submitForm">
        <input class="form-control" v-model="username" type="text" placeholder="Username"  name="username" required /><br>
        <input class="form-control" v-model="password" type="password" placeholder="Password" name="password" required /><br>
        <button type="submit" class="btn btn-warning">Register</button>
      </form>
    </div>

    <div v-if="errorMessage" class="alert"> {{ errorMessage }}</div>
    <div v-if="message" class="alertSuccess"> {{ message }}</div>
</template>

<script>
  import axios from 'axios';
  import { API_URL} from '@/vue.config'; 
  export default {
    data() {
      return {
        errorMessage: '',
        message: '',
        username: '',
        password: '',
    };
  },
  methods: {
    submitForm() {
      const data = {
        username: this.username,
        password: this.password,
      };
      const apiUrl = `${API_URL}/members/login/`;
      axios.post(apiUrl, data)
        .then(response => {
          this.message = 'Login successful';
          console.log(response.data);
        })
        .catch(error => {
           this.errorMessage = 'Login failed';
        });
    },
  },
};
</script>
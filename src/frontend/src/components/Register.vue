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
</style>

<template>
    <div class="registerForm">
      <h1>Register</h1><br><br>
      <form @submit.prevent="submitForm">
        <input class="form-control" v-model="username" type="text" placeholder="Username"  name="username" required /><br>
        <input class="form-control" v-model="password1" type="password" placeholder="Password" name="password1" required /><br>
        <input class="form-control" v-model="password2" type="password" placeholder="Repeat password" name="password2" required/><br><br>
        <button type="submit" class="btn btn-warning">Register</button>
      </form>
    </div>
    <div v-if="errorHtml" v-html="errorHtml" class="alert"></div>

    <div class="alert alert-success d-flex align-items-center d-none" id="registrationGood" role="alert">
        <div>
          <i class="bi bi-check-circle-fill"></i> You have been successfully registered! Redirecting...
        </div>
    </div>
    <div class="alert alert-danger d-flex align-items-center d-none" id="registrationBad" role="alert"></div>
</template>

<script>
  import axios from 'axios';
  import { API_URL} from '@/vue.config'; 
  export default {
    data() {
      return {
        errorHtml: '',
        errorMessage: '',
        username: '',
        password1: '',
        password2: '',
    };
  },
  methods: {
    submitForm() {
      const data = {
                username: this.username,
                password1: this.password1,
                password2: this.password2,
      };
      const apiUrl = `${API_URL}/members/register/`;
      console.log('URL: ', apiUrl)
      console.log('Submitted:', this.username, this.password1, this.password2);
      axios.post(apiUrl, data)
        .then(response => {
          this.message = 'Registration successful';
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
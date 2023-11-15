<style scoped>
  /* Add any custom styles for your alert */
 /* .registerForm {
    text-align: center;
    margin: auto;
    width: 60%;
    padding: 10px;
    background-color:rgba(128, 255, 212, 0.2);
    border-radius: 20px;
  }
*/

.registerForm {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  margin: auto;
  margin-top: 20px;
  width: 60%;
  padding: 10px;
  border-radius: 20px;
}

.title{
  font-size: 40px;
  text-align: left;
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

.form-control{
  background-color: rgb(230, 238, 238);
  background: linear-gradient(to bottom,rgb(230, 238, 238), rgba(197, 204, 211,0.3));
  border: none;
  color:black;
  margin-top: 5px;
}

.form-control:active{
  border:none;
  outline:none;
}

.form-control:focus {
  border-color: white;
  outline: solid 1px rgb(137, 137, 137);
  box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.075), 0 0 8px rgba(157, 157, 157, 0.6);
}

.entry {
  font-size: 20px;
  margin-top: 20px;
  color: rgb(80,80,80)
}

.btn{
  margin-top: 20px;
  background-color:/* rgb(209, 215, 220)*/rgb(117, 225, 193);
  border:none;
  outline:none;
}

.btn:hover{
  outline:none;
  border:none;
}

.btn:active{
  background-color: rgb(117, 225, 193);
  outline:none;
  border:none;
}

.btn:focus{
  outline: solid 1px rgb(137, 137, 137);
}

</style>

<template>
    <div class="registerForm">
      <form @submit.prevent="submitForm">
        <div class="title">Sign Up</div>
        <h1 class="entry">Username</h1>
        <input class="form-control" v-model="username" type="text"  name="username" required />
        <h1 class="entry">Password</h1>
        <input class="form-control" v-model="password1" type="password"  name="password1" required />
        <h1 class="entry">Repeat Password</h1>
        <input class="form-control" v-model="password2" type="password" name="password2" required/>
        <button type="submit" class="btn">Sign Up</button>
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
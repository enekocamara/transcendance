<script setup>
import { RouterLink, RouterView } from 'vue-router';
import HelloWorld from './components/HelloWorld.vue';
import 'bootstrap-icons/font/bootstrap-icons.css';

</script>

<template>
  <nav class="navbar navbar-expand-lg">
    <div class="container-fluid">
      <div class="d-flex align-items-center">
        <router-link to="/" class="navbar-brand">PONGWARS</router-link>
      </div>

      <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>

      <div class="collapse navbar-collapse" id="navbarSupportedContent">
        <ul class="navbar-nav mx-auto">
          <li class="nav-item">
            <router-link to="/game" class="btn">Play</router-link>
          </li>
        </ul>

        <form class="d-flex">
          <router-link to="/login" class="btn btn-outline-success btn-border-color">Log In</router-link>
          <router-link to="/register" class="btn btn-outline-success btn-border-color">Register</router-link>
        </form>
      </div>
    </div>
  </nav>

  <RouterView />
</template>

<style scoped>

.navbar{
  /*background-color: #69e1bd;*/
  background: linear-gradient(to bottom, #69e1bd, rgba(106, 226, 190, 0.7));
}
.navbar-collapse.show {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  /* Change background color */
  color: #fff; /* Change text color */
  padding: 10px; /* Adjust padding */
  /* Add any other styles as needed */
}

.navbar-brand{
  margin-left: 15px;
}

.btn{
  margin-right: 20px;
  color: black;
  background-color: white;
  /*border: 1px solid black;*/
  border:none;
  outline:none;
}

.btn:hover{
  background-color: rgb(236, 237, 243);
  color: black;
  box-shadow: 1px 1px 1px rgb(216, 216, 216);
  transition: background-color 0.5s ease, box-shadow 0.3s ease;
}

header {
  line-height: 1.5;
  max-height: 100vh;
}

@media (min-width: 2000px) {/*1024*/
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>

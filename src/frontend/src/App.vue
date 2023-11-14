<script setup>
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
import 'bootstrap-icons/font/bootstrap-icons.css';
</script>

<template>
  <nav class="navbar navbar-expand-lg" style="background-color: #69e1bd">
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
            <a class="nav-link" href="#">Link</a>
          </li>
          <li class="nav-item">
            <a class="nav-link disabled" aria-disabled="true">Disabled</a>
          </li>
        </ul>

        <form class="d-flex">
          <router-link to="/login" class="btn btn-outline-success btn-border-color" style="margin-right: 5px;">Log In</router-link>
          <router-link to="/register" class="btn btn-outline-success btn-border-color">Register</router-link>
        </form>
      </div>
    </div>
  </nav>


  <RouterView />
</template>

<style scoped>
.btn-outline-success {
  background-color: white;
}

.btn-border-color {
  background-color: white;
}

header {
  line-height: 1.5;
  max-height: 100vh;
}

@media (min-width: 1024px) {
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

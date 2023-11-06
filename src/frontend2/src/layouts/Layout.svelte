<!-- Layout.svelte -->
<script>
    import { authGuard } from '../components/authGuard.js'; // Import your authGuard
  </script>
  
  <main>
    <!-- Your layout content -->
    <slot></slot>
  </main>
  
  <script>
    export let page;
  
    // Define route and attach the authGuard
    $: page.beforeEnter = authGuard;
  </script>
  
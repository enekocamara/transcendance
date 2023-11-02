<script>
    //import getPublicIp from '../../components/getPublicIp.js';
    //import {config} from 'dotenv';
    import Cookies from 'js-cookie';
    import { goto } from '$app/navigation';
    //import { navigate } from 'svelte-routing';
    
    let username = "";
    let password = "";
    let message  = "";

    async function handleSubmit() {
        const backendUrl = import.meta.env.VITE_API_URL;
        console.log(backendUrl + '/login');
        const response = await fetch(backendUrl + '/login', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'Access-Control-Allow-Origin': '*'
            },
            body: JSON.stringify({ username, password })
        });
        const data = await response.json();
        console.log(data);
        if (response.ok)
        {
          const token = data.token;
          Cookies.set('token', token, {
            expires: 1,
            httpOnly: true,
          });
          //navigate('/game', {replace: true});
          goto('/game');
        }
        message = data.message;
    }
</script>

<form on:submit|preventDefault={handleSubmit}>
    <label for="username">Username:</label>
    <input type="text" id="username" bind:value={username} required>
    
    <label for="password">Password:</label>
    <input type="password" id="password" bind:value={password} required>
    
    <button type="submit">Submit</button>
</form>

{#if message}
  <p>{message}</p>
{/if}
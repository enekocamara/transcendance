<script>
    import getPublicIp from '../../components/getPublicIp.js';
    let username = "";
    let password = "";
    let message  = "";

    async function handleSubmit() {
        const publicIp = await getPublicIp();
        const response = await fetch('http://' + publicIp + ':3000/register', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password })
        });
        if (response.ok) {
          const data = await response.json();
          message = data.message; // Set the message from the response
        } else {
          message = 'Error: Registration failed'; // Set an error message
        }
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
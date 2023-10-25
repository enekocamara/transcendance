<script>
    let username = "";
    let password = "";

    async function handleSubmit() {
        const response = await fetch('http://localhost:3000/register', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password })
        });
        //const data = await response.json();
        //console.log(data); // Process response here
    }
</script>

<form on:submit|preventDefault={handleSubmit}>
    <label for="username">Username:</label>
    <input type="text" id="username" bind:value={username} required>
    
    <label for="password">Password:</label>
    <input type="password" id="password" bind:value={password} required>
    
    <button type="submit">Submit</button>
</form>

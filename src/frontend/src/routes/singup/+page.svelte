<script>
    let username = "";
    let password = "";

    function handleSubmit() {
        // Logic to handle submission, such as sending to an API endpoint
    }
</script>

<form on:submit|preventDefault={handleSubmit}>
    <label for="username">Username:</label>
    <input type="text" id="username" bind:value={username} required>
    
    <label for="password">Password:</label>
    <input type="password" id="password" bind:value={password} required>
    
    <button type="submit">Submit</button>
</form>

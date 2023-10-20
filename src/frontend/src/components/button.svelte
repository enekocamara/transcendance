<script>
		import { onMount } from 'svelte';
		async function fetchData() {
			console.log('prueba');
			try {
				const response = await fetch('http://localhost:3000/spheniscidae', {
					method: 'GET',
					headers: {
						'Content-Type': 'application/json',
					},
					// Include additional request options if necessary
				});
				if (!response.ok) {
					throw new Error('Network response was not ok');
				}
				const data = await response.json();
				console.log(data); // Process your data as needed
			} catch (error) {
				console.error('There was a problem fetching the data: ', error);
			}
		}
</script>
<button on:click={fetchData}>Fetch Data</button>
<script>
		import { onMount } from 'svelte';
		import getPublicIp from './getPublicIp.js';

		async function fetchData() {
			console.log('prueba');
			try {
				const publicIp = await getPublicIp();
				const response = await fetch('http://${publicIp}:3000/spheniscidae', {
					method: 'GET',
					headers: {
						'Content-Type': 'application/json',
						'Access-Control-Allow-Origin': '*'
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
<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let files: string[] = [];
	onMount(async () => {
		files = (await invoke('get_files', { path: '/Users/nichwch/notes' })) as string[];
		console.log({ files });
	});
</script>

<h1>Welcome to your reading list!</h1>
<div>This was generated based off the content of the following files:</div>
{#each files as file}
	<div>{file}</div>
{/each}
<div>The following topics were identified, which were</div>

<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let message = '';
</script>

<h1>Welcome to your reading list!</h1>
<button
	on:click={async () => {
		message = await invoke('greet', { name: 'nick' });
	}}>press me</button
>
<div>{message}</div>
<div>This was generated based off the content of the following files:</div>
<div>The following topics were identified, which were</div>

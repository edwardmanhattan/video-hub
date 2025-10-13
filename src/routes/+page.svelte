<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let config = $state('');
	let videoUrl = $state('');
	let videoPath = $state('');
	let status = $state('');
	let isDownloading = $state(false);

	async function loadConfig() {
		try {
			config = await invoke('read_config');
			status = 'Config loaded';
		} catch (e) {
			status = `Error: ${e}`;
		}
	}

	async function downloadVideo() {
		isDownloading = true;
		try {
			await invoke('download_video', { url: videoUrl, path: videoPath });
			status = 'Video downloaded';
		} catch (e) {
			status = `Error: ${e}`;
		} finally {
			isDownloading = false;
		}
	}

	async function playVideo() {
		try {
			await invoke('play_video', { path: videoPath });
			status = 'Video playing';
		} catch (e) {
			status = `Error: ${e}`;
		}
	}

	async function playAllVideos() {
		try {
			await invoke('play_all_videos');
			status = 'All videos playing';
		} catch (e) {
			status = `Error: ${e}`;
		}
	}
</script>

<div class="p-8">
	<h1>Video Hub</h1>

	<br />

	<button class="btn btn-accent" onclick={loadConfig}>Load Config</button>

	<br />
	<br />

	<p>Config: {config}</p>

	<br />

	<div class="flex items-center gap-2">
		<input bind:value={videoUrl} class="input" placeholder="Video URL" />
		<input bind:value={videoPath} class="input" placeholder="Save Path" />
	</div>

	<br />

	<div class="flex items-center gap-2">
		<button onclick={downloadVideo} class="btn btn-secondary" disabled={isDownloading}
			>Download Video</button
		>
		<button onclick={playVideo} class="btn btn-info">Play Video</button>
		<button onclick={playAllVideos} class="btn btn-success">Play All Videos</button>
	</div>

	<br />

	{#if isDownloading}
		<p><span class="loading loading-ball"></span> Downloading video...</p>
	{/if}

	<p>Status: {status}</p>
</div>

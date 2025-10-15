<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { readFile } from '@tauri-apps/plugin-fs';

	let config = $state('');
	let videoUrl = $state('');
	let videoPath = $state('');
	let status = $state('');
	let isDownloading = $state(false);
	let videoList = $state<string[]>([]);
	let videos = $state<string[]>([]);
	let currentVideoIndex = $state(-1);
		let videoElement: HTMLVideoElement | undefined = $state(undefined);
		let videoError = $state('');
	let mp4Folder = $state('');

	onMount(async () => {
		document.addEventListener('fullscreenchange', () => {
			if (!document.fullscreenElement) {
				currentVideoIndex = -1;
				status = 'Playback stopped';
			}
		});
		try {
			mp4Folder = await invoke('get_mp4_folder_cmd');
		} catch (e) {
			status = `Error loading MP4 folder: ${e}`;
		}
	});

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
			videoList = await invoke('get_video_list');
			if (videoList.length === 0) {
				status = 'No videos found';
				return;
			}
			currentVideoIndex = 0;
			playCurrentVideo();
			status = 'Playing all videos';
		} catch (e) {
			status = `Error: ${e}`;
		}
	}

		async function playCurrentVideo() {
			if (currentVideoIndex >= 0 && currentVideoIndex < videoList.length) {
				const filename = videoList[currentVideoIndex];
				const fullPath = `${mp4Folder}/${filename}.mp4`;

				if (!videoElement) {
					return;
				}

				try {
					const data = await readFile(fullPath);
					const blob = new Blob([data], { type: 'video/mp4' });
					const src = URL.createObjectURL(blob);
					videoElement.src = src;
					await videoElement.play();
					if (videoElement.requestFullscreen) {
						await videoElement.requestFullscreen();
					}
				} catch (e) {
					status = `Error playing video: ${e}`;
				}
			}
		}

		function playNext() {
			currentVideoIndex++;
			if (currentVideoIndex >= videoList.length) {
				currentVideoIndex = 0; // loop back
			}
			playCurrentVideo();
		}

		function handleVideoError(event: Event) {
			const target = event.target as HTMLVideoElement;
			const error = target.error;
			let errorMsg = 'Unknown error';
			if (error) {
				switch (error.code) {
					case 1:
						errorMsg = 'Aborted';
						break;
					case 2:
						errorMsg = 'Network error';
						break;
					case 3:
						errorMsg = 'Decode error';
						break;
					case 4:
						errorMsg = 'Source not supported';
						break;
					default:
						errorMsg = `Error code: ${error.code}`;
				}
			}
			videoError = `Video error: ${errorMsg}`;
			status = videoError;
		}

	async function listVideos() {
		try {
			videos = await invoke('get_video_list');
			status = `Found ${videos.length} videos`;
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
		<button onclick={listVideos} class="btn btn-warning">List Videos</button>
	</div>

	<br />

	{#if isDownloading}
		<p><span class="loading loading-ball"></span> Downloading video...</p>
	{/if}

	<p>Status: {status}</p>
	{#if videoError}
		<p>Error: {videoError}</p>
	{/if}

	{#if videos.length > 0}
		<ul>
			{#each videos as v}
				<li>{v}</li>
			{/each}
		</ul>
	{/if}

	{#if currentVideoIndex >= 0}
		<p>Now playing: {videoList[currentVideoIndex]}</p>
		<video
			bind:this={videoElement}
			onended={playNext}
			onerror={handleVideoError}
			controls={false}
			preload="auto"
			style="width: 100%; max-width: 800px;"
			class="no-controls"
		>
			<track kind="captions" />
		</video>
	{/if}
</div>

<footer>
	<p>Video Hub@0.1.0</p>
</footer>

<style>
	.no-controls::-webkit-media-controls {
		display: none !important;
	}
	.no-controls::-webkit-media-controls-panel {
		display: none !important;
	}
	.no-controls::-webkit-media-controls-play-button {
		display: none !important;
	}
	.no-controls::-webkit-media-controls-volume-slider {
		display: none !important;
	}
	.no-controls::-webkit-media-controls-mute-button {
		display: none !important;
	}
	.no-controls::-webkit-media-controls-timeline {
		display: none !important;
	}
	.no-controls::-webkit-media-controls-current-time-display {
		display: none !important;
	}
	.no-controls::-webkit-media-controls-time-remaining-display {
		display: none !important;
	}
	.no-controls::-webkit-media-controls-fullscreen-button {
		display: none !important;
	}
	.no-controls::-moz-media-controls {
		display: none !important;
	}
	.no-controls::-moz-media-controls-panel {
		display: none !important;
	}
	.no-controls::-moz-media-controls-play-button {
		display: none !important;
	}
	.no-controls::-moz-media-controls-volume-slider {
		display: none !important;
	}
	.no-controls::-moz-media-controls-mute-button {
		display: none !important;
	}
	.no-controls::-moz-media-controls-timeline {
		display: none !important;
	}
	.no-controls::-moz-media-controls-current-time-display {
		display: none !important;
	}
	.no-controls::-moz-media-controls-time-remaining-display {
		display: none !important;
	}
	.no-controls::-moz-media-controls-fullscreen-button {
		display: none !important;
	}
</style>

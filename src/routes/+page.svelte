<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke, convertFileSrc } from '@tauri-apps/api/core';

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

	// Video selection with duration
	type VideoSelection = {
		name: string;
		selected: boolean;
		duration: number;
	};
	let videoSelections = $state<VideoSelection[]>([]);

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
				videoElement.src = convertFileSrc(fullPath);
				await videoElement.play();
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
			// Initialize video selections with default duration of 10 seconds
			videoSelections = videos.map((name) => ({
				name,
				selected: false,
				duration: 10
			}));
			status = `Found ${videos.length} videos`;
		} catch (e) {
			status = `Error: ${e}`;
		}
	}

	function getSelectedVideos(): string[] {
		return videoSelections
			.filter((selection) => selection.selected)
			.map((selection) => selection.name);
	}

	async function playSelectedVideosExternally() {
		try {
			const selectedVideos = getSelectedVideos();
			if (selectedVideos.length === 0) {
				status = 'No videos selected';
				return;
			}
			await invoke('play_selected_videos_externally', { videos: selectedVideos });
			status = `Playing ${selectedVideos.length} selected videos externally`;
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
		<button onclick={playSelectedVideosExternally} class="btn btn-primary"
			>Play Selected Videos Externally</button
		>
	</div>

	<br />

	{#if isDownloading}
		<p><span class="loading loading-ball"></span> Downloading video...</p>
	{/if}

	<p>Status: {status}</p>
	{#if videoError}
		<p>Error: {videoError}</p>
	{/if}

	{#if videoSelections.length > 0}
		<div class="mt-6">
			<h3 class="mb-4 text-lg font-semibold">Available Videos</h3>
			<div class="max-h-96 space-y-3 overflow-y-auto">
				{#each videoSelections as selection, index}
					<div class="flex items-center gap-4 rounded-lg border bg-base-200 p-3">
						<label class="flex cursor-pointer items-center gap-2">
							<input
								type="checkbox"
								bind:checked={selection.selected}
								class="checkbox checkbox-primary"
							/>
							<span class="font-medium">{selection.name}</span>
						</label>

						<div class="ml-auto flex items-center gap-2">
							<label class="text-sm font-medium">Duration:</label>
							<input
								type="number"
								min="1"
								max="300"
								bind:value={selection.duration}
								class="input-bordered input input-sm w-20"
								placeholder="10"
							/>
							<span class="text-sm text-gray-500">seconds</span>
						</div>
					</div>
				{/each}
			</div>

			<div class="mt-4 text-sm text-gray-600">
				Selected: {getSelectedVideos().length} videos
			</div>
		</div>
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

	<br />
	<br />

	<footer>
		<p class="font-bold">Video Hub@0.1.0</p>
	</footer>
</div>

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

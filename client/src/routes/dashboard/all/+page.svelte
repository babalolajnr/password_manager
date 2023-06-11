<script lang="ts">
	import { onMount } from 'svelte';
	import { baseUrl } from '../../../config';
	let open = false;

	function toggle() {
		open = !open;
	}

	async function getPasswords() {
		const response = await fetch(`${baseUrl}/password/`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok) {
			const data = await response.json();
			console.log(data);
		}
	}

	onMount(async () => {
		await getPasswords();
	});
</script>

<ul>
	<li class="border border-black border-opacity-20 rounded-lg py-4 px-3 flex flex-col gap-4">
		<div class="lg:flex justify-between">
			<div class="lg:flex gap-3">
				<div class="h-10 w-10 rounded-full bg-red-500" />
				<div class="flex items-center gap-1">
					<h3 class="text-lg font-semibold">YouTube</h3>
					<div class="flex items-baseline gap-1">
						<span class="text-gray-400"> - </span>
						<p class="text-gray-400">babalolajnr</p>
					</div>
				</div>
			</div>
			<div class="flex">
				<!-- Collapse Icon -->
				<div class="flex justify-end">
					<button class="focus:outline-none" on:click={toggle}>
						<svg
							class={'w-6 h-6 text-gray-400 duration-200' + (open ? ' transform rotate-180 ' : '')}
							fill="none"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path d="M19 9l-7 7-7-7" />
						</svg>
					</button>
				</div>
			</div>
		</div>
		<div class="flex flex-col border border-black border-opacity-20 rounded-lg">
			<div
				class="flex items-center justify-between border-b border-black border-opacity-20 px-4 py-2"
			>
				<div class="flex gap-2">
					<!-- Bookmark Icon -->
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
						<path
							d="M19 2H5c-1.1 0-2 .9-2 2v16l8-4.86L19 20V4c0-1.1-.9-2-2-2z"
							fill="none"
							stroke="currentColor"
							stroke-opacity="0.5"
						/>
					</svg>
				</div>
				<div>
					<!-- Edit button -->
					<button class="focus:outline-none border py-1 px-5 rounded-lg text-sm"> Edit </button>
				</div>
			</div>
			<div class="px-4">
				<div class="pt-5 pb-4">
					<ul class="flex flex-col gap-3">
						<li>
							<span class="font-bold">username</span>
							<span>babalolajnr</span>
						</li>
						<li>
							<span class="font-bold">email</span>
							<span>babalolajnr@gmail.com</span>
						</li>
						<li>
							<span class="font-bold">password</span>
							<span>**********</span>
						</li>
						<li>
							<span class="font-bold">website</span>
							<span>https://www.yotube.com</span>
						</li>
						<li>
							<span class="font-bold">username</span>
							<span>babalolajnr</span>
						</li>
					</ul>
					<ul />
				</div>
			</div>
		</div>
	</li>
</ul>

<style>
</style>

<script lang="ts">
	import NavBar from './components/NavBar.svelte';
	import SearchIcon from './components/SearchIcon.svelte';
	import SideBar from './components/SideBar.svelte';
</script>

<div class="flex flex-col mt-3 mx-12 nunito">
	<header class="flex justify-between">
		<div>PassMan</div>
		<NavBar />
		<div class="flex gap-3">
			<div>Image</div>
			<div>Email</div>
		</div>
	</header>
	<div class="border rounded-2xl flex mt-5">
		<SideBar />
		<div class="grow flex justify-center">
			<div class="mt-6 w-2/3 flex flex-col gap-10">
				<div class="flex justify-between">
					<div
						class="flex items-center border border-black border-opacity-20 rounded-lg w-1/2 px-2 py-2"
					>
						<SearchIcon />
						<input type="text" placeholder="Search" class="w-full outline-none" />
					</div>
					<button class="px-4 rounded-lg shadow-inner bg-purple-700 text-white">+ Add New</button>
				</div>
				<div>
					<slot />
				</div>
			</div>
		</div>
	</div>
</div>

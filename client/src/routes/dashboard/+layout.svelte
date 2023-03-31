<script lang="ts">
</script>

<div class="flex flex-col mt-3 mx-12">
	<header class="flex justify-between">
		<div>PassMan</div>
		<nav>
			<ul class="flex gap-3 items-center font-medium">
				<li
					class="px-6 py-2 rounded-lg hover:bg-slate-600 hover:bg-opacity-10 cursor-pointer transition-all duration-150"
				>
					All
				</li>
				<li
					class="px-6 py-2 rounded-lg hover:bg-slate-600 hover:bg-opacity-10 cursor-pointer transition-all duration-150"
				>
					Recently Imported
				</li>
				<li
					class="px-6 py-2 rounded-lg hover:bg-slate-600 hover:bg-opacity-10 cursor-pointer transition-all duration-150"
				>
					Favorites
				</li>
			</ul>
		</nav>
		<div class="flex gap-3">
			<div>Image</div>
			<div>Email</div>
		</div>
	</header>
	<div class="border rounded-2xl flex mt-5">
		<aside class="px-4 min-h-screen mx-6 basis-1/5 border-r">
			<div class="flex flex-col gap-5 mt-6">
				<div class="flex flex-col gap-3">
					<h3 class="font-medium text-black text-opacity-50">CATEGORIES</h3>
					<ul class="flex flex-col gap-2 ml-3">
						<li>Logins</li>
						<li>Identities</li>
						<li>Movie Apps</li>
						<li>Social Media</li>
						<li>News</li>
					</ul>
				</div>
				<div class="flex flex-col gap-3">
					<h3 class="font-medium text-black text-opacity-50">TAGS</h3>
					<ul class="flex flex-col gap-2 ml-3">
						<li>Starter Kit</li>
						<li>School</li>
						<li>Business</li>
						<li>Design Tool</li>
					</ul>
				</div>
			</div>
		</aside>
		<div>
			<slot />
		</div>
	</div>
</div>

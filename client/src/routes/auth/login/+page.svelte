<script lang="ts">
	import backgroundImage from '../../../lib/images/login.jpg';
	import { baseUrl } from '../../../config';
	import { fade, slide } from 'svelte/transition';
	import { goto } from '$app/navigation';

	let form = {
		email: '',
		password: '',
		rememberMe: false
	};

	let error = false;
	let loading = false;

	function toggleLoading() {
		loading = !loading;
	}

	async function submit(event: Event) {
		event.preventDefault();
		toggleLoading();

		const response = await fetch(`${baseUrl}/auth/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(form)
		});


		if (response.ok) {
			// Navigate to dashboard
            goto('/dashboard')
		} else {
			error = true;
			setTimeout(() => {
				error = false;
			}, 3000);
		}

		toggleLoading();
	}

	// $: {
	// 	console.error(form.email);
	// }
</script>

<main class="lg:min-h-screen font-inter">
	<div class="lg:flex lg:min-h-screen">
		<div class="lg:basis-1/2 bg-cover" style="background-image:url({backgroundImage})" />
		<div class="flex grow justify-center items-center">
			<form class="w-1/2 flex flex-col gap-5" on:submit={submit}>
				<div>
					<h3 class="text-lg text-black text-opacity-50">Welcome Back</h3>
					<h1 class="text-3xl font-extrabold">Login to your Account</h1>
				</div>
				<div class="flex flex-col gap-3">
					{#if error}
						<span class="text-red-600 font-semibold" transition:slide>Invalid Credentials</span>
					{/if}
					<div class="flex flex-col-reverse">
						<input
							type="email"
							class="py-3 border-2 border-black border-opacity-20 outline-none px-3 rounded-lg peer focus:border-black transition-all ease-in duration-150"
							name="email"
							bind:value={form.email}
							required
						/>
						<label
							for="email"
							class="text-black text-opacity-50 peer-focus:text-black transition-all ease-in duration-150"
							>Email</label
						>
					</div>
					<div class="flex flex-col-reverse">
						<input
							type="password"
							class="py-3 border-2 border-black border-opacity-20 outline-none px-3 rounded-lg peer focus:border-black transition-all ease-in duration-150"
							name="password"
							bind:value={form.password}
							required
						/>
						<label
							for="password"
							class="text-black text-opacity-50 peer-focus:text-black transition-all ease-in duration-150"
							>Password</label
						>
					</div>
					<div class="flex justify-between">
						<div>
							<input
								type="checkbox"
								name="remember-me"
								id=""
								class="peer"
								bind:checked={form.rememberMe}
							/>
							<label
								for="remember-me"
								class="text-opacity-50 text-black peer-checked:text-black transition-all duration-150 ease-in"
								>Remember me</label
							>
						</div>
						<div>
							<a
								href=""
								class="text-blue-600 hover:border-b-2 border-blue-500 transition-all duration-150 ease-in py-1"
								>Forgot password?</a
							>
						</div>
					</div>
					<div class="flex">
						<button
							type="submit"
							class="bg-green-500 grow py-3 rounded-lg font-semibold text-white
                             hover:bg-transparent hover:text-black hover:border-2
                            hover:border-green-500 transition-all duration-75 ease-in border-2 border-transparent"
							class:opacity-30={loading}
							disabled={loading}>Login</button
						>
					</div>
				</div>
			</form>
		</div>
	</div>
</main>

<style lang="postcss">
</style>

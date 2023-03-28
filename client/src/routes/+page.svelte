<script>
  import backgroundImage from "./assets/images/login.jpg";
</script>

<main class="lg:min-h-screen font-inter">
	<div class="lg:flex lg:min-h-screen">
		<div class="lg:basis-1/2 bg-cover" style="background-image:url({backgroundImage})" />
		<div class="flex grow justify-center items-center">
			<div class="w-1/2 flex flex-col gap-5">
				<div>
					<h3 class="text-lg text-black text-opacity-50">Welcome Back</h3>
					<h1 class="text-3xl font-extrabold">Login to your Account</h1>
				</div>
				<div class="flex flex-col gap-3">
					<div class="flex flex-col-reverse">
						<input
							type="email"
							class="py-3 border-2 border-black border-opacity-20 outline-none px-3 rounded-lg peer focus:border-black transition-all ease-in duration-150"
							name="email"
						/>
						<label
							for="email"
							class="text-black text-opacity-50 peer-focus:text-black transition-all ease-in duration-150"
							>Email</label
						>
					</div>
					<div class="flex flex-col-reverse">
						<input
							type="email"
							class="py-3 border-2 border-black border-opacity-20 outline-none px-3 rounded-lg peer focus:border-black transition-all ease-in duration-150"
							name="password"
						/>
						<label
							for="password"
							class="text-black text-opacity-50 peer-focus:text-black transition-all ease-in duration-150"
							>Password</label
						>
					</div>
					<div class="flex justify-between">
						<div>
							<input type="checkbox" name="remember-me" id="" class="peer" />
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
							class="bg-green-500 grow py-3 rounded-lg font-semibold text-white
                hover:bg-transparent hover:text-black hover:border-2
                 hover:border-green-500 transition-all duration-75 ease-in border-2 border-transparent"
							>Login</button
						>
					</div>
				</div>
			</div>
		</div>
	</div>
</main>

<style lang="postcss">
</style>

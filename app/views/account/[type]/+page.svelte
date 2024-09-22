<script lang="ts">
	import Input from '$lib/Input.svelte';

	let email: string = '';
	let password: string = '';
</script>

<form
	class="flex h-full w-full flex-col justify-center gap-4 self-center bg-slate-100 p-8 md:h-min md:w-min md:rounded-xl md:drop-shadow-lg"
>
	<!-- Top line -->
	<div class="flex flex-col items-center gap-8 md:flex-row md:gap-16">
		<div class="absolute top-16 flex flex-col items-center justify-center md:static">
			<img src="/favicon.png" alt="Logo" class="aspect-square h-full object-contain" />
			<h1 class="text-xl font-bold">Connexion</h1>
		</div>
		<div class="flex w-80 flex-col gap-1">
			<Input
				value="{email}"
				error="Adresse e-mail incorrect"
				placeholder="Adresse e-mail"
				type="text"
			/>
			<Input value="{password}" placeholder="Mot de passe" type="password" />
		</div>
	</div>
	<!-- Bottom line-->
	<div class="flex h-10 w-full content-end items-center">
		<a
			type="submit"
			class="mr-auto text-gray-700 transition-all hover:text-black"
			href="/password-reset"
		>
			Mot de passe oublié ?
		</a>
		<button
			type="submit"
			class="ml-auto h-full rounded-full bg-orange-500 px-4 text-slate-100 shadow-sm transition-all hover:bg-orange-600 hover:shadow-lg"
		>
			Valider
		</button>
	</div>
</form>

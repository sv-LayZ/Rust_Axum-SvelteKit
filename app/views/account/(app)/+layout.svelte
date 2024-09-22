<script lang="ts">
	import Navbar from '$lib/Navbar.svelte';
</script>

<Navbar>
	<a
		class="w-full rounded-md p-2 drop-shadow-md transition-colors duration-500 hover:bg-orange-500 hover:text-white"
		href="/"
	>
		Authorization
	</a>
</Navbar>
<div class="m-8 flex-1"><slot></slot></div>

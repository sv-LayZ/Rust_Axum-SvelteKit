<script lang="ts">
	export let value: string;
	export let placeholder: string | null = null;
	export let type: string = 'text';
	export let alt: string = '';
	export let error: string = '';
	const typeWorkaround = (node: any) => (node.type = type);
</script>

<div class="relative pb-4 pt-6">
	<div class="relative text-lg">
		{#if placeholder}
			<label
				class="pointer-events-none absolute transition-all {value
					? '-top-6 px-1 text-base text-orange-500'
					: 'top-3 px-4 text-gray-700'}"
				for="this"
			>
				{placeholder}
			</label>
		{/if}
		<input
			id="this"
			alt="{alt}"
			use:typeWorkaround
			bind:value="{value}"
			class="w-full rounded-md border p-3 shadow-sm {error ? 'border-red-500' : ''}"
		/>
	</div>
	<span class="absolute bottom-0 px-1 text-xs text-red-500">{error}</span>
</div>

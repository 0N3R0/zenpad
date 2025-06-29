<script lang="ts">
  import { goto } from "$app/navigation";

	export let type: "button" | "submit" | "reset" = "button";
	export let href: string | null = null;
	export let ariaLabel: string = "";
	export let disabled: boolean = false;
	export let visible: boolean = true;
	export let _class: string = "";

	function handle_click () {
		if (!href)
			return;

		console.log(`Going to ${href}`);
		
		goto(href ? href : "");
	}
</script>

{#if visible}
	<button
		on:click={handle_click}
		type={type}
		disabled={disabled}
		aria-label={ariaLabel}
		class="{_class} flex items-center justify-center px-3 py-2 text-gray-200 text-balance cursor-pointer darker-grotesque"
	>
		<slot />
	</button>
{/if}
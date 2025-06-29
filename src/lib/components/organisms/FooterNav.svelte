<script lang="ts">
	import NavLink from '../molecules/NavLink.svelte';
	import { get_nav_links_as_array } from '../../configs/navigation';
	import { page } from '$app/state';


	const nav_links = get_nav_links_as_array();
</script>

<footer class="min-h-22 sticky bottom-0.5 w-full flex justify-center items-center">

	<div class="w-full h-full flex justify-center items-center gap-1">

		{#each nav_links as {label, href, icon, disabled, visible}}

			<div class={`
				transition-all duration-300 ease-in-out mt-auto w-30 border-b-2
				${page.url.pathname === href ? 'border-cyan-200' : 'border-transparent'}
				${page.url.pathname !== href ? 'hover:border-gray-200' : ''}
				flex justify-center items-center text-gray-950`
			}>
				<NavLink
					href={href}
					icon={icon}
					disabled={disabled}
					visible={visible}
					ariaLabel={label}
					type='button'
				/>
			</div>

		{/each}

	</div>

</footer>
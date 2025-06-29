<script lang="ts">
  import '../app.css';

  import FooterNav from '../lib/components/organisms/FooterNav.svelte';
</script>

<main class="w-full max-h-full overflow-hidden flex-grow border-b-2 border-double border-slate-800">
  <slot />
</main>

<FooterNav />
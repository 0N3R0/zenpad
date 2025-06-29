
<script lang="ts">
  import NotesLeftBarOptions from "../molecules/NotesLeftBarOptions.svelte";
  import NotesLeftBarContent from "../molecules/NotesLeftBarContent.svelte";

  export let hide: boolean = false;

</script>

{#if !hide}
  <section class="
    h-full w-4/12
    flex flex-col
    overflow-hidden
    border-r-2 border-double border-slate-800
    xl:w-2/9
    lg:w-2/8
    md:w-2/7"
  >
    <NotesLeftBarOptions />
    <NotesLeftBarContent />
  </section>
{/if}
<script lang="ts">
  import NotesLeftBarContentNote from "./NotesLeftBarContentNote.svelte";
  let noteInfo: string[] = ["Alphabet of morse....Alphabet of morse....Alphabet of morse....Alphabet of morse....", "Woahhh! Did it is it?", "I don't know what it does!", "Alphabet of morse...", "Woahhh! Did it is it?", "I don't know what it does!", "Alphabet of morse...", "Woahhh! Did it is it?", "I don't know what it does!", "Alphabet of morse...", "Woahhh! Did it is it?", "I don't know what it does!", "Alphabet of morse...", "Woahhh! Did it is it?", "I don't know what it does!", "Alphabet of morse...", "Woahhh! Did it is it?", "I don't know what it does!", "Alphabet of morse...", "Woahhh! Did it is it?", "I don't know what it does!"];
</script>

<div class="
  h-full
  flex-1
  overflow-auto
  notes-left-bar-content
  select-none
">
  {#each noteInfo as title}
    <NotesLeftBarContentNote title={title} />
  {/each}
</div>
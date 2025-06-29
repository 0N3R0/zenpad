<script lang="ts">
  import { LEFT_BAR_OPTIONS } from "$lib/configs/notesLeftBarOptions";
  
  export let name: string;
  export let size: number = 24;
  export let stroke_width: string = "1";
</script>

{#if name === LEFT_BAR_OPTIONS.create_note.icon}

  <div>
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} stroke="currentColor" stroke-width={stroke_width} stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" class="icon icon-tabler icons-tabler-outline icon-tabler-note">
      <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
      <path d="M13 20l7 -7" />
      <path d="M13 20v-6a1 1 0 0 1 1 -1h6v-7a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h7" />
    </svg>
  </div>

{:else if name === LEFT_BAR_OPTIONS.delete_note.icon}

  <div>
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} stroke="currentColor" stroke-width={stroke_width} stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" class="icon icon-tabler icons-tabler-outline icon-tabler-trash">
      <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
      <path d="M4 7l16 0" />
      <path d="M10 11l0 6" />
      <path d="M14 11l0 6" />
      <path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" />
      <path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" />
    </svg>
  </div>

{:else if name === LEFT_BAR_OPTIONS.change_notes_directory.icon}

  <div>
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} stroke="currentColor" stroke-width={stroke_width} stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" class="icon icon-tabler icons-tabler-outline icon-tabler-folder-search">
      <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
      <path d="M11 19h-6a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v2.5" />
      <path d="M18 18m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />
      <path d="M20.2 20.2l1.8 1.8" />
    </svg>
  </div>
{/if}
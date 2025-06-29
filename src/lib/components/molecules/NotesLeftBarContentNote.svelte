<script lang="ts">
  import Button from "../atoms/Button.svelte";
  import NotesLeftBarOptionsIcon from "../atoms/NotesLeftBarOptionsIcon.svelte";
  import { LEFT_BAR_OPTIONS } from "../../configs/notesLeftBarOptions";

  export let title: string;
</script>

<div class="
    w-full max-h-11 h-11
    px-3 py-2
    group
    flex items-center justify-center
    border-b-2 border-double border-gray-800
    hover:bg-gray-700
    hover:cursor-pointer
">
  <div class="
    relative
    w-full h-full
    flex items-center
  ">
    <div class="
      flex items-center
      overflow-x-auto
      whitespace-nowrap
      notes-left-bar-content-note
    ">
      <span class="
        h-10
        flex items-center
        text-cyan-200
        text-lg darker-grotesque
      ">
        {title}
      </span>
    </div>
    <div
      class="
        absolute right-[-5]
        w-7 h-7
        hidden group-hover:flex justify-center items-center
        bg-slate-800
        rounded-full
        "
    >
      <NotesLeftBarOptionsIcon name={LEFT_BAR_OPTIONS.delete_note.icon} size={20} />
    </div>
  </div>
</div>
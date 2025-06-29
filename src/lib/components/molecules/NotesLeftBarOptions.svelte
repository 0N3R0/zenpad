<script lang="ts">
  import NotesLeftBarOptionsIcon from "../atoms/NotesLeftBarOptionsIcon.svelte";
  import Button from "../atoms/Button.svelte";
  import { LEFT_BAR_OPTIONS, get_left_bar_options_as_array } from "../../configs/notesLeftBarOptions"

  const OPTIONS = get_left_bar_options_as_array();
</script>

<div class="
  w-full min-h-20
  flex flex-col justify-center items-center
  select-none
">
  <div class="
    w-full
    border-b-2 border-double border-slate-800
  ">
    <Button _class="w-full gap-3 bg-slate-900 hover:bg-slate-950 transition-all duration-200">
      <span class="
        text-lg
      ">
        WCZYTAJ FOLDER
      </span>
      <NotesLeftBarOptionsIcon
        name={LEFT_BAR_OPTIONS.change_notes_directory.icon}
      />
    </Button>
  </div>

  <div class="  
    w-full
    border-b-2 border-double border-slate-800
  ">
    <Button _class="w-full gap-3 bg-slate-900 hover:bg-slate-950 transition-all duration-200">
      <span class="
        text-lg
      ">
        UTWÓRZ NOTATKĘ
      </span>
      <NotesLeftBarOptionsIcon
        name={LEFT_BAR_OPTIONS.create_note.icon}
      />
    </Button>
  </div>
</div>
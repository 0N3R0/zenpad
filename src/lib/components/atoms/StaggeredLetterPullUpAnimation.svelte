<script lang="ts">
  
  import { cn } from "$lib/utils/utils";
  import { AnimatePresence, Motion } from "svelte-motion";

  export let words = "Letter Pull Up";
  export let delay = 0.03;

  let className: any = "";
  export { className as class };
  let pullupVariant = {
    hidden: { y: 100, opacity: 0 },
    visible: (i: any) => ({
      y: 0,
      opacity: 1,
      transition: { delay: i * delay },
    }),
  };
  let letters = words.split("");
</script>

<div class="flex justify-center">
    <!-- <AnimatePresence let:item list={[{ key: words }]}> -->
  {#each letters as letter, i}
      <Motion
        variants={pullupVariant}
        initial="hidden"
        animate="visible"
        custom={i}
        let:motion
      >
        <h1
          class={cn(
            className
          )}
          use:motion
        >
          {#if letter === " "}
            <span>&nbsp;</span>
          {:else}
            {letter}
          {/if}
        </h1>
      </Motion>
      {/each}
    <!-- </AnimatePresence> -->
</div>

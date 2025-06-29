<script lang="ts">
  import Button from "../atoms/Button.svelte";
  import Icon from "../atoms/Icon.svelte";

  export let type: "button" | "submit" | "reset" = "button";
  export let href: string | null = null;
  export let ariaLabel: string = "";
  export let icon: string = "";
  export let disabled: boolean = false;
  export let visible: boolean = true;
</script>


<Button
  _class="w-full select-none"
  type={type}
  href={href}
  ariaLabel={ariaLabel}
  disabled={disabled}
  visible={visible}
>
  <div class="flex flex-col justify-center items-center">
    <Icon name={icon} size={35} />
    
    {ariaLabel}
  </div>
</Button>
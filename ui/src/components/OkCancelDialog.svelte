<script lang="ts">
  import type { Component } from "svelte";

  export let component: Component;
  export let data: object;

  export let showDialog: boolean;

  let dialog: HTMLDialogElement;
  $: {
    if (dialog && showDialog) dialog.showModal();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
  bind:this={dialog}
  on:close={() => (showDialog = false)}
  on:click|self={() => {
    dialog.close();
  }}
>
  <div>
    <svelte:component this={component} {...data} />
  </div>
</dialog>

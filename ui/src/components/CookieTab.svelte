<script lang="ts">
  import { onMount } from "svelte";

  export let getContent: CallableFunction;
  export let updateCallback: CallableFunction;

  let data: string;

  onMount(() => {
    data = getContent();
  });
</script>

<div>
  <textarea
    bind:value={data}
    onchange={() => updateCallback(data)}
    style="width: 100%; height: 5rem"
  ></textarea>
</div>

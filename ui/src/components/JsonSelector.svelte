<script lang="ts">
  import type { TemplateObject } from "../types/TemplateObject";

  export let options: TemplateObject[] = [];
  export let updateSelectCallback: CallableFunction;
  let selected: TemplateObject = {
    id: 0,
    name: "",
    url: "",
    method: "GET",
    header: "{}",
    data: "{}",
  };

  function onChange() {
    console.info(selected);
    updateSelectCallback(selected);
  }
</script>

<div>
  <select bind:value={selected} on:change={onChange}>
    {#each options as option}
      <option value={option}>
        {option.name}
      </option>
    {/each}
  </select>
</div>

<style lang="scss">
  select {
    margin: 10px;
    width: 80%;
    height: 2rem;
  }
</style>

<script lang="ts">
  export let data: string;
  export let updateCallback: CallableFunction;

  $: {
    updateCallback(data);
  }
</script>

<div>
  <textarea bind:value={data} style="width: 100%; height: 5rem"></textarea>
</div>

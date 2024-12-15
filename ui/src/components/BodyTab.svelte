<script lang="ts">
  import { JSONEditor } from "svelte-jsoneditor";

  export let getContent: CallableFunction;
  export let updateCallback: CallableFunction;
</script>

<div>
  <JSONEditor content={getContent()} onChange={updateCallback} />
</div>

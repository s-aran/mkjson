<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import JsonSelector from "../components/JsonSelector.svelte";
  import RequestTab from "../components/RequestTab.svelte";
  import { Method, type MyJson } from "../types/MyJson";
  import type { TemplateObject } from "../types/TemplateObject";
  import OkCancelDialog from "../components/OkCancelDialog.svelte";
  import NameInput from "../dialogs/NameInput.svelte";
  import { getText } from "../utils/json";

  let config: Config = {
    version: 0,
    templates: [],
  };

  let showInputNameDialog = false;
  let response: str = "";
  let updateResponse = (r: string) => {
    response = r;
  };

  let loadedName: string;
  let loadedData: MyJson = {
    url: "",
    method: Method.GET,
    header: { json: {} },
    cookie: "",
    body: { json: {} },
  };

  const onSelectUpdate = (selected: TemplateObject) => {
    console.info(selected);

    loadedName = selected.name;
    loadedData = {
      url: selected.url,
      method: selected.method,
      header: { text: selected.header },
      cookie: "",
      body: { json: JSON.parse(selected.data) },
    };
  };

  onMount(async () => {
    config = await invoke("load_config");
    console.info("load_config", config);
  });

  $: {
    if (config.version > 0) {
      invoke("save_config", { config });
    }
  }
</script>

<header>
  <div>
    <div id="selector-area">
      <JsonSelector
        options={config.templates.map((e, i) => ({
          id: i,
          name: e.name,
          url: e.url,
          method: e.method,
          header: e.header,
          data: e.data,
        }))}
        updateSelectCallback={onSelectUpdate}
      />
    </div>

    <button
      onclick={async () => {
        const target = config.templates.filter((e) => e.name === loadedName)[0];
        target.url = loadedData.url;
        target.header = getText(loadedData.header);
        target.data = getText(loadedData.body);

        await invoke("save_config", { config });
      }}>save</button
    >
    <!-- <button
      onclick={async () => {
        showInputNameDialog = true;
      }}>add new</button
    >
    <button onclick={async () => await invoke("save_config", { config })}
      >duplicate</button
    > -->
  </div>
</header>

<div class="container">
  <dev>
    <RequestTab data={loadedData} {response} updateCallback={updateResponse} />
  </dev>

  <div>
    <textarea readonly bind:value={response} style="width: 99%; height: 5rem"
    ></textarea>
  </div>
</div>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import {
    JSONEditor,
    type Content,
    type ContentErrors,
    type JSONPatchResult,
  } from "svelte-jsoneditor";
  import JsonSelector from "../components/JsonSelector.svelte";
  import RequestTab from "../components/RequestTab.svelte";
  import { Method, type MyJson } from "../types/MyJson";
  import type { TemplateObject } from "../types/TemplateObject";

  let config: Config = {
    version: 0,
    templates: [],
  };

  let content: Content = {
    text: undefined, // can be used to pass a stringified JSON document instead
    json: {
      array: [1, 2, 3],
      boolean: true,
      color: "#82b92c",
      null: null,
      number: 123,
      object: { a: "b", c: "d" },
      string: "Hello World",
    },
  };

  let response: str = "";

  let loadedData: MyJson = {
    url: "",
    method: Method.GET,
    header: { json: {} },
    cookie: "",
    body: { json: {} },
  };

  const onSelectUpdate = (selected: TemplateObject) => {
    console.info(selected);

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

  const handleChange = (
    updatedContent: Content,
    previousContent: Content,
    changeStatus: {
      contentErrors: ContentErrors | undefined;
      patchResult: JSONPatchResult | undefined;
    },
  ) => {
    // content is an object { json: unknown } | { text: string }
    console.log("onChange: ", {
      updatedContent,
      previousContent,
      changeStatus,
    });

    content = updatedContent;
  };

  $: {
    if (config.version > 0) {
      invoke("save_config", { config });
    }
  }
</script>

<header>
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
</header>

<div class="container">
  <dev>
    <RequestTab data={loadedData} />
  </dev>

  <div>
    <textarea id="response" readonly bind:value={response}></textarea>
  </div>
</div>

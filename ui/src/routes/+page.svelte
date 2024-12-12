<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { JSONEditor } from "svelte-jsoneditor";
  import JsonSelector from "../components/JsonSelector.svelte";

  let config: Config = {
    version: 0,
    templates: [],
  };

  let content: JSON = JSON.parse(
    JSON.stringify({
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
    }),
  );

  let cookie: string = "";
  let url: string = "";
  let response: string = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  }

  const getContent = () => content;
  const updateContent = (newContent: string) => {
    content = JSON.parse(JSON.stringify({ json: JSON.parse(newContent) }));
  };

  const onSelectUpdate = (selected: TemplateObject) => {
    console.info(selected);
    url = selected.url;
    content = JSON.parse(selected.data);
    updateContent(selected.data);
  };

  onMount(async () => {
    config = await invoke("load_config");
    console.info("load_config", config);
  });

  const handleChange = (
    updatedContent: JSON,
    previousContent: JSON,
    { contentErrors, patchResult },
  ) => {
    // content is an object { json: unknown } | { text: string }
    console.log("onChange: ", {
      updatedContent,
      previousContent,
      contentErrors,
      patchResult,
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
        data: e.data,
      }))}
      updateSelectCallback={onSelectUpdate}
    />
  </div>
</header>

<div class="container">
  <p>
    <textarea id="cookie" placeholder="Cookie..." bind:value={cookie}
    ></textarea>
  </p>
  <p>
    <input id="url" bind:value={url} placeholder="url..." />
  </p>
  <p>
    <button
      type="submit"
      name="get"
      onclick={async () => await invoke("http_get")}>GET</button
    >
    <button
      type="submit"
      name="post"
      onclick={async () => {
        console.info(getContent());

        response = await invoke("http_post", {
          urlStr: url,
          cookieStr: cookie,
          headers: { hoge: "piyo" },
          dataStr: JSON.stringify(getContent()),
        });
      }}>POST</button
    >
    <button type="submit" name="put">PUT</button>
    <button type="submit" name="delete">DELETE</button>
  </p>

  <div>
    <JSONEditor {content} onchange={handleChange} />
  </div>

  <dic>
    <textarea id="response" readonly bind:value={response}></textarea>
  </dic>
</div>

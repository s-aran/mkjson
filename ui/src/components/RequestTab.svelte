<script lang="ts">
  import type { Content } from "svelte-jsoneditor";
  import BodyTab from "./BodyTab.svelte";
  import Tabs from "./Tabs.svelte";

  import { invoke } from "@tauri-apps/api/core";
  import { Method, type MyJson } from "../types/MyJson";
  import { getJson, getText } from "../utils/json";
  import CookieTab from "./CookieTab.svelte";
  import HeaderTab from "./HeaderTab.svelte";

  export let data: MyJson;
  export let response: string;
  export let updateCallback: CallableFunction;

  let items = [
    {
      label: "Body",
      value: 1,
      component: BodyTab,
      data: {
        getContent: () => data.body,
        updateCallback: (c: Content) => {
          console.info("cb", c);
          data.body = c;
        },
      },
    },
    {
      label: "Header",
      value: 2,
      component: HeaderTab,
      data: {
        getContent: () => data.header,
        updateCallback: (c: Content) => {
          console.info("cb", c);
          data.header = c;
        },
      },
    },
    {
      label: "Cookie",
      value: 3,
      component: CookieTab,
      data: {
        getContent: () => data.cookie,
        updateCallback: (c: string) => {
          console.info("cb", c);
          data.cookie = c;
        },
      },
    },
  ];

  const getHandlerName = () => {
    switch (data.method) {
      case Method.GET:
        return "http_get";
      case Method.POST:
        return "http_post";
      case Method.PUT:
        return "http_put";
      case Method.PATCH:
        return "http_patch";
      case Method.DELETE:
        return "http_delete";
      default:
        throw "unknown method";
    }
  };
</script>

<p>
  <input id="url" bind:value={data.url} placeholder="url..." />
</p>

<p>
  <button
    type="submit"
    name="post"
    onclick={async () => {
      console.info(data.header);
      const response = await invoke(getHandlerName(), {
        urlStr: data.url,
        cookieStr: data.cookie,
        headers: getJson(data.header),
        dataStr: getText(data.body),
      });
      updateCallback(response);
    }}>send</button
  >
</p>

<Tabs {items} />

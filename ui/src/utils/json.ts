import type { Content } from "svelte-jsoneditor";

export const getText = (content: Content) => {
	if ("text" in Object.keys(content) && content.text) {
		return content.text;
	} else {
		return JSON.stringify(content.json);
	}
};

export const getJson = (content: Content) => {
	if ("json" in Object.keys(content) && content.json) {
		return content.json;
	} else {
		return JSON.parse(content.text);
	}
};

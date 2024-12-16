import type { Content } from "svelte-jsoneditor";

export const getText = (content: Content) => {
	if (Object.keys(content).includes("text") && content.text) {
		return content.text;
	} else {
		return JSON.stringify(content.json);
	}
};

export const getJson = (content: Content) => {
	if (Object.keys(content).includes("json") && content.json) {
		return content.json;
	} else {
		return JSON.parse(content.text);
	}
};

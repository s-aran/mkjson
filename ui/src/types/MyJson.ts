import type { Content } from "svelte-jsoneditor";

export enum Method {
	GET = "GET",
	POST = "POST",
	PUT = "PUT",
	PATCH = "PATCH",
	DELETE = "DELETE",
}

export interface MyJson {
	url: string;
	method: string;
	header: Content;
	cookie: string;
	body: Content;
}

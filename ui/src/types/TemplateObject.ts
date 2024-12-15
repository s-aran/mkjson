import type { Method } from "./MyJson";

export interface TemplateObject {
	id: number;
	name: string;
	url: string;
	method: string;
	header: string;
	data: string;
}

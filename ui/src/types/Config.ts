interface ConfigTemplate {
	name: string;
	url: string;
	method: string;
	header: string;
	data: string;
}

interface Config {
	version: number;
	templates: ConfigTemplate[];
}

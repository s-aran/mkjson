interface ConfigTemplate {
	name: string;
	url: string;
	data: string;
	header: string;
}

interface Config {
	version: number;
	templates: ConfigTemplate[];
}

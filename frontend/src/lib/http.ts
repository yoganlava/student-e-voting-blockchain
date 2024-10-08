export default async function http(url: string, data: any = undefined, method: string = 'GET') {
	let config: any = {
		method,
		headers: {
			'Content-Type': 'application/json'
		}
	};

	if (method != 'GET') config.body = JSON.stringify(data);

	console.log(config);

	const response = await fetch(url, config);

	if (!response.ok) return console.error(response);
	return response.json();
}

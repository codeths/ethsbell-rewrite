function $(id) {
	return document.querySelector(`#${id}`);
}

async function request(url, request, error_span) {
	request = {
		headers: {
			Authorization,
		},
		...request,
	};
	const result = await fetch(url, request);
	if (result.ok) {
		$(error_span).innerHTML = '';
		return result;
	}

	const error = `Failed with ${result.status}: ${await result.text()}`;
	$(error_span).innerHTML = error;
	throw new Error(error);
}

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
	const result = await fetch(url, request).catch(error => {});
	if (error_span) {
		if (!result) {
			$(error_span).innerHTML = 'Failed to fetch';
			throw new Error('Failed to fetch');
		}

		if (result.ok) {
			$(error_span).innerHTML = '';
			return true;
		}

		const error = `Failed with ${result.status}: ${await result.text()}`;
		$(error_span).innerHTML = error;
		throw new Error(error);
	}

	return result && result.ok;
}

Object.assign(window, {
	$,
	request,
});

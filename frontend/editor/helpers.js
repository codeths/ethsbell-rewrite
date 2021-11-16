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
	const result = await fetch(url, request).catch(e => {});
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

function setCookie(name, value, expires, path = window.location.pathname) {
	document.cookie = `${name}=${encodeURIComponent(value)}; ${
		expires ? `expires=${new Date(expires).toUTCString()}; ` : ''
	}path=${path}`;
}

function getCookie(name) {
	return document.cookie.split('; ').reduce((r, v) => {
		const parts = v.split('=');
		return parts[0] === name ? decodeURIComponent(parts[1]) : r;
	}, '');
}

function deleteCookie(name, path) {
	setCookie(name, '', null, path);
}

Object.assign(window, {
	$,
	request,
	setCookie,
	getCookie,
	deleteCookie,
});

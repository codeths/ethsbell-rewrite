let Authorization = getCookie('Authorization');

const {setCookie, getCookie, request, $} = window;

$('auth_form').addEventListener('submit', event => {
	event.preventDefault();
	const username = $('auth_username').value;
	const password = $('auth_password').value;
	const auth_b64 = btoa(`${username}:${password}`);
	authenticate(`Basic ${auth_b64}`);
});

async function authenticate(auth = Authorization) {
	Authorization = auth;
	const ok = await request('../api/v1/check-auth', {});
	if (ok) {
		setCookie(
			'Authorization',
			Authorization,
			new Date().setHours(24, 0, 0, 0),
		);
		$('authenticate').style.display = 'none';
		$('settings').style.display = 'initial';
	} else {
		$('auth_error').innerHTML = 'Failed to log in. Please try again.';
	}
}

if (Authorization) {
	authenticate();
}

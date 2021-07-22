let Authorization;

$('auth_form').addEventListener('submit', event => event.preventDefault());

$('auth_check').addEventListener('click', async () => {
	const username = $('auth_username').value;
	const password = $('auth_password').value;
	const auth_b64 = btoa(`${username}:${password}`);
	Authorization = `Basic ${auth_b64}`;
	await req('../api/v1/check-auth', {}, 'auth_error');
	$('authenticate').innerHTML = 'Auth OK';
});

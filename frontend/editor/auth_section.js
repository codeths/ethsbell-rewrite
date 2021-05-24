let Authorization;

$("auth_form").addEventListener("submit", event => event.preventDefault())

$("auth_check").addEventListener('click', async () => {
	let username = $("auth_username").value
	let password = $("auth_password").value
	let auth_b64 = btoa(`${username}:${password}`)
	Authorization = `Basic ${auth_b64}`
	await req('../api/v1/spec', {}, "auth_error")
	$("authenticate").innerHTML = "Auth OK"
})

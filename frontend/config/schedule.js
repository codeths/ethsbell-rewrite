getel('upload').addEventListener('click', async () => {
	const file = (getel('cfg')).files[0];
	if (!file) {
		getel('status').innerHTML = 'No file provided';
		return;
	}

	const text = await file.text();

	let data;
	try {
		data = JSON.parse(text);
	} catch {
		getel('status').innerHTML = 'Invalid json';
		return;
	}

	localStorage.setItem('schedule', text);
	getel('status').innerHTML = 'OK';
});

function getel(id) {
	const selector = `#${id}`;
	return document.querySelector(selector);
}

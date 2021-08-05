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
	update_working();
	getel('status').innerHTML = 'OK';
});

function getel(id) {
	const selector = `#${id}`;
	return document.querySelector(selector);
}

const working_copy = JSON.parse(localStorage.getItem('schedule')) || {
	schedule: {},
	foreground_color: '#1a2741',
	background_color: '#c34614',
	foreground_text_color: '#ffffff',
	background_text_color: '#ffffff',
};

getel('class_id').addEventListener('change', switchClass);
switchClass();
populate();

function switchClass() {
	const id = getel('class_id').value;
	if (id === '') {
		return;
	}

	const data = working_copy.schedule[id];
	getel('name').value = data ? data.name : id;
	getel('url').value = data ? data.url || '' : '';
}

getel('name').addEventListener('input', update_working);
getel('url').addEventListener('input', update_working);
getel('foreground_color').addEventListener('input', update_working);
getel('background_color').addEventListener('input', update_working);
getel('foreground_text_color').addEventListener('input', update_working);
getel('background_text_color').addEventListener('input', update_working);

function populate() {
	getel('foreground_color').value = working_copy.foreground_color;
	getel('background_color').value = working_copy.background_color;
	getel('foreground_text_color').value = working_copy.foreground_text_color;
	getel('background_text_color').value = working_copy.background_text_color;
}

function update_working() {
	working_copy.foreground_color = getel('foreground_color').value;
	working_copy.background_color = getel('background_color').value;
	working_copy.foreground_text_color = getel('foreground_text_color').value;
	working_copy.background_text_color = getel('background_text_color').value;
	const id = getel('class_id').value;
	let name = getel('name').value;
	if (name.length === 0) {
		name = undefined;
	}

	let url = getel('url').value;
	if (url.length === 0) {
		url = undefined;
	} else if (url.indexOf('://') == -1 {
		url = 'https://' + url;
	}

	if (name) {
		working_copy.schedule[id] = {
			name,
			url,
		};
	} else {
		delete working_copy.schedule[id];
	}
}

getel('save').addEventListener('click', () => {
	update_working();
	localStorage.setItem('schedule', JSON.stringify(working_copy));
	window.open(window.location);
});

getel('download').addEventListener('click', () => {
	alert('doesn\'t work yet');
});

getel('default-page').addEventListener('change', event => {
	working_copy.default_page = event.target.value;
});

getel('reset-colors').addEventListener('click', () => {
	Object.assign(working_copy, {
		foreground_color: '#1a2741',
		background_color: '#c34614',
		foreground_text_color: '#ffffff',
		background_text_color: '#ffffff',
	});
});

getel('reset-schedule').addEventListener('click', () => {
	working_copy.schedule = {};
});


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
	populated = false;
	update_working();
	getel('status').innerHTML = 'OK';
});

function getel(id) {
	const selector = `#${id}`;
	return document.querySelector(selector);
}

let working_copy = JSON.parse(localStorage.getItem('schedule')) || {
	schedule: {},
	foreground_color: "#1a2741",
	background_color: "#c34614",
	foreground_text_color: "#ffffff",
	background_text_color: "#ffffff"
}

getel('class_id').addEventListener('change', switchClass)
switchClass()
populate()

function switchClass() {
	let id = getel('class_id').value
	if (id === '') {
		return
	}

	let data = config.schedule[id]
	getel('name').value = data ? data.name : id
	getel('url').value = data ? data.url || '' : ''
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
	let id = getel('class_id').value
	let name = getel('name').value
	if (name.length == 0) {name = undefined;}
	let url = getel('url').value
	if (url.length == 0) { url = undefined; }
	
	if (!name) {
		delete working_copy.schedule[id];
	} else {
		working_copy.schedule[id] = {
			name,
			url
		}
	}
}

getel('save').addEventListener('click', () => {
	update_working()
	localStorage.setItem('schedule', JSON.stringify(working_copy))
})

getel('download').addEventListener('click', () => {
	alert('doesn\'t work yet')
})

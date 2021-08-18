const defaultConfig = {
	schedule: {},
	foreground_color: '#1a2741',
	background_color: '#c34614',
	foreground_text_color: '#ffffff',
	background_text_color: '#ffffff',
	include_period_name: true,
};

function getConfig() {
	return Object.assign(defaultConfig, JSON.parse(localStorage.getItem('schedule')) || '{}');
}

getel('upload').addEventListener('click', async () => {
	const file = (getel('cfg')).files[0];
	if (!file) {
		alert('Please select a file');
		return;
	}

	const text = await file.text();

	let data;
	try {
		data = JSON.parse(text);
	} catch {
		alert('Invalid file contents');
		return;
	}

	localStorage.setItem('schedule', text);
	broadcastConfigToExtension();
	populate();
	setTheme();
	alert('Settings imported.');
});

getel('cfg').addEventListener('change', () => {
	getel('upload').disabled = getel('cfg').files.length !== 1;
});

getel('include_period_name').addEventListener('change', () => {
	editPeriodExample();
});

function getel(id) {
	const selector = `#${id}`;
	return document.querySelector(selector);
}

function populate() {
	const initial = getConfig();

	getel('foreground_color').value = initial.foreground_color;
	getel('background_color').value = initial.background_color;
	getel('foreground_text_color').value = initial.foreground_text_color;
	getel('background_text_color').value = initial.background_text_color;
	getel('default-page').value = initial.default_page || '/';
	getel('cfg_body').innerHTML = [
		{name: 'Early Bird', code: 'EarlyBird'},
		{name: 'Lunch 1', code: 'Lunch 1'},
		{name: 'Lunch 2', code: 'Lunch 2'},
		{name: 'Block 1', code: 'Block1'},
		{name: 'Block 2', code: 'Block2'},
		{name: 'Block 3', code: 'Block3'},
		{name: 'Block 4', code: 'Block4'},
		{name: 'Block 5', code: 'Block5'},
		{name: 'Block 6', code: 'Block6'},
		{name: 'Block 7', code: 'Block7'},
		{name: 'Block 8', code: 'Block8'},
	].map(x => `<tr><td>${x.name}</td><td><input data-for="name" data-period="${x.code}" value="${initial.schedule[x.code]?.name || ''}"}></td><td><input type="url" data-for="url" data-period="${x.code}" value="${initial.schedule[x.code]?.url || ''}"}></td></tr>`).join('\n');
	getel('include_period_name').checked = initial.include_period_name || initial.include_period_name === undefined;
	editPeriodExample();
}

function editPeriodExample() {
	// Sometimes I just like to watch the world burn.
	getel('example_name').innerHTML = `<a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">${getel('include_period_name').checked ? 'Block 1 - Custom Name' : 'Custom Name'}</a>`;
}

populate();

getel('save-schedule').addEventListener('click', () => {
	const schedule = {};
	for (const x of [...document.querySelectorAll('input[data-for=name]')]) {
		const period = x.getAttribute('data-period');
		if (!schedule[period]) {
			schedule[period] = {};
		}

		schedule[period].name = x.value || null;
	}

	for (const x of [...document.querySelectorAll('input[data-for=url]')]) {
		const period = x.getAttribute('data-period');
		if (!schedule[period]) {
			schedule[period] = {};
		}

		schedule[period].url = x.value || null;
	}

	const data = getConfig();

	data.schedule = schedule;
	data.include_period_name = getel('include_period_name').checked;

	localStorage.setItem('schedule', JSON.stringify(data));
	broadcastConfigToExtension();
	alert('Saved.');
});

getel('save-default').addEventListener('click', () => {
	const data = getConfig();

	data.default_page = getel('default-page').value;

	localStorage.setItem('schedule', JSON.stringify(data));
	broadcastConfigToExtension();
	alert('Saved.');
});

getel('save-colors').addEventListener('click', () => {
	const data = getConfig();

	data.foreground_color = getel('foreground_color').value;
	data.background_color = getel('background_color').value;
	data.foreground_text_color = getel('foreground_text_color').value;
	data.background_text_color = getel('background_text_color').value;

	localStorage.setItem('schedule', JSON.stringify(data));
	broadcastConfigToExtension();
	setTheme();
	alert('Saved.');
});

getel('download').addEventListener('click', () => {
	const blob = new Blob([localStorage.getItem('schedule')], {
		type: 'application/json',
	});

	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = 'ethsbell.json';
	a.click();
	URL.revokeObjectURL(url);
});

getel('reset-colors').addEventListener('click', () => {
	if (!confirm('Are you sure you want to reset?')) {
		return;
	}

	const data = getConfig();

	data.foreground_color = '#1a2741';
	data.background_color = '#c34614';
	data.foreground_text_color = '#ffffff';
	data.background_text_color = '#ffffff';

	localStorage.setItem('schedule', JSON.stringify(data));
	broadcastConfigToExtension();
	setTheme();
	populate();
});

getel('reset-schedule').addEventListener('click', () => {
	if (!confirm('Are you sure you want to reset?')) {
		return;
	}

	const data = getConfig();

	data.schedule = {};

	localStorage.setItem('schedule', JSON.stringify(data));
	broadcastConfigToExtension();
	populate();
});

getel('instructions_toggle').addEventListener('click', () => {
	getel('instructions').classList.toggle('show');
});

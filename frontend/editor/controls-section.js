let data;

$('pull').addEventListener('click', pull);

async function pull() {
	if (
		data
		&& !confirm(
			'This will overwrite any existing changes. Are you sure you want to continue?',
		)
	) {
		return;
	}

	data = await (await fetch('../api/v1/spec')).json();
	update_view();
}

$('push').addEventListener('click', async () => {
	const res = await fetch('../api/v1/spec', {
		method: 'POST',
		body: JSON.stringify(data),
		headers: {
			Authorization,
		},
	});
	if (res.ok) {
		alert('Saved!');
	} else {
		alert(`Error: ${res.status} ${res.statusText}`);
	}
});

$('update').addEventListener('click', async () => {
	const res = await fetch('../api/v1/update', {
		method: 'POST',
		headers: {
			Authorization,
		},
	});
	if (res.ok) {
		alert('Saved!');
	} else {
		alert(`Error: ${res.status} ${res.statusText}`);
	}
});

$('select_schedule').addEventListener('change', () => {
	update_view();
});

let schedule_name;
let period_index;
let schedule;
let period;

function update_view() {
	// Fill drop-down menus
	schedule_name = $('select_schedule').value;
	$('select_schedule').innerHTML = '';
	for (const schedule_type of Object.keys(data.schedule_types).sort((a, b) =>
		data.schedule_types[a].friendly_name.localeCompare(
			data.schedule_types[b].friendly_name,
		),
	)) {
		$(
			'select_schedule',
		).innerHTML += `<option value="${schedule_type}">${data.schedule_types[schedule_type].friendly_name}</option>`;
	}

	if (!schedule_name) {
		schedule_name = $('select_schedule').value;
	}

	$('select_schedule').value = schedule_name;
	$('add_period').disabled = !schedule_name;

	// Load current data
	schedule = schedule_name && data.schedule_types[schedule_name];

	for (const el of document.querySelectorAll('.schedule_setting')) {
		el.disabled = !schedule;
	}

	if (schedule) {
		displayPeriods();
		// Fill form fields
		$('schedule_friendly_name').value = schedule.friendly_name;
		$('schedule_code').value = schedule_name;
		$('schedule_color').value = `#${schedule.color
			.map(x => `0${x.toString(16)}`.slice(-2))
			.join('')}`;
		$('schedule_hide').checked = schedule.hide;
		$('schedule_regex').value = schedule.regex;
	} else {
		$('periods').innerHTML = '';
	}

	$('calendars').value = data.calendar_urls.slice().reverse().join('\n') + '\n';
	$('calendars').setAttribute('rows', $('calendars').value.match(/\n/g).length + 1);

	$('typical_schedule').innerHTML = '';

	for (const [i, day] of ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'].entries()) {
		const label = document.createElement('label');
		label.innerHTML = day;
		label.setAttribute('for', `typical_schedule_${i}`);
		$('typical_schedule').append(label);

		const select = document.createElement('select');
		select.innerHTML = $('select_schedule').innerHTML;
		select.value = data.typical_schedule[i];
		select.addEventListener('change', () => {
			data.typical_schedule[i] = select.value;
		});
		$('typical_schedule').append(select);

		const br = document.createElement('br');
		$('typical_schedule').append(br);
	}
}

// Schedule add, copy, and remove
$('add_schedule').addEventListener('click', () => {
	const new_name = codeStr(
		prompt(
			'Set the internal name for the new schedule type (like no_school or orange_day)',
		),
	);
	if (data.schedule_types[new_name]) {
		return alert('A schedule already exists with this code.');
	}

	if (!new_name) {
		return;
	}

	data.schedule_types[new_name] = {
		friendly_name: 'New Schedule',
		periods: [],
		regex: '',
		color: [255, 255, 255],
		hide: false,
	};

	update_view();
	$('select_schedule').value = new_name;
	update_view();
});
$('copy_schedule').addEventListener('click', () => {
	const new_name = codeStr(
		prompt(
			'Set the internal name for the newly copied schedule (like no_school or orange_day)',
		),
	);
	if (data.schedule_types[new_name]) {
		return alert('A schedule already exists with this code.');
	}

	if (!new_name) {
		return;
	}

	data.schedule_types[new_name] = Object.assign(
		{},
		data.schedule_types[schedule_name],
	);
	data.schedule_types[new_name].friendly_name = `Copy of ${schedule_name}`;

	update_view();
	$('select_schedule').value = new_name;
	update_view();
});
$('remove_schedule').addEventListener('click', () => {
	const response = confirm(`Do you really want to delete ${schedule_name}?`);
	if (response) {
		delete data.schedule_types[schedule_name];
		$('select_schedule').value = '';
		update_view();
	}
});

// Period add, copy, and remove
$('add_period').addEventListener('click', () => {
	if (schedule) {
		schedule.periods.push({
			friendly_name: '',
			start: '00:00:00',
			end: '00:00:00',
			kind: {Class: ''},
		});
	}

	displayPeriods();
});

// Handle form changes
$('schedule_friendly_name').addEventListener('change', event => {
	event.target.value = event.target.value.trim();

	if (!event.target.value) {
		alert('This value is required.');
		update_view();
		return;
	}

	schedule.friendly_name = event.target.value;
	update_view();
});
$('schedule_code').addEventListener('change', event => {
	const v = codeStr(event.target.value);
	event.target.value = v;

	if (!v) {
		alert('This value is required.');
		update_view();
		return;
	}

	if (v === schedule_name) {
		return;
	}

	if (!confirm('Changing the code of existing schedules can cause issues and is generally not recommended. Are you sure you want to change it?')) {
		event.target.value = schedule_name;
		return;
	}

	if (data.schedule_types[v]) {
		alert('A schedule already exists with this code.');
		update_view();
		return;
	}

	if (!v) {
		event.target.value = schedule_name;
		return;
	}

	data.schedule_types[v] = schedule;
	delete data.schedule_types[schedule_name];
	schedule_name = v;
	schedule = data.schedule_types[v];
	update_view();
	$('select_schedule').value = v;
	update_view();
});

$('schedule_color').addEventListener('change', event => {
	schedule.color = event.target.value
		.slice(1)
		.match(/.{2}/g)
		.map(x => Number.parseInt(x, 16));
});

$('schedule_hide').addEventListener('change', event => {
	schedule.hide = event.target.checked;
});

$('schedule_regex').addEventListener('change', event => {
	try {
		const regex = new RegExp(event.target.value);
	} catch {
		alert('Invalid regular expression.');
		update_view();
		return;
	}

	schedule.regex = event.target.value;
});

$('calendars').addEventListener('change', event => {
	const items = event.target.value.split('\n').map(x => x.trim()).filter(x => x).reverse();

	if (items.some(x => {
		try {
			const url = new URL(x);
			return false;
		} catch {
			return true;
		}
	})) {
		alert('Invalid calendar URL.');
		update_view();
		return;
	}

	data.calendar_urls = items;

	update_view();
});

$('calendars').addEventListener('input', event => {
	$('calendars').setAttribute('rows', $('calendars').value.match(/\n/g).length + 1);
});

const template = $('period_settings');

function displayPeriods() {
	$('periods').innerHTML = '';
	schedule.periods.forEach((period, i, arr) => {
		const new_element = document.createElement('div');
		new_element.classList.add('period');
		new_element.innerHTML = template.innerHTML;

		for (const element of new_element.querySelectorAll('[id]')) {
			element.id += `_${i}`;
		}

		for (const element of new_element.querySelectorAll('[for]')) {
			element.for += `_${i}`;
		}

		const $$ = id => new_element.querySelector(`#${id}_${i}`);

		$$('period_friendly_name').value = period.friendly_name;
		$$('period_start').value = period.start;
		$$('period_end').value = period.end;
		if (typeof period.kind === 'string') {
			$$('period_kind').value = period.kind;
			$$('period_class_index').disabled = true;
		} else {
			$$('period_kind').value = 'Class';
			$$('period_class_index').disabled = false;
			$$('period_class_index').value = period.kind.Class;
		}

		$$('move_up').disabled = i === 0;
		$$('move_down').disabled = i === arr.length - 1;

		$$('period_friendly_name').addEventListener('change', event => {
			event.target.value = event.target.value.trim();

			if (!event.target.value) {
				alert('This value is required.');
				update_view();
				return;
			}

			period.friendly_name = event.target.value;
		});
		$$('period_start').addEventListener('change', event => {
			event.target.value = event.target.value.trim();

			if (!event.target.value) {
				alert('This value is required.');
				update_view();
				return;
			}

			if (/^\d{2}:\d{2}$/.test(event.target.value)) {
				event.target.value += ':00';
			}

			period.start = event.target.value;
		});
		$$('period_end').addEventListener('change', event => {
			event.target.value = event.target.value.trim();

			if (!event.target.value) {
				alert('This value is required.');
				update_view();
				return;
			}

			if (/^\d{2}:\d{2}$/.test(event.target.value)) {
				event.target.value += ':00';
			}

			period.end = event.target.value;
		});
		$$('period_kind').addEventListener('change', event => {
			if (event.target.value === 'Class') {
				period.kind = {
					Class: '',
				};
				$$('period_class_index').disabled = false;
			} else {
				period.kind = event.target.value;
				$$('period_class_index').value = '';
				$$('period_class_index').disabled = true;
			}
		});
		$$('period_class_index').addEventListener('change', event => {
			event.target.value = event.target.value.trim();

			if (!event.target.value) {
				alert('This value is required.');
				update_view();
				return;
			}

			if (event.target.value === period.kind.Class) {
				return;
			}

			if (period.kind.Class && !confirm('Changing the code of existing periods can cause issues and is generally not recommended. Are you sure you want to change it?')) {
				event.target.value = period.kind.Class;
				return;
			}

			period.kind = {
				Class: event.target.value,
			};
		});
		$$('move_up').addEventListener('click', () => {
			[arr[i], arr[i - 1]] = [arr[i - 1], arr[i]];
			displayPeriods();
		});
		$$('move_down').addEventListener('click', () => {
			[arr[i], arr[i + 1]] = [arr[i + 1], arr[i]];
			displayPeriods();
		});
		$$('remove_period').addEventListener('click', () => {
			schedule.periods.splice(i, 1);
			displayPeriods();
		});

		$('periods').append(new_element);
	});
}

function codeStr(string) {
	return (string || '')
		.toLowerCase()
		.trim()
		.replace(/\s/g, '_');
}

$('export').addEventListener('click', () => {
	const blob = new Blob([JSON.stringify(data)], {
		type: 'application/json',
	});

	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = 'def.json';
	a.click();
	URL.revokeObjectURL(url);
});

$('import').addEventListener('change', async event => {
	const file = event.target.files[0];
	if (!file) {
		return;
	}

	const text = await file.text();

	try {
		const json = JSON.parse(text);
		data = json;
	} catch {
		alert('Invalid file contents');
	}

	event.target.value = '';
	update_view();

	alert('Imported settings.');
});

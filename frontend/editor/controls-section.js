let data;

function check_data(error_span) {
	if (!data) {
		$(error_span).innerHTML = 'No data';
		throw new Error('No data');
	}
}

$('pull').addEventListener('click', pull);

async function pull() {
	data = await (await fetch('../api/v1/spec')).json();
	update_view();
}

window.pull = pull;

$('push').addEventListener('click', async () => {
	await fetch('../api/v1/spec', {
		method: 'POST',
		body: JSON.stringify(data),
		headers: {
			Authorization,
		},
	});
});

$('update').addEventListener('click', async () => {
	await fetch('../api/v1/update', {
		method: 'POST',
		headers: {
			Authorization,
		},
	});
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
	schedule_name =
		$('select_schedule').value || Object.keys(data.schedule_types)[0];
	$('select_schedule').innerHTML = '';
	for (const schedule_type of Object.keys(data.schedule_types)) {
		$(
			'select_schedule',
		).innerHTML += `<option value="${schedule_type}">${data.schedule_types[schedule_type].friendly_name}</option>`;
	}

	$('select_schedule').value = schedule_name;
	// Load current data
	schedule = data.schedule_types[schedule_name];
	displayPeriods();
	// Fill form fields
	$('schedule_friendly_name').value = schedule.friendly_name;
	$('schedule_regex').value = schedule.regex;
}

// Schedule add, copy, and remove
$('add_schedule').addEventListener('click', () => {
	check_data('controls_error');
	const new_name = prompt(
		'Set the internal name for the new schedule type (like no_school or orange_day)',
	);
	if (new_name.length > 0) {
		data.schedule_types[new_name] = {
			friendly_name: new_name,
			periods: [],
			regex: '',
		};
	}

	update_view();
	$('select_schedule').value = new_name;
	update_view();
});
$('copy_schedule').addEventListener('click', () => {
	check_data('controls_error');
	const new_name = prompt(
		'Set the internal name for the newly copied schedule (like no_school or orange_day)',
	);
	if (new_name.length > 0) {
		data.schedule_types[new_name] = JSON.parse(
			JSON.stringify(data.schedule_types[schedule_name]),
		);
		data.schedule_types[new_name].friendly_name = new_name;
	}

	update_view();
	$('select_schedule').value = new_name;
	update_view();
});
$('remove_schedule').addEventListener('click', () => {
	check_data('controls_error');
	const response = confirm(`Do you really want to delete ${schedule_name}?`);
	if (response) {
		delete data.schedule_types[schedule_name];
		$('select_schedule').value = Object.keys(data.schedule_types)[0];
		update_view();
	}
});

// Period add, copy, and remove
$('add_period').addEventListener('click', () => {
	if (schedule)
		schedule.periods.push({
			friendly_name: '',
			start: '00:00:00',
			end: '00:00:00',
			kind: {Class: ''},
		});

	displayPeriods();
});

// Handle form changes
$('schedule_friendly_name').addEventListener('change', event => {
	check_data('controls_error');
	data.schedule_types[schedule_name].friendly_name = event.target.value;
});
$('schedule_regex').addEventListener('change', event => {
	check_data('controls_error');
	data.schedule_types[schedule_name].regex = event.target.value;
});

let template = $('period_settings');

function displayPeriods() {
	$('periods').innerHTML = '';
	schedule.periods.forEach((period, i, arr) => {
		const new_element = document.createElement('div');
		new_element.classList.add('period');
		new_element.innerHTML = template.innerHTML;

		[...new_element.querySelectorAll('[id]')].forEach(element => {
			element.id = element.id + `_${i}`;
		});
		[...new_element.querySelectorAll('[for]')].forEach(element => {
			element.for = element.for + `_${i}`;
		});

		let $$ = id => new_element.querySelector(`#${id}_${i}`);

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
			period.friendly_name = event.target.value;
		});
		$$('period_start').addEventListener('change', event => {
			period.start = event.target.value;
		});
		$$('period_end').addEventListener('change', event => {
			period.end = event.target.value;
		});
		$$('period_kind').addEventListener('change', event => {
			if (event.target.value === 'Class') {
				period.kind = {
					Class: Number.parseInt($('period_class_index').value, 10),
				};
				$('period_class_index').disabled = false;
			} else {
				period.kind = event.target.value;
				$('period_class_index').disabled = true;
			}
		});
		$$('period_class_index').addEventListener('change', event => {
			period.periods[period_index].kind.Class = Number.parseInt(
				event.target.value,
				10,
			);
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

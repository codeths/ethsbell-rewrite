const tableElement = document.querySelector('#table');
const tbodyElement = document.querySelector('#tbody');
const scheduleSelect = document.querySelector('#schedule-select');

function buildTable(data) {
	tbodyElement.innerHTML = '';

	for (const period of data) {
		const tr = document.createElement('tr');
		tr.innerHTML = `<td>${period.friendly_name}</td>
						<td>${human_time(period.start)} - ${human_time(period.end)}</td>
						<td>${human_time_left(period.end, period.start, true)}</td>`;
		tbodyElement.appendChild(tr);
	}
}

let schedules = {};
let currentSchedule = {};

async function getToday() {
	const today = await get('/api/v1/today');
	if (!today) return;

	currentSchedule = today;

	buildTable(currentSchedule.periods);
	
	return;
}


async function getSchedules() {
	const today = await get('/api/v1/spec');
	if (!today) return;

	schedules = today.schedule_types;

	scheduleSelect.innerHTML = '<option value="" disabled selected>Select a Period</option>';

	for (let schedule in schedules) {
		let option = document.createElement('option');
		option.value = schedule;
		if (currentSchedule[1] == schedule) option.selected = true;
		option.innerHTML = schedules[schedule].friendly_name;
		scheduleSelect.appendChild(option);
	}

	return;
}

scheduleSelect.addEventListener('change', function() {
	let selected = scheduleSelect.value;
	if (schedules[selected]) {
		buildTable(schedules[selected].periods);
	}
});

(async () => {
	await getToday();
	await getSchedules();
})();
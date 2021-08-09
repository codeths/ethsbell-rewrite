const CALENDAR_WEEKS = 4;

const tableElement = document.querySelector('#table');
const tbodyElement = document.querySelector('#tbody');
const scheduleSelect = document.querySelector('#schedule-select');
const dateSelect = document.querySelector('#date-select');
const calendarTable = document.querySelector('#calendar-table');

let schedules = {};
let currentSchedule = {};

getel('today').addEventListener('click', () => {
	dateSelect.value = date_to_string(current_date(), false);
	getDate(dateSelect.valueAsDate);
});

getel('previous').addEventListener('click', () => {
	dateSelect.valueAsDate = new Date(dateSelect.valueAsDate.getTime() - 1000 * 60 * 60 * 24);
	getDate(dateSelect.valueAsDate);
});

getel('next').addEventListener('click', () => {
	dateSelect.valueAsDate = new Date(dateSelect.valueAsDate.getTime() + 1000 * 60 * 60 * 24);
	getDate(dateSelect.valueAsDate);
});

async function getDate(date = current_date(), setCurrent = false) {
	const dateString = date_to_string(date, false);

	const day = await get(`/api/v1/on/${dateString}`);
	if (!day) {
		return;
	}

	place_boxes(day, date, true, setCurrent || dateString === date_to_string(current_date(), false));

	if (setCurrent) {
		currentSchedule = day;
	}

	for (const option of [...scheduleSelect.querySelectorAll('option')]) {
		if (option.textContent === day.friendly_name) {
			option.selected = true;
		}
	}
}

async function getSchedules() {
	const today = await get('/api/v1/spec');
	if (!today) {
		return;
	}

	schedules = today.schedule_types;

	scheduleSelect.innerHTML = '<option value="" disabled selected>Select a Schedule</option>';

	for (const schedule of Object.keys(schedules).sort((a, b) => {
		if (schedules[a].hide && !schedules[b].hide || schedules[a].periods.length === 0) {
			return 1;
		}

		if (!schedules[a].hide && schedules[b].hide || schedules[b].periods.length === 0) {
			return -1;
		}

		return schedules[a].friendly_name.localeCompare(schedules[b].friendly_name);
	})) {
		const option = document.createElement('option');
		option.value = schedule;
		if (currentSchedule.friendly_name === schedules[schedule].friendly_name) {
			option.selected = true;
		} else {
			option.hidden = schedules[schedule].hide || schedules[schedule].periods.length === 0;
		}

		option.innerHTML = schedules[schedule].friendly_name;
		scheduleSelect.append(option);
	}
}

async function getScheduleList(start, end) {
	const scheduleList = await get(`/api/v1/schedule/from/${date_to_string(start, false)}/to/${date_to_string(end, false)}`);
	if (!schedules) {
		return;
	}

	const days = scheduleList.map((scheduleCode, i) => {
		const date = new Date(start);
		date.setDate(date.getDate() + i);
		let schedule = schedules[scheduleCode] || null;
		if (!schedule) {
			try {
				schedule = JSON.parse(scheduleCode);
			} catch {}
		}

		const name = schedule?.friendly_name || null;
		const backgroundColor = schedule?.color ? bytes_to_color(schedule.color) : '#FFFFFF';
		const textColor = black_or_white(backgroundColor);
		return {
			code: scheduleCode,
			data: schedule,
			name,
			date,
			backgroundColor,
			textColor,
		};
	});

	return days;
}

scheduleSelect.addEventListener('change', () => {
	const selected = scheduleSelect.value;
	if (schedules[selected]) {
		place_boxes(schedules[selected], current_date(), true, scheduleSelect.options[scheduleSelect.selectedIndex].text === currentSchedule.friendly_name);
	}
});

dateSelect.value = date_to_string(current_date(), false);

dateSelect.addEventListener('change', () => {
	getDate(dateSelect.valueAsDate);
});

const startOfWeek = current_date();
startOfWeek.setHours(0, 0, 0, 0);
startOfWeek.setDate(startOfWeek.getDate() - startOfWeek.getDay());
const endOfNextWeek = new Date(startOfWeek);
endOfNextWeek.setDate(endOfNextWeek.getDate() + (CALENDAR_WEEKS * 7));

(async () => {
	await getDate(current_date(), true);
	await getSchedules();
	const scheduleList = await getScheduleList(startOfWeek, endOfNextWeek);
	calendarTable.innerHTML = '';
	const table = document.createElement('table');
	const tbody = document.createElement('tbody');

	let index = 0;
	for (let row = 0; row < CALENDAR_WEEKS; row++) {
		const tr = document.createElement('tr');
		for (let col = 0; col < 7; col++) {
			const td = document.createElement('td');
			const day = scheduleList[index];
			const humanDate = day.date.toLocaleDateString('en-US', {
				weekday: 'short',
				month: 'long',
				day: 'numeric',
				timeZone: 'America/Chicago',
			});
			td.classList.add('day');
			td.innerHTML = `<span class="day-name">${humanDate}</span><div class="day-schedule">${day.name}</div>`;
			if (day.date.toLocaleDateString() === current_date().toLocaleDateString()) {
				td.classList.add('today');
			}

			td.querySelector('.day-schedule').style.backgroundColor = day.backgroundColor;
			td.querySelector('.day-schedule').style.color = day.textColor;
			if (day.data.periods.length === 0) {
				td.classList.add('no-periods');
			} else {
				td.addEventListener('click', () => {
					place_boxes(day.data, day.date, true, day.date.toLocaleDateString() === current_date().toLocaleDateString());
					scheduleSelect.value = day.code;
					dateSelect.value = date_to_string(day.date);
				});
			}

			tr.append(td);
			index++;
		}

		tbody.append(tr);
	}

	table.append(tbody);
	calendarTable.append(table);
})();

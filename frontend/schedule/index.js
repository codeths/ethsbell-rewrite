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
	getDate(dateSelect.value);
});

getel('previous').addEventListener('click', () => {
	dateSelect.valueAsDate = new Date(dateSelect.valueAsDate.getTime() - 1000 * 60 * 60 * 24);
	getDate(dateSelect.value);
});

getel('next').addEventListener('click', () => {
	dateSelect.valueAsDate = new Date(dateSelect.valueAsDate.getTime() + 1000 * 60 * 60 * 24);
	getDate(dateSelect.value);
});

async function getDate(date = current_date(), setCurrent = false) {
	const dateString = typeof date === 'string' ? date : date_to_string(date, false);

	const day = await get(`/api/v1/on/${dateString}`);
	if (!day) {
		return;
	}

	place_boxes(day, typeof date === 'string' ? date_string_to_date(date, true) : date, true, setCurrent || dateString === date_to_string(current_date(), false));

	if (setCurrent) {
		currentSchedule = day;
	}

	for (const option of scheduleSelect.querySelectorAll('option')) {
		if (option.textContent === day.friendly_name) {
			option.selected = true;
		}
	}
}

let scheduleArray = [];

function setScheduleValue(code) {
	scheduleSelect.innerHTML = '';
	for (const schedule of scheduleArray.filter(x => !x.hide || code === x.code)) {
		const option = document.createElement('option');
		option.value = schedule.code;
		if (code === schedule.code) {
			option.selected = true;
		}

		if (schedule.hide || schedule.periods.length === 0) {
			option.disabled = true;
		}

		option.innerHTML = schedule.friendly_name;
		scheduleSelect.append(option);
	}
}

async function getSchedules() {
	const today = await get('/api/v1/spec');
	if (!today) {
		return;
	}

	schedules = today.schedule_types;

	scheduleSelect.innerHTML = '<option value="" disabled selected>Select a Schedule</option>';

	scheduleArray = Object.keys(schedules).map(x => Object.assign({code: x}, schedules[x])).sort((a, b) => {
		if ((!a.hide && b.hide) || a.periods.length === 0) {
			return 1;
		}

		if ((a.hide && !b.hide) || b.periods.length === 0) {
			return -1;
		}

		return a.friendly_name.localeCompare(b.friendly_name);
	});

	const currentScheduleCode = currentSchedule && scheduleArray.find(x => x.friendly_name === currentSchedule.friendly_name)?.code || null;

	setScheduleValue(currentScheduleCode);
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
	getDate(dateSelect.value);
});

const startOfWeek = current_date();
startOfWeek.setHours(0, 0, 0, 0);
startOfWeek.setDate(startOfWeek.getDate() - startOfWeek.getDay());
const endOfNextWeek = new Date(startOfWeek);
endOfNextWeek.setDate(endOfNextWeek.getDate() + CALENDAR_WEEKS * 7);

dateSelect.min = date_to_string(startOfWeek, false);
dateSelect.max = date_to_string(new Date(endOfNextWeek.getTime() - 60 * 60 * 24), false);

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
					setScheduleValue(day.code);
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

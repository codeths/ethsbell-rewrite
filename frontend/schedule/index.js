const CALENDAR_WEEKS = 4;

const tableElement = document.querySelector('#table');
const tbodyElement = document.querySelector('#tbody');
const scheduleSelect = document.querySelector('#schedule-select');
const dateSelect = document.querySelector('#date-select');
const calendarTable = document.querySelector('#calendar-table');

function buildTable(data) {
	tbodyElement.innerHTML = '';

	for (const period of data) {
		const tr = document.createElement('tr');
		tr.innerHTML = `<td>${period.friendly_name}</td>
						<td>${human_time(period.start)} - ${human_time(period.end)}</td>
						<td>${human_time_left(period.end, period.start, true)}</td>`;
		tbodyElement.append(tr);
	}
}

let schedules = {};
let currentSchedule = {};

async function getDate(date = current_date(), setCurrent = false) {
	if (date instanceof Date) date = date_to_string(date);
	const day = await get(`/api/v1/on/${date}`);
	if (!day) {
		return;
	}

	if (setCurrent) {
		currentSchedule = day;
	}

	buildTable(day.periods);

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

	for (const schedule of Object.keys(schedules)) {
		const option = document.createElement('option');
		option.value = schedule;
		if (currentSchedule.friendly_name === schedules[schedule].friendly_name) {
			option.selected = true;
		}

		option.innerHTML = schedules[schedule].friendly_name;
		scheduleSelect.append(option);
	}
}

async function getScheduleList(start, end) {
	const scheduleList = await get(`/api/v1/schedule/from/${date_to_string(start)}/to/${date_to_string(end)}`);
	if (!schedules) {
		return;
	}

	const days = scheduleList.map((scheduleCode, i) => {
		const date = new Date(start);
		date.setDate(date.getDate() + i);
		const schedule = schedules[scheduleCode] || null;
		const name = schedule?.friendly_name || null;
		const backgroundColor = schedule?.color ? bytes_to_color(schedule.color) : '#FFFFFF';
		const textColor = black_or_white(backgroundColor);
		return {
			code: scheduleCode,
			schedule: schedule?.periods || null,
			name,
			date,
			backgroundColor,
			textColor
		};
	});

	return days;
}

scheduleSelect.addEventListener('change', () => {
	const selected = scheduleSelect.value;
	if (schedules[selected]) {
		buildTable(schedules[selected].periods);
	}
});

dateSelect.valueAsDate = current_date();

dateSelect.addEventListener('change', () => {
	const date = dateSelect.value;
	getDate(date);
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
                timeZone: 'America/Chicago'
            });
            td.classList.add('day');
            td.innerHTML = `<span class="day-name">${humanDate}</span><div class="day-schedule">${day.name}</div>`;
			if (day.date.toLocaleDateString() == new Date().toLocaleDateString()) td.classList.add('today');
			td.querySelector('.day-schedule').style.backgroundColor = day.backgroundColor;
			td.querySelector('.day-schedule').style.color = day.textColor;
			td.addEventListener('click', () => {
				buildTable(day.schedule);
				scheduleSelect.value = day.code;
				dateSelect.value = date_to_string(day.date);
			});
			tr.appendChild(td);
            index++;
        }
        tbody.appendChild(tr);
    }

	table.appendChild(tbody);
    calendarTable.appendChild(table);
})();

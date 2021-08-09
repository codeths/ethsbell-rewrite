let all_data;

const calendarElement = document.querySelector('#event-wrapper');
const nowBarElement = document.querySelector('#calendar-today #now');
const preferred_event_min_width = 200;
const min_visible_width = 50;
const min_event_height = 47.5;
const padding = 5;
const pixels_per_minute = 1.5;
let startDate;
let startTime;
let endDate;
let endTime;
let events;
let showNowBar = true;
let backgroundColor = 'rgb(255, 255, 255)';
let textColor = 'black';

/// Place period boxes for a list of periods.
function place_boxes(data_unprocessed, date = current_date(), force = false, today = true) {
	showNowBar = today;

	calendarElement.innerHTML = '';
	if (data_unprocessed && (!events || force)) {
		calendarElement.style.height = 'auto';
		if (!data_unprocessed.periods[0]) {
			updateNowBar();
			return;
		}

		const data = replace_period(data_unprocessed.periods).filter(v => v);
		startDate = date_from_api(data[0].start, date);
		startTime = startDate.getTime() / 1000;
		endDate = date_from_api(data[data.length - 1].end, date);
		endTime = endDate.getTime() / 1000;
		updateNowBar();
		const sInDay = endTime - startTime;
		let containerHeight = sInDay / 60 * pixels_per_minute;
		const lastColHeight = {};

		// Resolve rows so everything is mutually non-intersecting.
		events = [];

		for (const [i, period] of data.sort((a, b) => date_from_api(a.start, date) - date_from_api(b.start, date)).entries()) {
			// Set up variables
			const start = date_from_api(period.start, date).getTime() / 1000;
			const end = date_from_api(period.end, date).getTime() / 1000;
			const duration = end - start;
			let heightChange = 0;

			const startPos = ((start - startTime) / 60) * pixels_per_minute;
			let height = (duration / 60) * pixels_per_minute;
			const endPos = startPos + height;
			if (height < min_event_height) {
				heightChange = (min_event_height - height);
				height = min_event_height;
			}

			let col = 0;

			while (events.filter(event => event.col === col).some(event => startPos < event.endPos || endPos < event.startPos)) {
				col++;
			}

			lastColHeight[col] = heightChange;

			events.push({
				startPos,
				endPos,
				height,
				col,
				period,
			});
		}

		containerHeight += Math.max(...Object.values(lastColHeight));
		calendarElement.style.height = `${containerHeight}px`;
	}

	if (!events) {
		return;
	}

	const indicatorDate = new Date(Math.ceil(startDate.getTime() / 1000 / 60 / 60) * 1000 * 60 * 60);
	while (indicatorDate.getTime() < endDate.getTime()) {
		const time = indicatorDate.toLocaleTimeString('en-US', {timeZone: 'America/Chicago'});
		const formatted = `${time.split(':')[0]} ${time.split(' ')[1]}`;
		const top = ((indicatorDate.getTime() / 1000) - startTime) / 60 * pixels_per_minute;
		const span = document.createElement('span');
		span.classList.add('time');
		span.textContent = formatted;
		span.style.top = `${top}px`;
		calendarElement.append(span);
		indicatorDate.setTime(indicatorDate.getTime() + (60 * 60 * 1000));
	}

	const number_cols = Math.max(...events.map(event => event.col)) + 1;
	const colwidth = calendarElement.clientWidth / number_cols;
	const percent = 1 / number_cols * 100;

	if (data_unprocessed) {
		backgroundColor = data_unprocessed.color ? bytes_to_color(data_unprocessed.color) : '#FFFFFF';
		textColor = black_or_white(backgroundColor);
	}

	for (const event of events) {
		let colspan = 1;

		while (event.col + colspan < number_cols && !events.filter(e => e.col === event.col + colspan).some(e => [event.startPos, event.endPos].some(p => p >= e.startPos && p <= e.endPos))) {
			colspan++;
		}

		let widthOffset = event.col === 0 ? 0 : padding * -2;
		if (colwidth * colspan < preferred_event_min_width && colspan < number_cols) {
			widthOffset = preferred_event_min_width - colwidth * colspan;
			if (widthOffset > colwidth) {
				widthOffset += colwidth - min_visible_width;
			}
		}

		const leftOffset = widthOffset * event.col;

		const element = document.createElement('div');
		element.classList.add('event');

		element.style.top = `${event.startPos - 3}px`;
		element.style.height = `${event.height}px`;

		element.style.left = `calc(${percent * event.col}% ${leftOffset < 0 ? '+' : '-'} ${Math.abs(leftOffset)}px)`;
		element.style.width = `calc(${percent * colspan}% ${widthOffset < 0 ? '-' : '+'} ${Math.abs(widthOffset)}px)`;

		element.style.zIndex = number_cols - event.col + 1;
		element.setAttribute('tabindex', '0');

		element.addEventListener('focus', () => {
			for (const element_ of [...document.querySelectorAll('#calendar-table .event.selected')]) {
				element_.classList.remove('selected');
			}

			element.classList.add('selected');
		});

		const childElement = document.createElement('div');
		childElement.classList.add('event-child');
		childElement.style.backgroundColor = backgroundColor;
		childElement.style.color = textColor;

		const nameElement = document.createElement('span');
		nameElement.classList.add('event-name');
		nameElement.textContent = event.period.friendly_name;
		childElement.append(nameElement);

		const timeElement = document.createElement('span');
		timeElement.classList.add('event-time');
		timeElement.textContent = `${human_time(event.period.start)} - ${human_time(event.period.end)} (${human_time_left(event.period.end, event.period.start, true)})`;
		childElement.append(timeElement);

		element.append(childElement);
		calendarElement.append(element);
	}
}

window.addEventListener('resize', () => place_boxes());

setInterval(updateNowBar, 1000);

function updateNowBar() {
	const now = current_date().getTime() / 1000;
	if (nowBarElement && showNowBar && startTime && endTime && now >= startTime && now <= endTime) {
		nowBarElement.style.top = `${((now - startTime) / 60 * pixels_per_minute) + 10}px`;
		nowBarElement.style.display = 'block';
	} else {
		nowBarElement.style.display = 'none';
	}
}

updateNowBar();

Object.assign(window, {
	place_boxes,
});


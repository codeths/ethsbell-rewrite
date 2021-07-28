let all_data;

const calendarEl = document.getElementById('event-wrapper');
const preferred_event_min_width = 300;
const min_visible_width = 50;
const min_event_height = 30;
const padding = 5;
const pixels_per_minute = 1.5;

/// Place period boxes for a list of periods.
function place_boxes(data_unprocessed, date = current_date()) {
	const data = replace_period(data_unprocessed).filter(v => v);
	const startDate = date_from_api(data[0].start, date);
	const startTime = startDate.getTime() / 1000;
	const endDate = date_from_api(data[data.length - 1].end, date)
	const endTime = endDate.getTime() / 1000;
	const sInDay = endTime - startTime;
	let containerHeight = sInDay / 60 * pixels_per_minute;
	let lastColHeight = {};

	// Resolve rows so everything is mutually non-intersecting.
	const events = [];

	data.sort((a, b) => date_from_api(a.start, date) - date_from_api(b.start, date)).forEach((period, i) => {
		// Set up variables
		const start = date_from_api(period.start, date).getTime() / 1000;
		const end = date_from_api(period.end, date).getTime() / 1000;
		const duration = end - start;
		let heightChange = 0;

		let startPos = ((start - startTime) / 60) * pixels_per_minute;
		let height = (duration / 60) * pixels_per_minute;
		let endPos = startPos + height;
		if (height < min_event_height) {
			heightChange = (min_event_height - height);
			height = min_event_height;
		}

		let col = 0;

		while (events.filter(e => e.col == col).some(e => startPos < e.endPos || endPos < e.startPos)) {
			col++;
		}

		lastColHeight[col] = heightChange;

		events.push({
			startPos,
			endPos,
			height,
			col,
			period
		});
	});

	const num_cols = Math.max(...events.map(e => e.col)) + 1;
	const colwidth = calendarEl.clientWidth / num_cols;
	const percent = 1 / num_cols * 100;

	calendarEl.innerHTML = '';
	containerHeight += Math.max(...Object.values(lastColHeight));
	calendarEl.style.height = `${containerHeight}px`;

	events.forEach((event) => {
		let colspan = 1;

		while (event.col + colspan < num_cols && !events.filter(e => e.col == event.col + colspan).some(e => [event.startPos, event.endPos].some(p => p >= e.startPos && p <= e.endPos))) {
			colspan++;
		}

		let widthOffset = event.col == 0 ? 0 : padding * -2;
		if (colwidth * colspan < preferred_event_min_width) {
			widthOffset = preferred_event_min_width - colwidth * colspan;
			if (widthOffset > colwidth) widthOffset += colwidth - min_visible_width;
		}

		let leftOffset = widthOffset * event.col;

		const el = document.createElement('div');
		el.classList.add('event');

		el.style.top = `${event.startPos}px`;
		el.style.height = `${event.height}px`;

		el.style.left = `calc(${percent * event.col}% ${leftOffset < 0 ? '+' : '-'} ${Math.abs(leftOffset)}px)`
		el.style.width = `calc(${percent * colspan}% ${widthOffset < 0 ? '-' : '+'} ${Math.abs(widthOffset)}px)`;

		el.style.zIndex = num_cols - event.col + 1;

		const childEl = document.createElement('div');
		childEl.classList.add('event-child');

		childEl.addEventListener('click', (e) => popup(e, event.period));
		childEl.addEventListener('mouseenter', (e) => popup(e, event.period));

		const nameEl = document.createElement('span');
		nameEl.classList.add('event-name');
		nameEl.innerText = event.period.friendly_name;
		childEl.appendChild(nameEl);

		const timeEl = document.createElement('span');
		timeEl.classList.add('event-time');
		timeEl.innerText = `${human_time(event.period.start)} - ${human_time(event.period.end)} (${human_time_left(event.period.end, event.period.start, true)})`
		childEl.appendChild(timeEl);

		el.appendChild(childEl);
		calendarEl.appendChild(el);
	});
}

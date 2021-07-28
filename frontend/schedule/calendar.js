let all_data;

const calendarEl = document.getElementById('event-wrapper');
const nowBarEl = document.querySelector('#calendar #now');
const preferred_event_min_width = 300;
const min_visible_width = 50;
const min_event_height = 30;
const padding = 5;
const pixels_per_minute = 1.5;
let startDate, startTime, endDate, endTime, events;

/// Place period boxes for a list of periods.
function place_boxes(data_unprocessed, date = current_date(), force = false) {
	calendarEl.innerHTML = '';
	if (!events || force) {
		calendarEl.style.height = `auto`;
		if (data_unprocessed.length == 0) return;
		const data = replace_period(data_unprocessed).filter(v => v);
		startDate = date_from_api(data[0].start, date);
		startTime = startDate.getTime() / 1000;
		endDate = date_from_api(data[data.length - 1].end, date)
		endTime = endDate.getTime() / 1000;
		updateNowBar();
		const sInDay = endTime - startTime;
		let containerHeight = sInDay / 60 * pixels_per_minute;
		let lastColHeight = {};

		let indicatorDate = new Date(Math.ceil(startDate.getTime() / 1000 / 60 / 60) * 1000 * 60 * 60);
		while (indicatorDate.getTime() < endDate.getTime()) {
			const time = indicatorDate.toLocaleTimeString('en-US', { timeZone: 'America/Chicago' })
			const formatted = `${time.split(':')[0]} ${time.split(' ')[1]}`;
			const top = (indicatorDate.getTime() / 1000 - startTime) / 60 * pixels_per_minute;
			const span = document.createElement('span');
			span.classList.add('time');
			span.innerText = formatted;
			span.style.top = `${top}px`;
			calendarEl.appendChild(span);
			indicatorDate.setTime(indicatorDate.getTime() + 60 * 60 * 1000);
		}

		// Resolve rows so everything is mutually non-intersecting.
		events = [];

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

		containerHeight += Math.max(...Object.values(lastColHeight));
		calendarEl.style.height = `${containerHeight}px`;
	}

	const num_cols = Math.max(...events.map(e => e.col)) + 1;
	const colwidth = calendarEl.clientWidth / num_cols;
	const percent = 1 / num_cols * 100;

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
		el.setAttribute('tabindex', "0");

		el.addEventListener('focus', () => {
			[...document.querySelectorAll('#calendar .event.selected')].forEach(el => el.classList.remove('selected'));
			el.classList.add('selected');
		});

		const childEl = document.createElement('div');
		childEl.classList.add('event-child');

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

window.addEventListener('resize', place_boxes);

setInterval(updateNowBar, 1000);

function updateNowBar() {
	const now = new Date().getTime() / 1000;
	if (nowBarEl && startTime && endTime && now >= startTime && now <= endTime) {
		nowBarEl.style.top = `${(now - startTime) / 60 * pixels_per_minute}px`;
		nowBarEl.style.display = 'block';
	} else {
		nowBarEl.style.display = 'none';
	}
}

updateNowBar();
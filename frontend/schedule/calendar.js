let all_data;

const calendarEl = document.getElementById('calendar');
const container_height = calendarEl.clientHeight;
const preferred_event_min_width = 300;
const min_visible_width = 50;
const min_event_height = 50;
const padding = 5;
let pixels_per_minute;

let viewport_seconds = 3600 * 4; // The number of seconds the viewport should show
let viewport_offset = 0;
const row_height = 50; // The height of a row
const box_height = 40; // The height of a box
const row_start = 10; // The height rows start at
const text_height = 30; // The size of the font
let has_resize_listener = false;
let scroll_timeout;
let last_scroll_touch_x;
let scrolled_distance = 0;

/// Place period boxes for a list of periods.
function place_boxes(data_unprocessed) {
	const data = replace_period(data_unprocessed).filter(v => v);
	const startTime = data[0].start_timestamp;
	const endTime = data[data.length - 1].end_timestamp;
	const msInDay = endTime - startTime;
	pixels_per_minute = container_height / (msInDay / 60);

	// Resolve rows so everything is mutually non-intersecting.
	const events = [];
	let offsets = {
		"0": 0
	};
	let fullOffset = 0;

	let num_cols = 1;

	data.sort((a, b) => a.start_timestamp - b.start_timestamp);
	data.forEach((period) => {
		let offset = 0;

		// Set up variables
		const start = period.start_timestamp;
		const end = period.end_timestamp;
		const duration = end - start;

		let startPos = ((start - startTime) / 60) * pixels_per_minute;

		let nextFullOffset = fullOffset;
		events.filter(e => e.endPos - e.offset < startPos && !e.colspan).forEach(event => {
			console.log(event.period.friendly_name);
			console.log(events);
			let colspan = 1;

			while (events.filter(e => e.col == event.col && e.period.friendly_name !== event.period.friendly_name).some(e => [event.offsetPos, event.endPos].some(p => p >= e.startPos && p <= e.endPos))) {
				event.col++;
			}
			num_cols = Math.max(num_cols, event.col + 1);

			while (event.col + colspan < num_cols && !events.filter(e => e.col == event.col + colspan).some(e => [event.startPos, event.endPos].some(p => p >= e.startPos && p <= e.endPos))) {
				colspan++;
			}

			event.colspan = colspan;
			for (let i = event.col; i < event.col + colspan; i++) {
				if (!offsets[i]) offsets[i] = fullOffset;
				offsets[i] += event.offset;
			}

			if (event.offset > nextFullOffset) nextFullOffset = event.offset;
		});

		fullOffset = nextFullOffset;

		let offsetPos = startPos + fullOffset;

		let height = (duration / 60) * pixels_per_minute;
		if (height < min_event_height) {
			offset += min_event_height - height;
			height = min_event_height;
		}

		const endPos = startPos + height;
		let col = 0;

		while (events.filter(e => e.col == col).some(e => [offsetPos, endPos].some(p => p >= e.startPos && p <= e.endPos))) {
			col++;
		}

		// console.log(period.friendly_name);
		// console.log(offsets);
		// console.log(fullOffset);
		// console.log(startPos);

		startPos += (offsets[col] ?? fullOffset);
		// console.log(startPos);


		num_cols = Math.max(num_cols, col + 1);

		events.push({
			startPos,
			endPos,
			height,
			col,
			period,
			offset
		});
	});

	console.log(events);

	const colwidth = calendarEl.clientWidth / num_cols;
	const percent = 1 / num_cols * 100;

	calendarEl.innerHTML = '';

	events.forEach((event) => {
		let colspan = 1;

		while (event.col + colspan < num_cols && !events.filter(e => e.col == event.col + colspan).some(e => [event.startPos, event.endPos].some(p => p >= e.startPos && p <= e.endPos))) {
			colspan++;
		}

		let widthOffset = padding * -2;
		if (colwidth * colspan < preferred_event_min_width) {
			widthOffset = preferred_event_min_width - colwidth * colspan;
			if (widthOffset > colwidth) widthOffset += colwidth - min_visible_width;
		}

		const el = document.createElement('div');
		el.classList.add('event');

		el.style.top = `${event.startPos}px`;
		el.style.height = `${event.height - padding * 2}px`;

		el.style.left = `calc(${percent * event.col}% ${widthOffset < 0 ? '+' : '-'} ${Math.abs(widthOffset)}px)`
		el.style.width = `calc(${percent * colspan}% ${widthOffset < 0 ? '-' : '+'} ${Math.abs(widthOffset)}px)`;

		el.style.zIndex = num_cols - event.col + 1;

		el.addEventListener('click', (e) => popup(e, event.period));
		el.addEventListener('mouseenter', (e) => popup(e, event.period));

		const nameEl = document.createElement('span');
		nameEl.classList.add('event-name');
		nameEl.innerText = event.period.friendly_name;
		el.appendChild(nameEl);

		const timeEl = document.createElement('span');
		timeEl.classList.add('event-time');
		timeEl.innerText = `${human_time(event.period.start)} - ${human_time(event.period.end)}`
		el.appendChild(timeEl);

		calendarEl.appendChild(el);
	});
}

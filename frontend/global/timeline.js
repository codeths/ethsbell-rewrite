let all_data;

let viewport_seconds = 3600 * 6; // The number of seconds the viewport should show
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
	let data = replace_period(data_unprocessed);
	if (!has_resize_listener) {
		window.addEventListener('resize', () => {
			place_boxes(all_data);
		});
		has_resize_listener = true;
		const set_length = event_ => {
			viewport_seconds = event_.target.value * 3600;
			place_boxes(all_data);
		};

		const set_offset = event_ => {
			viewport_offset = event_.target.value * 3600;
			place_boxes(all_data);
		};

		// Update fields
		const events = ['input'];
		getel('timeline_length').value = viewport_seconds / 3600;
		getel('timeline_offset').value = 0;
		for (const event_name of events) {
			getel('timeline_length')?.addEventListener(event_name, set_length);
			getel('timeline_offset')?.addEventListener(event_name, set_offset);
		}

		// Scroll behaviour
		const scroll = () => {
			scroll_timeout = false;
			viewport_offset += scrolled_distance * 2;
			scrolled_distance = 0;
			place_boxes(all_data);
			getel('timeline_offset').value = Math.round(viewport_offset / 360) / 10;
		};

		getel('timeline')?.addEventListener('wheel', event => {
			scrolled_distance += event.deltaX * 3;
			if (!scroll_timeout) {
				setTimeout(scroll, 10);
				scroll_timeout = true;
			}
		});
		getel('timeline')?.addEventListener('touchmove', event => {
			const new_touch_x = event.touches[0].clientX;
			if (last_scroll_touch_x) {
				scrolled_distance += (last_scroll_touch_x - new_touch_x) * 10;
				if (!scroll_timeout) {
					setTimeout(scroll, 10);
					scroll_timeout = true;
				}
			}

			last_scroll_touch_x = new_touch_x;
		});
		getel('timeline')?.addEventListener('touchend', () => {
			last_scroll_touch_x = undefined;
		});
	}

	// Resolve rows so everything is mutually non-intersecting.
	const boxes = [];
	data = data.flat().filter(v => v);
	data.sort((a, b) => a.start - b.start);
	for (const period of data) {
		// Set up variables
		const start = period.start_timestamp;
		let row = 0;
		let intersecting = 0;
		// Increment the row until nothing intersects.
		do {
			intersecting = 0;
			for (const box of boxes) {
				if (box.row === row && box.end > start) {
					intersecting += 1;
				}
			}

			if (intersecting > 0) {
				row += 1;
			}
		} while (intersecting > 0);

		boxes.push({
			row,
			start: period.start_timestamp,
			end: period.end_timestamp,
			kind: period.kind,
			name: period.friendly_name,
			period,
		});
	}

	// Determine where the boxes should be placed on-screen
	const now = (Date.now() / 1000) + viewport_offset;
	const viewport_width = getel('timeline>svg').clientWidth;
	for (const box of boxes) {
		const length = (box.end - box.start);
		const relative_time = box.start - now;
		const fraction_time = relative_time / viewport_seconds;
		let fraction_position = 0.5 + fraction_time;
		let fraction_outside_length = 0;
		if (fraction_position < 0) {
			fraction_outside_length = -fraction_position;
			fraction_position = 0;
		}

		const absolute_x = viewport_width * fraction_position;
		fraction_position *= 100;
		let fraction_length = length / viewport_seconds;
		fraction_length -= fraction_outside_length;
		if (fraction_length <= 0 || fraction_position > 100) {
			box.hidden = true;
		} else {
			fraction_length *= 100;
			if (fraction_length + fraction_position > 100) {
				fraction_length = 100 - fraction_position;
			}

			box.x = fraction_position;
			box.y = row_start + (row_height * box.row);
			box.w = fraction_length;
			box.h = box_height;
			const text_margin = (box_height - text_height) / 2;
			box.tx = absolute_x + text_margin;
			box.ty = box.y + (box.h / 2) + (text_margin * 2);
			box.th = text_height;
		}
	}

	// Set the box's emoji and TODO color
	for (const box of boxes) {
		if (box.hidden) {
			continue;
		}

		let emoji;
		if (box.kind.Class) {
			emoji = '🏫';
		} else if (box.kind.ClassOrLunch) {
			emoji = '🏫/🥪';
		} else {
			switch (box.kind) {
				case 'Lunch':
					emoji = '🥪';
					break;
				case 'Break':
					emoji = '🛌';
					break;
				case 'AMSupport':
					emoji = '🐔';
					break;
				case 'Passing':
					emoji = '🏃';
					break;
				case 'BeforeSchool':
					emoji = '🌄';
					break;
				case 'AfterSchool':
					emoji = '🌇';
					break;
				case 'Announcements':
					emoji = '📣';
					break;
				default:
					emoji = emoji || '😕';
			}
		}

		box.emoji = emoji;
		box.color = 'white';
	}

	// Write the boxes to the DOM
	let output = '';
	for (const box of boxes) {
		if (box.hidden) {
			continue;
		}

		output += getel('period_box').innerHTML
			.replace('X', box.x)
			.replace('Y', box.y)
			.replace('W', box.w)
			.replace('H', box.h)
			.replace('TX', box.tx)
			.replace('TY', box.ty)
			.replace('TH', box.th)
			.replace('COLOR', box.color)
			.replace('CONTENT', ((box.w > 10) && (box.w * viewport_width / 100 > text_height * box.name.length / 1.35)) ? `${box.emoji} ${period_html(box.period)}` : box.emoji);
	}

	getel('boxes').innerHTML = output;
}

let periodText;
let endTimeText;
let nextText;
let schedulenameElement;

let lastData;
// Gets data from /today/now/near

let progressIntervals = [];

function display(data) {
	if (!data) {
		return;
	}

	for (const interval of progressIntervals) {
		clearInterval(interval);
	}

	progressIntervals = [];

	if (data[2] && (!data[1] || !data[1][0] || data[1][0].kind !== 'BeforeSchool')) {
		getel('next_period').innerHTML = getel('next_period_template').innerHTML;
		put_period_to_element(getel('next_period'), data[2]);
		getel('next_parent').style.display = 'block';
	} else {
		getel('next_parent').style.display = 'none';
	}

	const template = getel('current_period_time_template');
	const parent = getel('current_parent');
	parent.innerHTML = '';
	if (data[1][0]) {
		if (lastData) {
			const oldPeriods = lastData
				.flat()
				.filter(x => x)
				.map(x => x.friendly_name);
			const nowPeriods = new Set(data[1].filter(x => x).map(x => x.friendly_name));

			if (oldPeriods.every(x => !nowPeriods.has(x))) {
				schedule();
			}
		}

		for (const i of data[1]) {
			const new_element = document.createElement('div');
			new_element.innerHTML = template.innerHTML;
			const didPutProgress = put_period_to_element(new_element, i);
			parent.append(new_element);
			if (didPutProgress) {
				const size = Number.parseFloat(document.defaultView.getComputedStyle(new_element, null).fontSize.slice(0, -2));
				const svg = new_element.querySelector('.progress-ring');
				svg.setAttribute('width', size);
				svg.setAttribute('height', size);
				const circle = new_element.querySelector('.progress-ring__circle');
				const border = new_element.querySelector('.progress-ring__border');
				circle.setAttribute('cx', size / 2);
				circle.setAttribute('cy', size / 2);
				circle.setAttribute('r', size / 4 - 1);
				circle.setAttribute('stroke-width', size / 2 - 2);
				border.setAttribute('cx', size / 2);
				border.setAttribute('cy', size / 2);
				border.setAttribute('r', size / 2 - 2);

				progressIntervals.push(
					setInterval(() => {
						update_progress_circular(i, new_element);
					}, 1000),
				);
				update_progress_circular(i, new_element);
			}
		}
	} else {
		const new_element = document.createElement('div');
		new_element.innerHTML = template.innerHTML;
		put_period_to_element(new_element, null);
		parent.append(new_element);
	}

	lastData = data;
}

window.addEventListener('resize', () => display(lastData));

async function schedule() {
	const day = await get('/api/v1/today');
	if (!day) {
		return;
	}

	setColors(bytes_to_color(day.color));

	if (day.periods.length > 0) {
		schedulenameElement.innerHTML = `${day.friendly_name}`;
		schedulenameElement.style.display = 'inline-block';
	} else {
		schedulenameElement.innerHTML = '';
		schedulenameElement.style.display = 'none';
	}
}

function setColors(color) {
	if (!config.use_schedule_color) {
		return;
	}

	color ??= getCookie('schedule_color');
	if (!color) {
		return;
	}

	setCookie('schedule_color', color, new Date().setHours(24, 0, 0, 0));
	document.body.style.setProperty('--background_color', color);
	// Document.body.style.setProperty('--foreground_color', 'unset');
	document.body.style.setProperty('--background_text_color', black_or_white(color));
	// Document.body.style.setProperty('--foreground_text_color', 'unset');
}

setColors();

window.addEventListener('load', () => {
	periodText = document.querySelector('#period');
	endTimeText = document.querySelector('#end_time');
	nextText = document.querySelector('#next');
	schedulenameElement = document.querySelector('#schedulename');

	go(display);
	schedule();
});

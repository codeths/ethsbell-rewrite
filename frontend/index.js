const periodText = document.querySelector('#period');
const endTimeText = document.querySelector('#end_time');
const nextText = document.querySelector('#next');

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

	lastData = data;

	if (data[2] && (!data[1] || !data[1][0] || data[1][0].kind !== 'BeforeSchool')) {
		put_period_to_element(getel('next_period'), data[2]);
		getel('next_parent').style.display = 'block';
	} else {
		getel('next_parent').style.display = 'none';
	}

	const template = getel('current_period_time_template');
	const parent = getel('current_parent');
	parent.innerHTML = '';
	if (data[1][0]) {
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

				progressIntervals.push(setInterval(() => {
					update_progress_circular(i, new_element);
				}, 1000));
				update_progress_circular(i, new_element);

				const timeLeft = date_from_api(i.end).getTime() - Date.now();
				if (timeLeft <= 0) {
					setTimeout(() => go(display, false), 2000);
				}
			}
		}
	} else {
		const new_element = document.createElement('div');
		new_element.innerHTML = template.innerHTML;
		put_period_to_element(new_element, null);
		parent.append(new_element);
	}
}

window.addEventListener('resize', () => display(lastData));

go(display);

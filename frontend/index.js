const periodText = document.querySelector('#period');
const endTimeText = document.querySelector('#end_time');
const nextText = document.querySelector('#next');

let lastData;
// Gets data from /today/now/near
function display(data) {
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
			put_period_to_element(new_element, i);
			parent.append(new_element);
		}
	} else {
		const new_element = document.createElement('div');
		new_element.innerHTML = template.innerHTML;
		put_period_to_element(new_element, null);
		parent.append(new_element);
	}

	update_progress(data);
}

go(display);

setInterval(() => {
	if (lastData) {
		update_progress(lastData);
	}
}, 1000);

// Full screen button
const fullScreenButton = document.querySelector('#fullscreen');
const wrapper = document.querySelector('#wrapper');
const enterFull = document.querySelector('#enter_full');
const exitFull = document.querySelector('#exit_full');

// Show button if browser supports full screen
if (canFullScreen()) {
	fullScreenButton.style.display = 'inline-block';
	fullScreenButton.addEventListener('click', async () => {
		toggleFullScreen(wrapper);
	});
}

// Show appropriate icon when entering/exiting full screen
document.addEventListener('fullscreenchange', () => {
	if (isFullScreen()) {
		enterFull.classList.add('none');
		exitFull.classList.remove('none');
	} else {
		enterFull.classList.remove('none');
		exitFull.classList.add('none');
	}

	// Show full screen button
	fadeIn();
	idleTimer();
});

// Hide full screen button and cursor if full screen when idle
let idleCursorTimeout = setTimeout(fadeOut, 5000);
let doNotHideButton = false;

// Run fade in animation
function fadeIn() {
	if (fullScreenButton.classList.contains('fadeout')) {
		fullScreenButton.classList.add('fadein');
		fullScreenButton.classList.remove('fadeout');
	}

	document.body.classList.remove('hidecursor');
}

// Run fade out animation
function fadeOut() {
	if (doNotHideButton) {
		return;
	}

	if (!isFullScreen()) {
		return;
	}

	fullScreenButton.classList.remove('fadein');
	fullScreenButton.classList.add('fadeout');
	setTimeout(() => document.body.classList.add('hidecursor'), 500);
}

// Reset idle timer
function idleTimer() {
	clearTimeout(idleCursorTimeout);
	idleCursorTimeout = setTimeout(fadeOut, 5000);
}

// Reset idle timer when mouse moved
document.addEventListener('mousemove', () => {
	fadeIn();
	idleTimer();
});

// Block fade out when hovering full screen button
fullScreenButton.addEventListener('mouseenter', () => {
	doNotHideButton = true;
});

// Unblock fade out when stop hovering full screen button
fullScreenButton.addEventListener('mouseleave', () => {
	doNotHideButton = false;
});

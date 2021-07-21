const periodText = document.querySelector('#period');
const endTimeText = document.querySelector('#end_time');
const nextText = document.querySelector('#next');

// Gets data from /today/now/near
function display(data) {
	if (data[1]) {
		const names = data[1].map(period => period.friendly_name);
		const ends = data[1].map(period => `${human_time(period.end)} (in ${human_time_left(period.end)})`);
		periodText.textContent = `${human_list(names)} ${data[1].length > 1 ? 'end' : 'ends'} at`;

		endTimeText.textContent = ends.every(value => value === ends[0]) ? `${ends[0]}` : `${human_list(ends)}${data[1].length > 1 ? ', respectively.' : '.'}`;

		nextText.textContent = data[2] ? `The next period is ${data[2].friendly_name}, which ends at ${human_time(data[2].end)}` : 'This is the last period.';
	} else {
		periodText.textContent = 'There is no current period.';
		endTimeText.textContent = '';
		nextText.textContent = '';
	}
}

go();

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
	if (fullScreenButton.classList.contains('fade_out')) {
		fullScreenButton.classList.add('fade_in');
		fullScreenButton.classList.remove('fade_out');
	}

	document.body.classList.remove('hide_cursor');
}

// Run fade out animation
function fadeOut() {
	if (doNotHideButton) {
		return;
	}

	if (!isFullScreen()) {
		return;
	}

	fullScreenButton.classList.remove('fade_in');
	fullScreenButton.classList.add('fade_out');
	setTimeout(() => document.body.classList.add('hide_cursor'), 500);
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
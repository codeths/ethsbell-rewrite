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
let doNotIdleTimer = false;

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

	doNotIdleTimer = true;

	document.body.classList.add('hidecursor');
	fullScreenButton.classList.remove('fadein');
	fullScreenButton.classList.add('fadeout');

	setTimeout(() => {
		doNotIdleTimer = false;
	}, 50);
}

// Reset idle timer
function idleTimer() {
	clearTimeout(idleCursorTimeout);
	idleCursorTimeout = setTimeout(fadeOut, 5000);
}

// Reset idle timer when mouse moved
document.addEventListener('mousemove', () => {
	if (!doNotIdleTimer) {
		fadeIn();
		idleTimer();
	}
});

// Block fade out when hovering full screen button
fullScreenButton.addEventListener('mouseenter', () => {
	doNotHideButton = true;
});

// Unblock fade out when stop hovering full screen button
fullScreenButton.addEventListener('mouseleave', () => {
	doNotHideButton = false;
});

if (new URLSearchParams(
	window.location.search,
).get('fullscreen') !== null) {
	document.querySelector('nav').style.display = 'none';
}

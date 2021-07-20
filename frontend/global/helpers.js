let lastFetchedData = null;

async function get(endpoint = '/api/v1/today/now/near') {
	return fetch(`${endpoint}${window.location.search}`)
		.then(x => x.json()
			.catch(() => null))
		.catch(() => null);
}

function process(data) {
	// TODO: This will perform class name replacements
	return data;
}

function getel(id) {
	return document.querySelector(`#${id}`);
}

async function go() {
	if (lastFetchedData) {
		display(lastFetchedData);
	}

	const now = Date.now();
	const endOfMinute = Math.ceil(now / 60_000) * 60_000;
	setTimeout(go, endOfMinute - now);
	let data = await get();
	if (!data) {
		return;
	}

	data = process(data);
	lastFetchedData = data;
	display(data);
}

function human_list(items) {
	let output = '';
	if (items.length === 1) {
		return items[0].toString();
	}

	for (let i = 0; i < items.length; i++) {
		if (i === items.length - 1) {
			output += `${items.length > 2 ? ', ' : ' '}and ${items[i].toString()}`;
		} else if (i === 0) {
			output += items[i].toString();
		} else {
			output += `, ${items[i].toString()}`;
		}
	}

	return output;
}

// Add plural suffix to a unit
function plural_suffix(number, string) {
	return `${string}${number === 1 ? '' : 's'}`;
}

// Gets current epoch in seconds
// If timestamp query string is provided, that is used instead.
function current_epoch() {
	const timestampQueryString = new URLSearchParams(window.location.search).get('timestamp');
	if (timestampQueryString) {
		return Number.parseInt(timestampQueryString, 10) * 1000;
	}

	return Date.now();
}

function date_from_api(time) {
	const [h, m, s] = time.split(':');
	const now = new Date(Date.now());
	const date = new Date(now.getFullYear(), now.getMonth(), now.getDate(), h, m, s);
	return date;
}

function human_time(time) {
	const date = date_from_api(time);
	return date.toLocaleTimeString([], { hour: 'numeric', minute: '2-digit' });
}

// Gets a human readable duration from an epoch timestamp
function human_time_left(endTime, startTime = null, short = false) {
	const endDate = date_from_api(endTime).getTime();
	const startDate = startTime ? date_from_api(startTime).getTime() : current_epoch();
	const timeLeft = Math.floor((endDate - startDate) / 1000);
	const h = Math.floor(timeLeft / 60 / 60);
	const m = Math.ceil(timeLeft / 60 % 60);
	if (short) {
		if (h > 0) {
			return `${h}h ${m}m`;
		}

		return `${m}m`;
	}

	if (h > 0) {
		return `${h} ${plural_suffix(h, 'hour')} and ${m} ${plural_suffix(m, 'minute')}`;
	}

	return `${m} ${plural_suffix(m, 'minute')}`;
}

// Helper functions for full screen
function enterFullScreen(element = document.documentElement) {
	if (element.requestFullscreen) {
		element.requestFullscreen();
	} else if (element.webkitRequestFullscreen) {
		element.webkitRequestFullscreen();
	} else if (element.mozRequestFullScreen) {
		element.mozRequestFullScreen();
	} else if (element.msRequestFullscreen) {
		element.msRequestFullscreen();
	}
}

function exitFullscreen() {
	if (document.exitFullscreen) {
		document.exitFullscreen();
	} else if (document.webkitExitFullscreen) {
		document.webkitExitFullscreen();
	} else if (document.mozExitFullScreen) {
		document.mozExitFullScreen();
	} else if (document.msExitFullscreen) {
		document.msExitFullscreen();
	}
}

function canFullScreen() {
	return document.fullscreenEnabled || document.mozFullScreenEnabled || document.webkitFullscreenEnabled || document.msFullscreenEnabled;
}

function isFullScreen() {
	return (document.fullscreenElement || document.mozFullScreenElement || document.webkitIsFullScreen || document.msFullscreenElement || null) !== null;
}

function toggleFullScreen(element) {
	if (canFullScreen()) {
		if (isFullScreen()) {
			exitFullscreen();
		} else {
			enterFullScreen(element);
		}
	}
}

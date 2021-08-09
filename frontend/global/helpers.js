let lastFetchedData = null;

async function get(endpoint = '/api/v1/today/now/near') {
	return fetch(`${endpoint}${window.location.search}`)
		.then(x => x.json()
			.catch(() => null))
		.catch(() => null);
}

const config = JSON.parse(localStorage.getItem('schedule')) || {
	schedule: {},
	foreground_color: '#1a2741',
	background_color: '#c34614',
	foreground_text_color: '#ffffff',
	background_text_color: '#ffffff',
};

function replace_period(period) {
	if (!period) {
		return period;
	}

	if (Array.isArray(period)) {
		return period.map(v => replace_period(v));
	}

	if (period.kind?.Class || period.kind?.ClassOrLunch) {
		const class_id = period.kind.Class || period.kind.ClassOrLunch;
		const class_cfg = config.schedule[class_id];
		if (class_cfg) {
			period.friendly_name = class_cfg.name;
			period.url = class_cfg.url;
		}

		return period;
	}

	return period;
}

function process(data) {
	if (config) {
		return data.map(v => replace_period(v));
	}

	return data;
}

function period_html(period) {
	return period ? (period.url ? `<a href=${period.url}>${period.friendly_name}</a>` : period.friendly_name) : 'None';
}

function getel(id) {
	return document.querySelector(`#${id}`);
}

async function go(display) {
	if (lastFetchedData) {
		display(lastFetchedData);
	}

	const now = Date.now();
	const endOfMinute = Math.ceil(now / 60000) * 60000;
	setTimeout(() => go(display), endOfMinute - now);
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

// Gets current date
// If timestamp query string is provided, that is used instead of current.
function current_date() {
	const timestampQueryString = new URLSearchParams(window.location.search).get('timestamp');
	if (timestampQueryString) {
		return new Date(Number.parseInt(timestampQueryString, 10) * 1000);
	}

	return new Date();
}

function date_from_api(time, now = current_date()) {
	const [h, m, s] = time.split(':');
	const date = new Date(now.getUTCFullYear(), now.getUTCMonth(), now.getUTCDate(), h, m, s);
	return date;
}

function human_time(time) {
	const date = date_from_api(time);
	return date.toLocaleTimeString('en-US', {hour: 'numeric', minute: '2-digit', timeZone: 'America/Chicago'});
}

// Gets a human readable duration from an epoch timestamp
function human_time_left(endTime, startTime = null, short = false) {
	const startDate = startTime ? date_from_api(startTime).getTime() : current_date().getTime();
	const endDate = date_from_api(endTime).getTime();
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

// Convert date object to YYYY-MM-DD
function date_to_string(date = current_date(), utc = true) {
	if (utc) {
		return `${date.getUTCFullYear()}-${('0' + (date.getUTCMonth() + 1)).slice(-2)}-${('0' + date.getUTCDate()).slice(-2)}`;
	}

	return `${date.getFullYear()}-${('0' + (date.getMonth() + 1)).slice(-2)}-${('0' + date.getDate()).slice(-2)}`;
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

window.addEventListener('load', () => {
	const nav_links = document.querySelector('#nav-links');
	document.querySelector('#nav-toggle-button').addEventListener('click', () => {
		nav_links.classList.toggle('show');
		nav_links.style.maxHeight = nav_links.classList.contains('show') ? nav_links.querySelector('ul').clientHeight + 'px' : '';
	});
});

window.addEventListener('resize', () => {
	const nav_links = document.querySelector('#nav-links');
	if (nav_links.classList.contains('show')) {
		nav_links.style.maxHeight = nav_links.querySelector('ul').clientHeight + 'px';
	}
});

// Convert array of RGB to hex
function bytes_to_color(bytes) {
	return '#' + bytes.map(b => ('0' + b.toString(16)).slice(-2)).join('');
}

// Detect whether text should be black or white based on the background color
function black_or_white(color, opacity = 1) {
	if (!color.startsWith('#')) {
		color = `#${color}`;
	}

	const r = Number.parseInt(color.slice(1, 3), 16);
	const g = Number.parseInt(color.slice(3, 5), 16);
	const b = Number.parseInt(color.slice(5, 7), 16);
	const luma = (0.2126 * r) + (0.7152 * g) + (0.0722 * b);
	return luma > 128 ? `rgba(0, 0, 0, ${opacity})` : `rgba(255, 255, 255, ${opacity})`;
}

function getUTCOffset() {
	return Number.parseInt(new Date(new Date().setUTCHours(0, 0, 0, 0)).toLocaleTimeString('en-US', {timeZone: 'America/Chicago', hour12: false}).split(':')[0], 10) - 24;
}

function dateStringToDate(dateString) {
	const offset = getUTCOffset();
	const h = Math.trunc(offset);
	const m = Math.trunc((offset - h) * 60);
	return new Date(`${dateString}Z${h}:${m}`);
}

// Apply user colors
window.addEventListener('load', () => {
	const cfg = JSON.parse(localStorage.getItem('schedule'));

	document.querySelector('meta[name=theme-color]').setAttribute('content', (cfg || {}).foreground_color || '#1a2741');

	if (!cfg) {
		return;
	}

	if (cfg.background_color) {
		document.querySelector('body').style.setProperty('--background_color', cfg.background_color);
	}

	if (cfg.foreground_color) {
		document.querySelector('body').style.setProperty('--foreground_color', cfg.foreground_color);
	}

	if (cfg.background_text_color) {
		document.querySelector('body').style.setProperty('--background_text_color', cfg.background_text_color);
	}

	if (cfg.foreground_text_color) {
		document.querySelector('body').style.setProperty('--foreground_text_color', cfg.foreground_text_color);
	}
});

Object.assign(window, {
	black_or_white,
	bytes_to_color,
	canFullScreen,
	current_date,
	date_from_api,
	date_to_string,
	enterFullScreen,
	exitFullscreen,
	get,
	getel,
	go,
	human_list,
	human_time,
	human_time_left,
	isFullScreen,
	period_html,
	plural_suffix,
	process,
	replace_period,
	toggleFullScreen,
	put_period_to_element,
});

// Writes a period to an element and its children
function put_period_to_element(element, period) {
	if (period) {
		if (period.kind === 'BeforeSchool') {
			element.innerHTML = 'School hasn\'t started yet!';
			return false;
		}

		if (period.kind === 'AfterSchool') {
			element.innerHTML = 'School\'s out!';
			return false;
		}

		const start = element.querySelector('.start');
		const start_in = element.querySelector('.start_in');
		const end = element.querySelector('.end');
		const end_in = element.querySelector('.end_in');
		const name = element.querySelector('.name');

		if (start) {
			start.innerHTML = human_time(period.start);
		}

		if (start_in) {
			start_in.innerHTML = human_time_left(period.start, undefined, true);
		}

		if (end) {
			end.innerHTML = human_time(period.end);
		}

		if (end_in) {
			end_in.innerHTML = human_time_left(period.end, undefined, true);
		}

		if (name) {
			name.innerHTML = period.url ? `<a href="${period.url}">${period.friendly_name}</a>` : period.friendly_name;
		}

		return true;
	}

	element.innerHTML = 'No School';
	return false;
}

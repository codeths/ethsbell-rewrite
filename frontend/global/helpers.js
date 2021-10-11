// Polyfills

// String.prototype.replaceAll
if (!String.prototype.replaceAll) {
	String.prototype.replaceAll = function (search, replacement) {
		return this.replace(
			typeof search === 'string' ? new RegExp(search, 'g') : search,
			replacement,
		);
	};
}

// Start helpers
let lastFetchedData = null;
let serverOffset = null;

async function get(endpoint = '/api/v1/today/now/near') {
	if (serverOffset === null) {
		serverOffset = await getServerOffset();
	}

	return fetch(
		`${endpoint}?timestamp=${Math.floor(current_date().getTime() / 1000)}`,
	)
		.then(x => x.json().catch(() => null))
		.catch(() => null);
}

let config;
function updateConfig() {
	config = Object.assign(
		{
			schedule: {},
			foreground_color: '#1a2741',
			background_color: '#c34614',
			foreground_text_color: '#ffffff',
			background_text_color: '#ffffff',
			include_period_name: true,
		},
		JSON.parse(localStorage.getItem('schedule') || '{}'),
	);
	return config;
}

updateConfig();

function replace_period(period) {
	if (!period) {
		return period;
	}

	if (Array.isArray(period)) {
		return period.map(v => replace_period(v));
	}

	const period_tmp = Object.assign({}, period);

	const class_id = period_tmp.kind.Class || period_tmp.kind.ClassOrLunch;
	const class_cfg
		= config.schedule[class_id] || config.schedule[period_tmp.friendly_name];
	if (class_cfg) {
		if (class_cfg.name) {
			period_tmp.friendly_name
				= config.include_period_name
				|| config.include_period_name === undefined
					? `${period_tmp.friendly_name} - ${class_cfg.name}`
					: class_cfg.name;
		}

		if (class_cfg.url) {
			period_tmp.url = class_cfg.url;
		}
	}

	return period_tmp;
}

function process(data) {
	if (config) {
		return data.map(v => replace_period(v));
	}

	return data;
}

function period_html(period) {
	return period
		? (period.url
			? `<a href=${period.url}>${period.friendly_name
				.replaceAll('<', '&lt;')
				.replaceAll('>', '&gt;')}</a>`
			: period.friendly_name
				.replaceAll('<', '&lt;')
				.replaceAll('>', '&gt;'))
		: 'None';
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
			output += `${items.length > 2 ? ', ' : ' '}and ${items[
				i
			].toString()}`;
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
	const timestampQueryString = new URLSearchParams(
		window.location.search,
	).get('timestamp');
	if (timestampQueryString) {
		return new Date(Number.parseInt(timestampQueryString, 10) * 1000);
	}

	return new Date();
}

function date_from_api(time, now = current_date()) {
	const [h, m, s] = time.split(':');
	const date = new Date(
		now.getUTCFullYear(),
		now.getUTCMonth(),
		now.getUTCDate(),
		h,
		m,
		s,
	);
	date.setTime(date.getTime() + serverOffset);
	return date;
}

// YYYY-MM-DD to Date object
function date_string_to_date(dateString) {
	const [y, m, d] = dateString.split('-');
	return new Date(
		Number.parseInt(y, 10),
		Number.parseInt(m, 10) - 1,
		Number.parseInt(d, 10),
	);
}

function human_time(time) {
	const date = date_from_api(time);
	return date.toLocaleTimeString('en-US', {
		hour: 'numeric',
		minute: '2-digit',
	});
}

// Gets a human readable duration from an epoch timestamp
function human_time_left(endTime, startTime = null, short = false) {
	const startDate = startTime
		? date_from_api(startTime).getTime()
		: current_date().getTime();
	const endDate = date_from_api(endTime).getTime();
	const timeLeft = Math.floor((endDate - startDate) / 1000);
	const h = Math.floor(timeLeft / 60 / 60);
	const m = Math.ceil((timeLeft / 60) % 60);
	if (short) {
		if (h > 0) {
			return `${h}h ${m}m`;
		}

		return `${m}m`;
	}

	if (h > 0) {
		return `${h} ${plural_suffix(h, 'hour')} and ${m} ${plural_suffix(
			m,
			'minute',
		)}`;
	}

	return `${m} ${plural_suffix(m, 'minute')}`;
}

// Convert date object to YYYY-MM-DD
function date_to_string(date = current_date(), utc = true) {
	if (utc) {
		return `${date.getUTCFullYear()}-${(
			'0'
			+ (date.getUTCMonth() + 1)
		).slice(-2)}-${('0' + date.getUTCDate()).slice(-2)}`;
	}

	return `${date.getFullYear()}-${('0' + (date.getMonth() + 1)).slice(-2)}-${(
		'0' + date.getDate()
	).slice(-2)}`;
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
	return (
		document.fullscreenEnabled
		|| document.mozFullScreenEnabled
		|| document.webkitFullscreenEnabled
		|| document.msFullscreenEnabled
	);
}

function isFullScreen() {
	return (
		(document.fullscreenElement
			|| document.mozFullScreenElement
			|| document.webkitIsFullScreen
			|| document.msFullscreenElement
			|| null) !== null
	);
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
	document
		.querySelector('#nav-toggle-button')
		.addEventListener('click', () => {
			nav_links.classList.toggle('show');
			nav_links.style.maxHeight = nav_links.classList.contains('show')
				? nav_links.querySelector('ul').clientHeight + 'px'
				: '';
		});
});

window.addEventListener('resize', () => {
	const nav_links = document.querySelector('#nav-links');
	if (nav_links.classList.contains('show')) {
		nav_links.style.maxHeight
			= nav_links.querySelector('ul').clientHeight + 'px';
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
	const luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
	return luma > 128
		? `rgba(0, 0, 0, ${opacity})`
		: `rgba(255, 255, 255, ${opacity})`;
}

async function getServerOffset() {
	const response = await fetch('/api/v1/what-time-is-it');
	const data = await response.text();
	const offset = data.split(' ')[5];
	const h = Number.parseInt(offset.slice(0, -2), 10);
	const m = Number.parseInt(offset.slice(-2), 10);
	const serverUTCOffset = h * 60 + m;
	const utcOffset = new Date().getTimezoneOffset();
	const serverOffset = -serverUTCOffset - utcOffset;
	return serverOffset * 60 * 1000;
}

// Apply user colors
window.addEventListener('load', () => {
	setTheme();
});

function setTheme() {
	const cfg = config;

	document
		.querySelector('meta[name=theme-color]')
		.setAttribute('content', (cfg || {}).foreground_color || '#1a2741');

	if (!cfg) {
		return;
	}

	if (cfg.background_color) {
		document
			.querySelector('body')
			.style.setProperty('--background_color', cfg.background_color);
	}

	if (cfg.foreground_color) {
		document
			.querySelector('body')
			.style.setProperty('--foreground_color', cfg.foreground_color);
	}

	if (cfg.background_text_color) {
		document
			.querySelector('body')
			.style.setProperty(
				'--background_text_color',
				cfg.background_text_color,
			);
	}

	if (cfg.foreground_text_color) {
		document
			.querySelector('body')
			.style.setProperty(
				'--foreground_text_color',
				cfg.foreground_text_color,
			);
	}
}

function broadcastConfigToExtension() {
	updateConfig();
	if (
		typeof chrome !== 'undefined'
		&& typeof chrome.runtime !== 'undefined'
	) {
		chrome.runtime.sendMessage('gbkjjbecehodfeijbdmoieepgmfdlgle', {
			message: 'schedule',
			data: JSON.stringify(config),
		});
	}
}

broadcastConfigToExtension();

Object.assign(window, {
	black_or_white,
	bytes_to_color,
	canFullScreen,
	current_date,
	date_from_api,
	date_string_to_date,
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
	setTheme,
	broadcastConfigToExtension,
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

		element.innerHTML = element.innerHTML
			.replaceAll('{START}', human_time(period.start))
			.replaceAll('{END}', human_time(period.end))
			.replaceAll(
				'{START_IN}',
				human_time_left(period.start, undefined, true),
			)
			.replaceAll(
				'{END_IN}',
				human_time_left(period.end, undefined, true),
			)
			.replaceAll(
				'{NAME}',
				period.url
					? `<a href="${period.url}">${period.friendly_name}</a>`
					: period.friendly_name,
			);
		return true;
	}

	element.innerHTML = 'No School';
	return false;
}

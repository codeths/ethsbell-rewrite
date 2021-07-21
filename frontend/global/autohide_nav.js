let nav_timeout;

function show_nav() {
	document.querySelector('nav').classList.remove('hidden');
	document.querySelector('nav').classList.add('shown');
	if (nav_timeout) {
		clearTimeout(nav_timeout);
	}

	nav_timeout = setTimeout(hide_nav, 5000);
}

function hide_nav() {
	if (window.innerWidth > 576 || document.querySelector('nav').classList.contains('show')) {
		document.querySelector('nav').classList.remove('shown');
		document.querySelector('nav').classList.add('hidden');
	}
}

window.addEventListener('mousemove', show_nav);
show_nav();

const refer = new URL(document.referrer || 'https://example.com');
const our_host = window.location.origin;
const their_host = refer.origin;

if (our_host !== their_host && !window.location.search.includes('no_default')) {
	try {
		const cfg = JSON.parse(localStorage.getItem('schedule'));
		const page = cfg.default_page || '/';
		if (!window.location.pathname !== page) {
			window.location.href = `${page}${window.location.search}`;
		}
	} catch {}
}

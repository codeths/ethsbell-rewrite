function assert(value) {
	if (!value) {
		throw new Error('Assertion failed.');
	}
}

async function hasIssue() {
	try {
		const data = await fetch('https://ethsbell.instatus.com/history.atom').catch(() => null);
		assert(data && data.ok);

		const str = await data.text();
		assert(str);

		const dom = new window.DOMParser().parseFromString(str, 'text/xml');
		assert(dom);

		const events = dom.querySelectorAll('feed>entry');
		const active_events = [];
		for (const event of events) {
			const updates = event.querySelectorAll('content>p');
			assert(updates && updates.length > 0);

			const most_recent = updates[updates.length - 1];
			const status = most_recent.querySelector('strong').textContent;
			assert(status);
			if (status !== 'Resolved') {
				const id = event.querySelector('id').textContent;
				assert(id);
				active_events.push(id);
			}
		}

		return active_events.length > 0;
	} catch (error) {
		console.error(error);
		return false;
	}
}

(async () => {
	for (const element of document.querySelectorAll('.advisory')) {
		element.innerHTML = '';
	}

	if (await hasIssue()) {
		for (const element of document.querySelectorAll('.advisory')) {
			const narrow_screen = (screen.availWidth / screen.availHeight) < (4 / 3);
			element.innerHTML = `<a href="https://ethsbell.instatus.com" target="_blank">${narrow_screen ? '!!!' : 'ETHSBell is having issues;<br>click here for more info.'}</a>`;
		}
	}
})();


for (const element of document.querySelectorAll('.advisory')) {
	(async () => {
		let text = await fetch('/advisory.html')
			.then(response => (response.ok ? response.text() : ''))
			.then(text => text.trim())
			.catch(() => '');

		if (!text) {
			return;
		}

		const firstLine = text.slice(0, text.indexOf('\n') + 1);
		let id = null;
		if (firstLine && firstLine.startsWith('#')) {
			id = firstLine.slice(1).trim();
			text = text.slice(text.indexOf('\n') + 1);
		}

		if (id && getCookie('advisory') === id) {
			return;
		}

		element.innerHTML = text;

		const closebtn = document.createElement('button');
		closebtn.innerHTML = '&times;';
		closebtn.classList.add('advisory-close');
		closebtn.addEventListener('click', () => {
			if (id) {
				setCookie('advisory', id);
			}

			element.remove();
		});
		element.append(closebtn);
	})();
}

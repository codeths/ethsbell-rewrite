for (const element of document.querySelectorAll('.advisory')) {
	(async () => {
		element.innerHTML = await fetch('/advisory.html').then(response => response.ok ? response.text() : '');
	})();
}


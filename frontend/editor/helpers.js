function $(id) {
	return document.querySelector(`#${id}`)
}

async function req(url, request, error_span) {
	request = {
		headers: {
			Authorization
		},
		...request
	}
	let result = await fetch(url, request)
	if (result.ok) {
		$(error_span).innerHTML = ""
		return result
	} else {
		let error = `Failed with ${result.status}: ${await result.text()}`
		$(error_span).innerHTML = error
		throw new Error(error)
	}
}
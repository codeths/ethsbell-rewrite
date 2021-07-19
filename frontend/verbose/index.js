async function get() {
	return (await fetch('../api/v1/today/now/near')).json()
}

function process(data) {
	// TODO: This will perform class name replacements
	return data
}

function display(data) {
	// Display previous
	getel("prev_start").innerHTML = data[0] ? `from ${data[0].start}` : ""
	getel("prev_end").innerHTML = data[0] ? `until ${data[0].end}` : ""
	getel("prev_name").innerHTML = data[0].friendly_name || "None"
	// Display next
	getel("next_start").innerHTML = data[2] ? `at ${data[2]?.start}` : ""
	getel("next_end").innerHTML = data[2] ? `until ${data[2]?.end}` : ""
	getel("next_name").innerHTML = data[2]?.friendly_name || "None"
	// Display currents
	let currents = []
	for(const current of data[1]) {
		let new_text = getel("current").innerHTML;
		console.log(current);
		currents.push(new_text
			.replace("CURR_START", current.start)
			.replace("CURR_NAME", current.friendly_name)
			.replace("CURR_END", current.end))
	}
	getel("current_parent").innerHTML = currents.join(getel("current_separator"))
}

function getel(id) {
	return document.querySelector(`#${id}`)
}

async function go() {
	let data = await get()
	data = process(data)
	display(data)
}

go()
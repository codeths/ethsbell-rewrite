async function get() {
	return (await fetch('../api/v1/today/now/near')).json()
}

function process(data) {
	// TODO: This will perform class name replacements
	return data
}

function getel(id) {
	return document.querySelector(`#${id}`)
}

async function go() {
	let data = await get()
	data = process(data)
	display(data)
	window.setTimeout(go, 5*60*1000)
}

function human_list(items) {
	let output = "";
	if (items.length == 1) {
		return items[0].toString()
	}
	for (let i = 0; i < items.length; i++) {
		if (i == items.length - 1) {
				output += `${items.length > 2 ? ', ' : ' ' }and ${items[i].toString()}`
		} else if (i != 0) {
			output += `, ${items[i].toString()}`
		} else {
			output += items[i].toString()
		}
	}
	return output
}
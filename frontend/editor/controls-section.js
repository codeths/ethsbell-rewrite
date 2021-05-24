$("lock").addEventListener("click", async () => {
	let response = await (await req("../api/v1/lock", {}, "controls_error")).json()
	if (!response.includes("OK")) {
		let date = new Date(response)
		$("controls_error").innerHTML = `Locked previously at ${date.toLocaleString()}`
	}
})

$("unlock").addEventListener("click", async () => {
	let response = await (await req("../api/v1/force-unlock", {}, "controls_error")).text()
})

let data

function check_data(error_span) {
	if (!data) {
		$(error_span).innerHTML = "No data"
		throw new Error("No data")
	}
}

$("pull").addEventListener("click", async () => {
	data = await (await req("../api/v1/spec", {}, "controls_error")).json();
	update_view()
})

$("push").addEventListener("click", async () => {
	check_data("controls_error");
	await req("../api/v1/spec", {
		method: "POST",
		body: JSON.stringify(data)
	}, "controls_error")
})

$("select_period").addEventListener("change", update_view)
$("select_schedule").addEventListener("change", () => {
	$("select_period").value = 0
	update_view()
})

let schedule_name
let period_index
let schedule
let period

function update_view() {
	// Fill drop-down menus
	schedule_name = $("select_schedule").value || Object.keys(data.schedule_types)[0]
	period_index = $("select_period").value || 0
	$('select_period').innerHTML = ''
	$('select_schedule').innerHTML = ''
	for (const schedule_type of Object.keys(data.schedule_types)) {
		$('select_schedule').innerHTML += `<option value="${schedule_type}">${data.schedule_types[schedule_type].friendly_name}</option>`
	}
	$('select_schedule').value = schedule_name
	for (let index = 0; index < data.schedule_types[schedule_name].periods.length; ++index) {
		let period = data.schedule_types[schedule_name].periods[index]
		$('select_period').innerHTML += `<option value="${index}">${period.friendly_name}</option>`
	}
	$('select_period').value = period_index
	// Load current data
	schedule = data.schedule_types[schedule_name]
	period = schedule.periods[period_index]
	// Fill form fields
	$('schedule_friendly_name').value = schedule.friendly_name
	$('schedule_regex').value = schedule.regex
	if (period) {
		$('period_friendly_name').value = period.friendly_name
		$('period_start').value = period.start
		$('period_end').value = period.end
		if (typeof period.kind === 'string') {
			$('period_kind').value = period.kind
			$('period_class_index').disabled = true
		} else {
			$('period_kind').value = "Class"
			$('period_class_index').disabled = false
			$('period_class_index').value = period.kind.Class
		}
	}
}

// Schedule add, copy, and remove
$('add_schedule').addEventListener("click", () => {
	check_data("controls_error");
	let new_name = prompt("Set the internal name for the new schedule type (like no_school or orange_day)")
	if (new_name.length > 0) {
		data.schedule_types[new_name] = {
			friendly_name: new_name,
			periods: [],
			regex: ""
		}
	}
	update_view()
	$('select_schedule').value = new_name
	update_view()
})
$('copy_schedule').addEventListener("click", () => {
	check_data("controls_error");
	let new_name = prompt("Set the internal name for the newly copied schedule (like no_school or orange_day)")
	if (new_name.length > 0) {
		data.schedule_types[new_name] = JSON.parse(JSON.stringify(data.schedule_types[schedule_name]))
		data.schedule_types[new_name].friendly_name = new_name
	}
	update_view()
	$('select_schedule').value = new_name
	update_view()
})
$('remove_schedule').addEventListener("click", () => {
	check_data("controls_error");
	let response = confirm(`Do you really want to delete ${schedule_name}?`)
	if (response) {
		delete data.schedule_types[schedule_name]
		$('select_schedule').value = Object.keys(data.schedule_types)[0]
		update_view()
	}
})

// Period add, copy, and remove
$('add_period').addEventListener("click", () => {
	check_data("controls_error");
	let new_name = prompt("Set the name for the new period (like First Period or Lunch)")
	if (new_name.length > 0) {
		data.schedule_types[schedule_name].periods.push({
			friendly_name: new_name,
			start: "00:00:00",
			end: "00:00:00",
			kind: "Break"
		})
	}
	update_view()
	$('select_period').value = data.schedule_types[schedule_name].periods.length - 1
	update_view()
})
$('remove_period').addEventListener("click", () => {
	check_data("controls_error");
	let response = confirm(`Do you really want to delete ${period_index}?`)
	if (response) {
		data.schedule_types[schedule_name].periods.remove(period_index)
		$('select_period').value = 0
		update_view()
	}
})

// Handle form changes
$('schedule_friendly_name').addEventListener("change", event => {
	check_data("controls_error");
	data.schedule_types[schedule_name].friendly_name = event.target.value
})
$('schedule_regex').addEventListener('change', event => {
	check_data("controls_error");
	data.schedule_types[schedule_name].regex = event.target.value
})
$('period_friendly_name').addEventListener("change", event => {
	check_data("controls_error");
	data.schedule_types[schedule_name].periods[period_index].friendly_name = event.target.value
})
$('period_start').addEventListener("change", event => {
	check_data("controls_error");
	data.schedule_types[schedule_name].periods[period_index].start = event.target.value
})
$('period_end').addEventListener("change", event => {
	check_data("controls_error");
	data.schedule_types[schedule_name].periods[period_index].end = event.target.value
})
$('period_kind').addEventListener("change", event => {
	check_data("controls_error");
	if (event.target.value === "Class") {
		data.schedule_types[schedule_name].periods[period_index].kind = {
			Class: $('period_class_index').value
		}
		$('period_class_index').disabled = false
	} else {
		data.schedule_types[schedule_name].periods[period_index].kind = event.target.value
		$('period_class_index').disabled = true
	}
})
$('period_class_index').addEventListener("change", event => {
	check_data("controls_error");
	data.schedule_types[schedule_name].periods[period_index].kind.Class = event.target.value
})
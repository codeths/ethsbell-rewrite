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
	for (const current of data[1]) {
		let new_text = getel("current").innerHTML;
		console.log(current);
		currents.push(new_text
			.replace("CURR_START", current.start)
			.replace("CURR_NAME", current.friendly_name)
			.replace("CURR_END", current.end))
	}
	getel("current_parent").innerHTML = currents.join(getel("current_separator").innerHTML)
	place_boxes(data)
}

go()

const viewport_minutes = 90 // The number of minutes the viewport should show
const row_height = 50 // The height of a row
const box_height = 40 // The height of a box
const row_start = 10 // The height rows start at
const text_height = 30 // The size of the font

/// Place period boxes for a list of periods.
function place_boxes(data) {
	// Resolve rows so everything is mutually non-intersecting.
	let boxes = []
	data = data.flat().filter(v => v)
	data.sort((a, b) => {
		console.log(a, b);
		return a.start - b.start
	})
	for (const period of data) {
		// Set up variables
		let start = period.start_timestamp
		let row = 0
		let intersecting = 0
		// Increment the row until nothing intersects.
		do {
			intersecting = 0
			for (const box of boxes) {
				if (box.row == row && box.end > start) {
					intersecting += 1
				}
			}
			if (intersecting > 0) {
				row += 1
			}
		} while (intersecting > 0)
		boxes.push({
			row: row,
			start: period.start_timestamp,
			end: period.end_timestamp,
			kind: period.kind,
			name: period.friendly_name,
		})
	}
	// Determine where the boxes should be placed on-screen
	let center = window.innerWidth / 2
	minutes_to_pixels = window.innerWidth / viewport_minutes
	let now = (Date.now() / 1000)
	for (const box of boxes) {
		let length = (box.end - box.start) / 60
		let d_time = now - box.start
		let from_center = d_time * minutes_to_pixels / 60
		let x = Math.max(center - from_center, 5)
		let w = Math.min((length * minutes_to_pixels), window.innerWidth - 5 - x)
		box.w = w
		box.x = x
		box.y = row_start + (box.row * row_height)
		box.h = box_height
		box.th = text_height
		let text_margin = (box_height - text_height) / 2
		box.tx = x + text_margin
		box.ty = box.y + text_height
	}
	// Set the box's emoji and TODO color
	for (const box of boxes) {
		let emoji;
		if (box.kind.Class) {
			emoji = "🏫"
		} else if (box.kind.ClassOrLunch) {
			emoji = "🏫/🥪"
		} else {
			switch (box.kind) {
				case "Lunch":
					emoji = "🥪";
					break;
				case "Break":
					emoji = "🛌";
					break;
				case "AMSupport":
					emoji = "🐔";
					break;
				case "Passing":
					emoji = "🏃";
					break;
				case "BeforeSchool":
					emoji = "🌄";
					break;
				case "AfterSchool":
					emoji = "🌇";
					break;
				default:
					emoji = "😕";
			}
		}
		box.emoji = emoji;
		box.color = 'white'
	}
	console.log(boxes);
	// Write the boxes to the DOM
	let output = ""
	for (const box of boxes) {
		output += getel("period_box").innerHTML
			.replace("X", box.x)
			.replace("Y", box.y)
			.replace("W", box.w)
			.replace("H", box.h)
			.replace("TX", box.tx)
			.replace("TY", box.ty)
			.replace("TH", box.th)
			.replace("COLOR", box.color)
			.replace("CONTENT", `${box.emoji} ${box.name}`)
	}
	getel("boxes").innerHTML = output
}
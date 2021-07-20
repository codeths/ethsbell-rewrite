async function display(data) {
	// Display previous
	getel("prev_start").innerHTML = data[0] ? `from ${human_time(data[0].start)}` : ""
	getel("prev_end").innerHTML = data[0] ? `until ${human_time(data[0].end)}` : ""
	getel("prev_name").innerHTML = data[0].friendly_name || "None"
	// Display next
	getel("next_start").innerHTML = data[2] ? `at ${human_time(data[2]?.start)}` : ""
	getel("next_end").innerHTML = data[2] ? `until ${human_time(data[2]?.end)}` : ""
	getel("next_name").innerHTML = data[2]?.friendly_name || "None"
	// Display currents
	let currents = []
	for (const current of data[1]) {
		let new_text = getel("current").innerHTML;
		currents.push(new_text
			.replace("CURR_START", human_time(current.start))
			.replace("CURR_NAME", current.friendly_name)
			.replace("CURR_END", human_time(current.end)))
	}
	getel("current_parent").innerHTML = currents.join(getel("current_separator").innerHTML)
	let all_data = await get("api/v1/today").then(v=>v.periods)
	place_boxes(all_data)
}

go()

const viewport_seconds = 3600*3 // The number of minutes the viewport should show
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
	let now = (Date.now() / 1000) + 3600 * 19
	for (const box of boxes) {
		let length = (box.end - box.start)
		let relative_time = box.start - now
		let fraction_time = relative_time / viewport_seconds
		let fraction_position = 0.5 + fraction_time
		let fraction_outside_length = 0;
		if (fraction_position < 0) {
			fraction_outside_length = -fraction_position
			fraction_position = 0
		}
		let absolute_x = window.innerWidth * fraction_position
		fraction_position *= 100
		let fraction_length = length / viewport_seconds
		fraction_length -= fraction_outside_length
		if (fraction_length <= 0) {
			box.hidden = true
		} else {
			fraction_length *= 100
			if (fraction_length + fraction_position > 100) {
				fraction_length = 100 - fraction_position
			}
			box.x = fraction_position
			box.y = row_start + (row_height * box.row)
			box.w = fraction_length
			box.h = box_height
			let text_margin = (box_height - text_height) / 2
			box.tx = absolute_x + text_margin
			box.ty = box.y + (box.h/2) + text_margin*2
			box.th = text_height
		}
	}
	// Set the box's emoji and TODO color
	for (const box of boxes) {
		if (box.hidden) {
			continue
		}
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
				case "Announcements":
					emoji = "📣"
					break;
				default:
					emoji = emoji || "😕";
			}
		}
		box.emoji = emoji;
		box.color = 'white'
	}
	// Write the boxes to the DOM
	let output = ""
	for (const box of boxes) {
		if (box.hidden) {
			continue
		}
		output += getel("period_box").innerHTML
			.replace("X", box.x)
			.replace("Y", box.y)
			.replace("W", box.w)
			.replace("H", box.h)
			.replace("TX", box.tx)
			.replace("TY", box.ty)
			.replace("TH", box.th)
			.replace("COLOR", box.color)
			.replace("CONTENT", box.w > 10 ? `${box.emoji} ${box.name}` : box.emoji)
	}
	getel("boxes").innerHTML = output
}
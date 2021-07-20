function display(data) {
	let content = ''
	if (data[1]) {
		let names = data[1].map(period => period.friendly_name)
		let ends = data[1].map(period => human_time(period.end))
		content += `
		${data[1].length > 1 ? 'The current periods are ' : 'It is currently'} 
		${human_list(names)},
		which ${data[1].length > 1 ? 'end' : 'ends'} at
		${human_list(ends)}${data[1].length > 1 ? ', respectively.' : '.'}`
		if (data[2]) {
			content += ` The next period is ${data[2].friendly_name}, which begins at ${data[2].start}`
		}
	} else {
		content = "There is no current period."
	}
	getel("content").innerHTML = content
}

go()
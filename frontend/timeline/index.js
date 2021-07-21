async function display(data) {
	// Display previous
	getel('prev_start').innerHTML = data[0] ? `from ${human_time(data[0].start)}` : '';
	getel('prev_end').innerHTML = data[0] ? `until ${human_time(data[0].end)}` : '';
	getel('prev_name').innerHTML = period_html(data[0]);
	// Display next
	getel('next_start').innerHTML = data[2] ? `at ${human_time(data[2]?.start)}` : '';
	getel('next_end').innerHTML = data[2] ? `until ${human_time(data[2]?.end)}` : '';
	getel('next_name').innerHTML = period_html(data[2]);
	// Display currents
	const currents = [];
	const ends = [];
	for (const current of data[1]) {
		const new_text = getel('current').innerHTML;
		currents.push(new_text
			.replace('CURR_START', human_time(current.start))
			.replace('CURR_NAME', period_html(current))
			.replace('CURR_END', human_time(current.end)));
		ends.push(human_time_left(current.end));
	}

	getel('current_parent').innerHTML = currents.join(getel('current_separator').innerHTML);
	getel('time_left').innerHTML = `Ending in ${human_list(ends)}${ends.length > 1 ? ', respectively.' : '.'}`;
	all_data = await get('api/v1/today').then(v => v.periods);
	place_boxes(all_data);
}

go();

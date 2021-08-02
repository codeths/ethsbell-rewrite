function update_progress(data) {
	const progress_parent = getel('progress_parent');
	if (!progress_parent) return;
	progress_parent.innerHTML = '';
	for (const period of data[1]) {
		const progress = document.createElement('div');
		progress.classList.add('progress_bar');
		progress.id = `progress_bar_${period.friendly_name.split(' ').join('_')}`;
		const length = period.end_timestamp - period.start_timestamp;
		const now = (Date.now() / 1000) - period.start_timestamp;
		const position = now / length;
		progress.style.setProperty('--width', `${position * 100}%`);
		progress.title = `${period.friendly_name} progress`;
		progress_parent.append(progress);
	}
}

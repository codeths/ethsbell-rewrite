function update_progress(data) {
	const progress_parent = getel('progress_parent');
	if (!progress_parent) {
		return;
	}

	data = data[1].filter(x => !['BeforeSchool', 'AfterSchool'].includes(x.kind));

	progress_parent.innerHTML = '';
	if (data.length === 0) {
		progress_parent.style.display = 'none';
	} else {
		progress_parent.style.display = 'block';
		for (const period of data) {
			const progress = document.createElement('div');
			progress.classList.add('progress_bar');
			progress.id = `progress_bar_${period.friendly_name.split(' ').join('_')}`;
			const length = (date_from_api(period.end) - date_from_api(period.start)) / 1000;
			const now = (current_date().getTime() - date_from_api(period.start)) / 1000;
			const position = now / length;
			progress.style.setProperty('--width', `${position * 100}%`);
			progress.title = `${period.friendly_name} progress`;
			progress_parent.append(progress);
		}
	}
}

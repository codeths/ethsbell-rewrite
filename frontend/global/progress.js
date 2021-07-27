function update_progress(data) {
	let progress_parent = getel('progress_parent')
	progress_parent.innerHTML = ''
	for (const period of data[1]) {
		let progress = document.createElement('div')
		progress.classList.add('progress_bar')
		progress.id = `progress_bar_${period.friendly_name.split(' ').join('_')}`
		let length = period.end_timestamp - period.start_timestamp
		let now = (Date.now() / 1000) - period.start_timestamp
		let position = now / length
		progress.style.setProperty('--width', `${position * 100}%`)
		progress.title = `${period.friendly_name} progress`
		progress_parent.appendChild(progress)
	}
}
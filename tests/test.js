const fetch = require('node-fetch');

test('The schedule is built correctly', async done => {
	const result = await fetch('http://bell:8000/api/v1/schedule').catch(error => error);
	if (result instanceof Error) {
		console.log(result);
		expect(result instanceof Error).toBeFalsy();
	}

	const object = await result.json().catch(error => error);
	if (object instanceof Error) {
		console.log(object);
		expect(object instanceof Error).toBeFalsy();
	}

	expect(object.definition.override_calendar_url).toBe('http://ws/override.ics');
	expect(object.definition.schedule_types).toStrictEqual({
		no_school: {
			friendly_name: 'No School',
			periods: [{
				friendly_name: 'Test Period 1',
				start: '08:00:00',
				start_timestamp: 0,
				end: '12:00:00',
				end_timestamp: 0,
				kind: {
					Class: 0,
				},
			}],
			regex: '(No School)|(Non-Attendance)',
		},
		monday: {
			friendly_name: 'Async Monday',
			periods: [{
				friendly_name: 'Test Period 2',
				start: '12:00:00',
				start_timestamp: 0,
				end: '15:00:00',
				end_timestamp: 0,
				kind: {
					Class: 1,
				},
			}],
			regex: 'FORCE MONDAY',
		},
	});
	expect(object.definition.typical_schedule).toStrictEqual([
		'no_school',
		'monday',
		'no_school',
		'no_school',
		'no_school',
		'no_school',
		'no_school',
	]);
	expect(object.calendar['2020-02-20'].includes({ScheduleOverride: 'no_school'}));
	expect(object.calendar['2020-02-21'].includes({ScheduleOverride: 'monday'}));
	done();
});

test('The schedule for today is correct', async done => {
	const date = new Date();
	date.setFullYear(2020, 2, 20);
	date.setHours(12, 0, 0);
	const response = await fetch(`http://bell:8000/api/v1/today?timestamp=${date.getTime()}`).catch(error => error);
	expect(response instanceof Error).toBeFalsy();
	const json = await response.json().catch(error => error);
	expect(json instanceof Error).toBeFalsy();
	const start = json.periods[0].start_timestamp;
	const end = json.periods[0].end_timestamp;
	expect(json).toStrictEqual({
		friendly_name: 'No School',
		periods: [
			{
				friendly_name: 'Test Period 1',
				start: '08:00:00',
				start_timestamp: start,
				end: '12:00:00',
				end_timestamp: end,
				kind: {
					Class: 0,
				},
			},
		],
		regex: '(No School)|(Non-Attendance)',
	});
	done();
});

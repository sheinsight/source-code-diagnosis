function resolveAfter2Seconds(x) {
	return new Promise((resolve) => {
		setTimeout(() => {
			resolve(x);
		}, 2000);
	});
}

async function f1() {
	const x = await resolveAfter2Seconds(10);
}

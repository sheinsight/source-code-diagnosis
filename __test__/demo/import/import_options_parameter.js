import("./module.js", { a: 1 })
	.then((module) => {
		console.log(module.default);
	})
	.catch((error) => {
		console.error("Error importing module:", error);
	});

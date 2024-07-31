// Using a named property
const obj = {
	f: async function* () {
		yield 1;
		yield 2;
		yield 3;
	},
};

// The same object using shorthand syntax
const obj2 = {
	async *f() {
		yield 1;
		yield 2;
		yield 3;
	},
};

const obj3 = {
	async f() {},
};

const obj4 = {
	f: async function a() {},
};

const obj5 = {
	*f() {},
};

const obj6 = {
	fL: function* a() {},
};

const obj7 = {
	fL: function a() {},
};

// function a() {}

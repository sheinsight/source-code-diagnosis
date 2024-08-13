import fs from "node:fs";

const URL = [
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/classes.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/functions.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/grammar.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/regular_expressions.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/statements.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/addition.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/addition_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/async_function.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/async_generator_function.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/await.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_and.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_and_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_not.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_or.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_or_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_xor.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/bitwise_xor_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/class.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/comma.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/conditional.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/decrement.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/delete.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/destructuring.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/division.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/division_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/equality.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/exponentiation.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/exponentiation_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/function.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/generator_function.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/greater_than.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/greater_than_or_equal.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/grouping.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/import.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/import_meta.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/in.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/increment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/inequality.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/instanceof.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/left_shift.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/left_shift_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/less_than.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/less_than_or_equal.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_and.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_and_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_not.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_or.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/logical_or_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/multiplication.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/multiplication_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/new.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/new_target.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/null.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/nullish_coalescing.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/nullish_coalescing_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/object_initializer.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/optional_chaining.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/property_accessors.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/remainder.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/remainder_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/right_shift.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/right_shift_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/spread.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/strict_equality.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/strict_inequality.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/subtraction.json",
	"https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/subtraction_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/super.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/this.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/typeof.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/unary_negation.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/unary_plus.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/unsigned_right_shift.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/unsigned_right_shift_assignment.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/void.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/yield.json",
	// "https://raw.githubusercontent.com/mdn/browser-compat-data/main/javascript/operators/yield_star.json",
];

const res = [];

const names = [];

console.log(Map.groupBy);

for (const url of URL) {
	console.log(url);
	const content = await (await fetch(url)).json();
	const name_list = flattenName(content);
	names.push(...name_list);
	const flattened = flattenObject(content);
	res.push(...flattened);
}

fs.writeFileSync("./browser_compat_data.json", JSON.stringify(res, null, 2));
fs.writeFileSync("./browser_compat_names.json", JSON.stringify(names, null, 2));

function flattenName(obj, parentKey = null, result = []) {
	for (const key in obj) {
		if (key === "__compat") {
			result.push(parentKey);
		} else {
			const pk = parentKey ? `${parentKey}_${key}` : key;
			flattenName(obj[key], pk, result);
		}
	}
	return result;
}

function handle_support(obj) {
	const res = {};
	for (const key of Object.keys(obj)) {
		const item = obj[key];
		if (Array.isArray(item)) {
			res[key] = item[0].version_added;
		} else if (item === "mirror") {
			if (["chrome_android", "oculus", "opera"].includes(key)) {
				// biome-ignore lint/complexity/useLiteralKeys: <explanation>
				const chrome = obj["chrome"];
				if (Array.isArray(chrome)) {
					res[key] = chrome[0].version_added;
				} else {
					res[key] = chrome.version_added;
				}
			}
		} else {
			res[key] = item.version_added;
		}
		// console.log(key);
	}
	return res;
	// return Object.entries(obj).reduce((acc, [browser, support]) => {
	// 	console.log(browser, support);
	// 	if (Array.isArray(support)) {
	// 		acc[browser] = support[0].version_added;
	// 	} else {
	// 		acc[browser] = support.version_added;
	// 	}
	// 	return acc;
	// });
}

function flattenObject(obj, parentKey = null, result = []) {
	for (const key in obj) {
		if (key === "__compat") {
			const item = obj[key];
			// handle_support(item.support);
			// item.support = handle_support(item.support);
			result.push({
				[parentKey]: item,
			});
		} else {
			const pk = parentKey ? `${parentKey}_${key}` : key;
			flattenObject(obj[key], pk, result);
		}
	}
	return result;
}

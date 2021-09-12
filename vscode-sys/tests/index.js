const path = require('path');
const Mocha = require('mocha');
const glob = require('glob');
const rust = require('vs-code-sys');

function run() {
	// Create the mocha test
	const mocha = new Mocha({
		ui: 'tdd',
		color: true
	});

	const testsRoot = path.resolve(__dirname, '..');

	return new Promise((c, e) => {
		rust.print();
	});
}

module.exports = {
	run
};

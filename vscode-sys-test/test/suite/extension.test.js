const assert = require('assert');

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
global.vscode = require('vscode');
// const myExtension = require('../extension');
const rust = require('../../pkg/vscode_sys_test');

suite('VS Code Sys Test Suite', () => {
	test('show information message', () => {
		var promise = rust.test_show_information_message_test();

		return promise.then(
			function(value) { assert.equal(value, "btn1") },
			function(error) { console.log(error) }
		);
	}).timeout(5000);

	test('register command', () => {
		rust.register_command_test();
	});
});

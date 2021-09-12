const assert = require('assert');

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
global.vscode = require('vscode');
// const myExtension = require('../extension');
const rust = require('../../pkg/vscode_sys_test');

suite('VS Code Sys Test Suite', () => {
	vscode.window.showInformationMessage('Start all tests.');

	test('show information message', () => {
		rust.test_show_information_message();
	});

	test('register command', () => {
		rust.register_command();
	});
});

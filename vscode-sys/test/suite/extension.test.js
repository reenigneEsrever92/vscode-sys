const assert = require('assert');

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
const vscode = require('vscode');
// const myExtension = require('../extension');
const rust = require('./pkg/vs_code_sys')

suite('Extension Test Suite', () => {
	vscode.window.showInformationMessage('Start all tests.');

	test('Sample test', () => {
		rust.
	});
});

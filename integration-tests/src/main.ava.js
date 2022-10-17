"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const near_workspaces_1 = require("near-workspaces");
const ava_1 = __importDefault(require("ava"));
const test = ava_1.default;
test.beforeEach((t) => __awaiter(void 0, void 0, void 0, function* () {
    // Init the worker and start a Sandbox server
    const worker = yield near_workspaces_1.Worker.init();
    // Deploy contract
    const root = worker.rootAccount;
    const contract = yield root.createSubAccount('test-account');
    // Get wasm file path from package.json test script in folder above
    yield contract.deploy(process.argv[2]);
    // Save state for test runs, it is unique for each test
    t.context.worker = worker;
    t.context.accounts = { root, contract };
}));
test.afterEach((t) => __awaiter(void 0, void 0, void 0, function* () {
    // Stop Sandbox server
    yield t.context.worker.tearDown().catch((error) => {
        console.log('Failed to stop the Sandbox:', error);
    });
}));
test('returns the default greeting', (t) => __awaiter(void 0, void 0, void 0, function* () {
    const { contract } = t.context.accounts;
    const message = yield contract.view('get_greeting', {});
    t.is(message, 'Hello');
}));
test('changes the message', (t) => __awaiter(void 0, void 0, void 0, function* () {
    const { root, contract } = t.context.accounts;
    yield root.call(contract, 'set_greeting', { message: 'Howdy' });
    const message = yield contract.view('get_greeting', {});
    t.is(message, 'Howdy');
}));

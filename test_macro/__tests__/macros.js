const native = require('../native');

describe('macro functions', () => {
    test('Hello, World!', () => {
        expect(native.say_hello('World')).toBe('Hello, World!');
        expect(native.say_hello('Alice')).toBe('Hello, Alice!');
        expect(native.say_hello('Bob')).toBe('Hello, Bob!');
    })

    test("Greet users", () => {
        expect(native.greet({ name: 'Bob', age: 32 })).toBe('Bob is 32 years old');
        expect(native.greet({ name: 'Alice', age: 27 })).toBe('Alice is 27 years old');
    })

    test("fibonacci", () => {
        expect(native.fibonacci(5)).toBe(5);
        expect(native.fibonacci(10)).toBe(55);
    })

    describe("maybe_say_hello", () => {
        test("existing user", () => {
            expect(native.maybe_say_hello({ name: 'Bob', age: 32 })).toBe('Bob is 32 years old');
        })

        test("null user", () => {
            expect(native.maybe_say_hello()).toBe(null);
            expect(native.maybe_say_hello(undefined)).toBe(null);
            expect(native.maybe_say_hello(null)).toBe(null);
        })
    })
});

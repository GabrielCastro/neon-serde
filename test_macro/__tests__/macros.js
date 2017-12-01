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
});

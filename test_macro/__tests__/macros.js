const native = require('../native');
const expect = require('expect');

describe('macro functions', () => {
    it('Hello, World!', () => {
        expect(native.say_hello('World')).toBe('Hello, World!');
        expect(native.say_hello('Alice')).toBe('Hello, Alice!');
        expect(native.say_hello('Bob')).toBe('Hello, Bob!');
    })

    it("Greet users", () => {
        expect(native.greet({ name: 'Bob', age: 32 })).toBe('Bob is 32 years old');
        expect(native.greet({ name: 'Alice', age: 27 })).toBe('Alice is 27 years old');
    })

    it("fibonacci", () => {
        expect(native.fibonacci(5)).toBe(5);
        expect(native.fibonacci(10)).toBe(55);
    })

    it("buffers", () => {
        expect(native.sort_utf8_bytes("hello world"))
          .toEqual(new Buffer(" dehllloorw", 'ascii'))

        native.expect_buffer_only(new Buffer('000011110000', 'hex'))
        expect(() => {
          native.expect_buffer_only([1, 2, 3, 4])
        }).toThrow(/failed downcast to Buffer/)

        native.expect_array([0,0,0,0])
    })

    describe("maybe_say_hello", () => {
        it("existing user", () => {
            expect(native.maybe_say_hello({ name: 'Bob', age: 32 })).toBe('Bob is 32 years old');
        })

        it("null user", () => {
            expect(native.maybe_say_hello()).toBe(null);
            expect(native.maybe_say_hello(undefined)).toBe(null);
            expect(native.maybe_say_hello(null)).toBe(null);
        })
    })
});

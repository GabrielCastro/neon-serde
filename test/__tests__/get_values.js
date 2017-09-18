const native = require('../native');

describe('all values ok', () => {
    test('value 32', () => {
        expect(native.make_num_32()).toBe(32);
    });

    test('value 77', () => {
        expect(native.make_num_77()).toBe(77);
    });

    test('value Hello World', () => {
        expect(native.make_str_hello()).toBe('Hello World');
    });

    test('value array', () => {
        expect(native.make_num_array()).toEqual([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    });

    test('value object', () => {
        expect(native.make_obj()).toEqual({
            'a': 1,
            'b': [0.1, 1.1, 2.2, 3.3],
            'c': 'Hi'
        });
    });

    test('value map', () => {
        expect(native.make_map()).toEqual({
            'a': 1,
            'b': 2,
            'c': 3,
        });
    });

    test('make object', () => {
        expect(native.make_object()).toEqual({
            a: 1,
            b: [1, 2],
            c: "abc",
            d: false,
            e: null,
            f: null,
            g: [9, false, "efg"],
            h: '\uD83E\uDD37',
            i: "Empty",
            j: {Tuple: [27, "hij"]},
            k: {Struct: { a: 128, b: [9, 8, 7]}},
            l: "jkl",
            m: [0,1,2,3,4],
            o: {Value: ['z', 'y', 'x']}
        });
    });

    test('make_buff', () => {
        const buff = new Buffer([255, 254, 253]);
        expect(native.make_buff()).toEqual(buff);
    });

    test('expect_hello_world', () => {
        native.expect_hello_world("hello world");
    });

    test('expect_obj', () => {
        native.expect_obj({
            a: 1,
            b: [1, 2],
            c: "abc",
            d: false,
            e: null,
            f: undefined,
            g: [9, false, "efg"],
            h: '\uD83E\uDD37',
            i: "Empty",
            j: {Tuple: [27, "hij"]},
            k: {Struct: { a: 128, b: [9, 8, 7]}},
            l: "jkl",
            m: [0,1,2,3,4],
            o: {Value: ['z', 'y', 'x']}
        });
    });

    test('expect_num_array', () => {
        native.expect_num_array([0, 1, 2, 3]);
    });

    test('expect_buffer', () => {
        native.expect_buffer(new Buffer([252, 251, 250]));
        native.expect_buffer(new Uint8Array([252, 251, 250]));
        native.expect_buffer(new Uint8ClampedArray([252, 251, 250]));
    });
});

describe('throwing functions', () => {

    test('expect_hello_world', () => {
        expect(() => native.expect_hello_world("GoodBye World")).toThrow(/assertion failed:/);
    });

    test('expect_obj', () => {
        expect(() => native.expect_obj({})).toThrow(/missing field `a`/);
    });

    test('expect_num_array', () => {
        expect(() => native.expect_num_array([0, 0, 0, 0])).toThrow(/assertion failed:/);
    });

    test('expect_buffer', () => {
        expect(() => native.expect_buffer()).toThrow(/not enough arguments/);
    });

    test('getter that throws', () => {
        const obj = {
            a: 1,
            b: [1,3]
        };
        for (const ch of 'cdefghijklmo') {
            Object.defineProperty(obj, ch, {
                enumerable: true,
                configurable: false,
                get() {
                    throw new Error('Hi There prop ' + ch);
                }
            })
        }
        expect(() => native.expect_obj(obj))
            .toThrow(/Hi There prop c/);
    })
});

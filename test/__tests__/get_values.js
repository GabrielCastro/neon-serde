const native = require('../native');

describe('all values', () => {
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
            'c': 3
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
    })

    test('make_buff', () => {
        const buff = new Buffer([255, 254, 253]);
        expect(native.make_buff()).toEqual(buff);
    });

    test('expect_hello_world', () => {
        native.expect_hello_world("hello world");
    });

    test('expect_obj', () => {
        const a = native.expect_obj({
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
        });

        console.log('a', a);
    });

    test('expect_num_array', () => {
        native.expect_num_array([0, 1, 2, 3]);
    });
});

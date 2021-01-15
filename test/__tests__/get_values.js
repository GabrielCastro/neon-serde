const native = require('../native');
const expect = require('expect');

describe('all values ok', () => {
    it('value 32', () => {
        expect(native.make_num_32()).toBe(32);
    });

    it('value 77', () => {
        expect(native.make_num_77()).toBe(77);
    });

    it('value Hello World', () => {
        expect(native.make_str_hello()).toBe('Hello World');
    });

    it('value array', () => {
        expect(native.make_num_array()).toEqual([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    });

    it('value object', () => {
        expect(native.make_obj()).toEqual({
            'a': 1,
            'b': [0.1, 1.1, 2.2, 3.3],
            'c': 'Hi'
        });
    });

    it('value map', () => {
        expect(native.make_map()).toEqual({
            'a': 1,
            'b': 2,
            'c': 3,
        });
    });

    it('make object', () => {
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
            o: {Value: ['z', 'y', 'x']},
            p: [1, 2, 3.5],
            q: 999,
            r: 333,
        });
    });

    it('make_buff', () => {
        const buff = new Buffer([255, 254, 253]);
        expect(native.make_buff()).toEqual(buff);
    });

    it('expect_hello_world', () => {
        native.expect_hello_world("hello world");
    });

    it('expect_obj', () => {
        const o = {
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
            o: {Value: ['z', 'y', 'x']},
            p: [1, 2, 3.5],
            q: 999,
            r: 333,
        };

        o.self = o;

        native.expect_obj(o);
    });

    it('expect_num_array', () => {
        native.expect_num_array([0, 1, 2, 3]);
    });

    it('expect_buffer', () => {
        native.expect_buffer(new Buffer([252, 251, 250]));
        native.expect_buffer(new Uint8Array([252, 251, 250]));

        const version = Number(process.versions.modules);

        if (version >= 57) {
            native.expect_buffer(new Uint8ClampedArray([252, 251, 250]));
        }
    });

    it('rt_rust_js_rust', () => {
        const obj = native.make_object();
        native.expect_obj(obj);
    });

    it('rt_js_rust_js', () => {
        const o = {
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
            o: {Value: ['z', 'y', 'x']},
            p: [1, 2, 3.5],
            q: 999,
            r: 333,
        };
        const o2 = native.roundtrip_object(o);
        expect(o).toEqual(o2);
    });
});

describe('throwing functions', () => {

    it('expect_hello_world', () => {
        expect(() => native.expect_hello_world("GoodBye World")).toThrow(/assertion failed:/);
    });

    it('expect_obj', () => {
        expect(() => native.expect_obj({})).toThrow(/missing field `a`/);
    });

    it('expect_num_array', () => {
        expect(() => native.expect_num_array([0, 0, 0, 0])).toThrow(/assertion failed:/);
    });

    it('expect_buffer', () => {
        expect(() => native.expect_buffer()).toThrow(/not enough arguments/);
    });

    it('getter that throws', () => {
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
            .toThrow('JS exception');
    })
});

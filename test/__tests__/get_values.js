const native = require('../native');
debugger;
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

    test.skip('value map', () => {
        expect(native.make_map()).toEqual({
            'a': 1,
            'b': 2,
            'c': 3
        });
    });

    test('expect_hello_world', () => {
        native.expect_hello_world("hello world");
    });

    test('expect_obj', () => {
        native.expect_obj({
            a: 1,
            b: [1, 2],
            c: "abc"
        });
    });

    test('expect_num_array', () => {
        native.expect_num_array([0, 1, 2, 3]);
    });
});

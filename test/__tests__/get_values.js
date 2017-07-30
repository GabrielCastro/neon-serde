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
        expect(native.make_str_hello()).toBe("Hello World");
    });test('value array', () => {
        expect(native.make_num_array()).toEqual([0,1,2,3,4,5,6,7,8,9]);
    });
});

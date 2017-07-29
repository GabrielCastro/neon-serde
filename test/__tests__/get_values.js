const native = require('../native');

describe('all values', () => {
    test('value 1', () => {
        expect(native.make_num_77()).toBe(77);
        expect(native.make_num_32()).toBe(32);
        expect(native.make_str_hello()).toBe("Hello World");
    });
});

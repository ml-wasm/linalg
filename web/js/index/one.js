import { Floats1d } from '../../../pkg/linalg.js';

export const one_dimensional_floats = () => {
    one_dimensional_floats_constructors();
    one_dimensional_floats_basics();
    one_dimensional_floats_math();
};

const one_dimensional_floats_constructors = () => {
    console.group('constructors');

    console.log((new Floats1d([1, 2, 3, 4, 5])).data);

    console.log(Floats1d.newWithZeros(5).data)
    console.log(Floats1d.newWithOnes(5).data)
    console.log(Floats1d.newWithElement(5, 2).data)

    console.log(Floats1d.newWithSimpleFunc(5, () => Math.random()).data)
    console.log(Floats1d.newWithFunc(5, (i) => i * 5).data)

    console.log(Floats1d.newWithLinspace(0, 1, 5).data);
    console.log(Floats1d.newWithRange(0, 5, 1).data);
    console.log(Floats1d.newWithGeomspace(1, 1000, 4).data);
    console.log(Floats1d.newWithLogspace(10, 0, 3, 4).data);

    console.groupEnd();
}

const one_dimensional_floats_basics = () => {
    console.group('basics');

    const a = new Floats1d([5.0, 2.0, 10.0, 4.0, 1.0]);
    const b = new Floats1d([6.0, 7.0, 8.0, 9.0, 10.0]);

    console.log(a.toString());

    console.log('a.len():', a.len());
    console.log('a.shape()', a.shape());
    console.log('a.get(2):', a.get(2));
    a.set(2, 3.0);
    console.log('a.set(2, 10.0)', a.data);
    a.swap(0, 4);
    console.log('a.swap(0, 4)', a.data);
    console.log('a.reversed()', a.reversed().data);
    console.log('a.appended(6.0)', a.appended(6.0).data);
    console.log('a.extended(a)', a.extended(b).data);
    console.log('a.inserted(0, 0.0)', a.inserted(0, 0.0).data);
    console.log('a.spliced(0)', a.spliced(0)[0].data, a.spliced(0)[1]);

    console.groupEnd();
};

const one_dimensional_floats_math = () => {
    console.group('math');

    const a = new Floats1d([1.0, 2.0, 3.0, 4.0, 5.0]);
    const b = new Floats1d([6.0, 7.0, 8.0, 9.0, 10.0]);
    console.log(a.data);
    console.log(b.data);

    const add = a.add(b);
    const sub = a.sub(b);
    const mul = a.mul(b);
    const div = a.div(b);
    console.log('add', add.data);
    console.log('sub', sub.data);
    console.log('mul', mul.data);
    console.log('div', div.data);

    const a_clone = a.clone();
    a_clone.scaled_add(2, b);
    console.log('a.clone().scaled_add(2, b)', a_clone.data);
    console.log('a.sum()', a.sum());
    console.log('a.product()', a.product());
    console.log('a.var(0)', a.var(0));
    console.log('a.std(1)', a.std(1));

    console.groupEnd();
};

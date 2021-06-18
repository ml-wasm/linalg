import { Floats2d } from '../../../pkg/linalg.js';

export const two_dimensional_floats = () => {
    console.group('FLOATS');

    two_dimensional_floats_basics();
    two_dimensional_floats_math();

    console.groupEnd();
};

const two_dimensional_floats_basics = () => {
    console.group('basics');

    const a = new Floats2d([
        [1.0, 2.0, 3.0],
        [4.0, 6.0, 9.0],
    ]);
    const b = new Floats2d([
        [7.0, 8.0, 9.0],
        [10.0, 11.0, 12.0],
    ]);
    console.log(a.data);
    console.log(b.data);

    console.log(a.ncols());
    console.log(a.nrows());
    console.log(a.shape());
    console.log(a.get([0, 1]));
    console.log(a.getR(0).data);
    console.log(a.getC(0).data);
    a.set([0, 1], 20);
    console.log(a.data);
    a.setR(1, b.getR(1));
    console.log(a.data);
    a.setC(2, b.getC(2));
    console.log(a.data);
    a.swap([0, 0], [0, 0]);
    console.log(a.data);
    a.swapR(0, 1);
    console.log(a.data);
    a.swapC(1, 0);
    console.log(a.data);

    console.log(a.appendedR(b.getR(0)).data);
    console.log(a.appendedC(b.getC(1)).data);
    console.log(a.extendedR(b).data)
    console.log(a.extendedC(b).data)
    console.log(a.insertedR(1, b.getR(0)).data);
    console.log(a.insertedC(2, b.getC(0)).data);

    console.log(...a.splicedR(1).map(x => x.data));
    console.log(...a.splicedC(2).map(x => x.data));

    console.groupEnd();
};

const two_dimensional_floats_math = () => {
    console.group('math');

    const a = new Floats2d([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ]);
    const b = new Floats2d([
        [7.0, 8.0, 9.0],
        [10.0, 11.0, 12.0],
    ]);
    console.log(a.data);
    console.log(b.data);

    console.log(a.transposed().data);

    console.log(a.add(b).data);
    console.log(a.sub(b).data);
    console.log(a.mul(b).data);
    console.log(a.div(b).data);
    console.log(b.dot(a.transposed()).data)
    const aClone = a.clone()
    a.clone().scaledAdd(10, b);
    console.log(aClone.data)

    console.log(a.sum());
    console.log(a.sumR().data);
    console.log(a.sumC().data);
    console.log(a.product());

    console.log(a.max());
    console.log(a.maxR().data);
    console.log(a.maxC().data);
    console.log(a.min());
    console.log(a.minR().data);
    console.log(a.minC().data);

    console.log(a.mean());
    console.log(a.meanR().data);
    console.log(a.meanC().data);
    console.log(a.var(0));
    console.log(a.varR(0).data);
    console.log(a.varC(0).data);
    console.log(a.std(1));
    console.log(a.stdR(1).data);
    console.log(a.stdC(1).data);

    console.groupEnd();
};

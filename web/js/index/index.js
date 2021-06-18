import init, { initThreadPool, Integers2d, Strings2d } from '../../../pkg/linalg.js';

// import { bigTest } from './matmul.js';
import { one_dimensional_floats } from './one.js';
import { two_dimensional_floats } from './two.js';

(async () => {
    await init();
    await initThreadPool(navigator.hardwareConcurrency);

    // console.group(
    //     '%cMATRIX MULTPLICATION TEST',
    //     'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
    // );
    // bigTest(100, 1000, 10, 10);
    // console.groupEnd();

    console.group(
        '%cONE DIMENSIONAL',
        'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
    );
    one_dimensional_floats();
    console.groupEnd();

    console.group(
        '%cTWO DIMENSIONAL',
        'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
    );
    two_dimensional_floats();

    console.groupEnd();
})();

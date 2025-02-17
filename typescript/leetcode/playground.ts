type Fn<T> = () => Promise<T>;

function promiseAll<T>(functions: Fn<T>[]): Promise<T[]> {
    const result: Array<T> = [];

    let reason = null;
    for (let fn of functions) {
        fn()
            .then((res) => result.push(res))
            .catch((reason) => (reason = reason));
        if (reason != null) {
            return Promise.reject(reason);
        }
    }

    return Promise.resolve(result);
}

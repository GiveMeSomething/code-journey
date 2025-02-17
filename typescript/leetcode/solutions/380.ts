class RandomizedSet {
    private readonly _values: Map<number, boolean>;
    constructor() {
        this._values = new Map<number, boolean>();
    }

    insert(val: number): boolean {
        if (this._values.has(val)) {
            return false;
        }

        this._values.set(val, true);
        return true;
    }

    remove(val: number): boolean {
        if (!this._values.has(val)) {
            return false;
        }

        this._values.delete(val);
        return true;
    }

    getRandom(): number {
        let target = Math.floor(Math.random() * this._values.size);

        for (let key of this._values.keys()) {
            target--;
            if (target === 0) return key;
        }

        return 0;
    }
}

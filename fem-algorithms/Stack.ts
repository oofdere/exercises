type Node<T> = {
    value: T;
    prev?: Node<T>;
};

export default class Stack<T> {
    public length: number;
    top?: Node<T>;

    constructor() {
        this.length = 0;
    }

    push(item: T): void {
        this.top = {
            value: item,
            prev: this.top,
        };

        this.length++;
    }
    pop(): T | undefined {
        if (!this.top) {
            return undefined;
        }

        const val = this.top?.value;
        this.top = this.top?.prev;
        this.length--;

        return val;
    }
    peek(): T | undefined {
        return this.top?.value;
    }
}

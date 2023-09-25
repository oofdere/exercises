type QNode<T> = {
    value: T;
    next: QNode<T> | undefined;
};

export default class Queue<T> {
    public length: number;
    head: QNode<T> | undefined;
    tail: QNode<T> | undefined;

    constructor() {
        this.length = 0;
        this.head = undefined;
        this.tail = undefined;
    }

    enqueue(item: T): void {
        // create the new node
        const node = {
            value: item,
            next: undefined,
        };

        // if there's already a tail node, make it point to the next node
        if (this.tail !== undefined) {
            this.tail.next = node;
        }

        // if there's no head, make it point to this node
        if (this.head === undefined) {
            this.head = node;
        }

        // move the tail to the new node
        this.tail = node;
        this.length++;

        console.log(this.length, "add", node.value, this.head.value);
    }
    deque(): T | undefined {
        // move the current value into a temp variable
        const head = this.head?.value;

        // point the head to the next item in the queue
        this.head = this.head?.next;
        this.length = Math.max(this.length - 1, 0);

        console.log(this.length, "pop", head, this.head?.value);

        // return the popped value
        return head;
    }
    peek(): T | undefined {
        return this.head?.value;
    }
}

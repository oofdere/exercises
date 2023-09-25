export default function bubble_sort(arr: number[]): void {
    let l = arr.length;

    console.log(arr);

    do {
        for (let i = 0; i < l - 1; i++) {
            if (arr[i] > arr[i + 1]) {
                [arr[i], arr[i + 1]] = [arr[i + 1], arr[i]];
            }
        }

        console.log(l, arr);

        // the last element is now sorted, so exclude from next loop
        l--;
    } while (l > 1);
}

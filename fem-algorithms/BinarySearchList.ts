export default function bs_list(haystack: number[], needle: number): boolean {
    // get index of length/2
    const i = Math.floor(haystack.length / 2);

    if (haystack[i] === needle) {
        return true;
    } else if (haystack[i] > needle) {
        // shallow copy left side
        const j = haystack.slice(0, i - 1);
        return bs_list(j, needle);
    } else if (haystack[i] < needle) {
        // shallow copy right side
        const j = haystack.slice(i + 1);
        return bs_list(j, needle);
    } else {
        return false;
    }
}

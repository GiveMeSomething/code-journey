function merge(nums1: number[], m: number, nums2: number[], n: number): void {
    let i1 = m - 1; // nums1 index
    let i2 = n - 1; // nums2 index

    for (let i = m + n - 1; i >= 0; i--) {
        if (i1 < 0) {
            nums1[i] = nums2[i2];
            i2--;
            continue;
        }

        // Keep adding nums1 item to list
        if (i2 < 0) {
            nums1[i] = nums1[i1];
            i1--;
            continue;
        }

        if (nums2[i2] > nums1[i1]) {
            nums1[i] = nums2[i2];
            i2--;
        } else {
            nums1[i] = nums1[i1];
            nums1[i1] = 0;
            i1--;
        }
    }
}

export const main_88 = () => {
    const nums1 = [1, 2, 3, 0, 0, 0];
    const nums2 = [2, 5, 6];

    merge(nums1, 3, nums2, 3);
    console.log(nums1);
};

function average(salary: number[]): number {
    let min = salary[0];
    let max = 0;
    let sum = 0;
    for (let currentSalary of salary) {
        min = Math.min(min, currentSalary);
        max = Math.max(max, currentSalary);
        sum += currentSalary;
    }

    return (sum - min - max) / (salary.length - 2);
}

export const main_1491 = () => {
    const salary = [4000, 3000, 1000, 2000];

    const result = average(salary);
    console.log(result);
};

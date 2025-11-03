## Demo Linear Regression in predicting function signature

- Target function `y = 100 + x1 + 5 * x2 + 10 * x3`

### Setup
- Generate data using `gen_data.py`
- Train based on train dataset using linear regression
- Test model based on data generated data using `gen_data.py`

### Result

```
# Learning rate: 5 * 10 ** -6
# Stop threshold: 1 * 10 ** -5

295314 [97.45266106  1.01703072  5.07895193 10.12893165]
Result model [97.45262949  1.01703093  5.0789529  10.12893325]
```

```
# Learning rate: 5 * 10 ** -6
# Stop threshold: 1 ** 10 ** -6

388215 [99.19445803  1.0053856   5.02496687 10.0407719 ]
Result model [99.19444805  1.00538567  5.02496718 10.04077241]
```

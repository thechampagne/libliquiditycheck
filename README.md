# libliquiditycheck

[![](https://img.shields.io/github/v/tag/thechampagne/libliquiditycheck?label=version)](https://github.com/thechampagne/libliquiditycheck/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libliquiditycheck)](https://github.com/thechampagne/libliquiditycheck/blob/main/LICENSE)

A **C** library to validate if a string represents a monetary value.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libliquiditycheck.git
```
#### 2. Navigate to the root
```
cd libliquiditycheck
```
#### 3. Build the project
```
cargo build
```

### API

```c
typedef struct {
  int is_err;
  int is_none;
  char* p1;
  char* p2;
} liquidity_check_split_t;

int liquidity_check_validate(const char* input);

liquidity_check_split_t liquidity_check_split(const char* input);

void liquidity_check_string_free(char* ptr);
```

### References
 - [liquiditycheck](https://github.com/b3nj5m1n/liquidity_check)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libliquiditycheck/blob/main/LICENSE).

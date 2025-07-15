# 부동소수점(浮動小數點, floating point) 또는 떠돌이 소수점 방식

- https://ko.wikipedia.org/wiki/%EB%B6%80%EB%8F%99%EC%86%8C%EC%88%98%EC%A0%90
- https://en.wikipedia.org/wiki/Floating-point_arithmetic

<hr />

# 8진수 소수점 표현법
- https://coding-factory.tistory.com/652

<hr />

# 16 진법

$$ x\times 16^2 | x\times 16^1 | x\times 16^0 | x\times 16^{-1}| x\times 16^{-2}| x\times 16^{-3}| $$

- 123.1212 (dec)

|16 * 16^2|16 * 16^1|16 * 16^0|16 * 16^-1|16 * 16^-2|16 * 16^-3|
|-|-|-|-|-|-|
|-|-|-|0.0625|0.00390625|0.00024414|


# 8 진법

$$ x\times 8^2 | x\times 8^1 | x\times 8^0 | x\times 8^{-1}| x\times 8^{-2}| x\times 8^{-3}| $$

- 0o123.123 (octo)

|8 * 8^-1|8 * 8^-2|8 * 8^-3|
|-|-|-|
|0.125|0.0625|0.00195312|


# 2 진법

$$ x\times 2^2 | x\times 2^1 | x\times 2^0 | x\times 2^{-1}| x\times 2^{-2}| x\times 2^{-3}| $$

- 0b123.123 (binary)

|2 * 2^-1|2 * 2^-2|2 * 2^-3|
|-|-|-|
|0.5|0.25|0.125|


# python으로 테스트
- 출처 : https://blog.naver.com/youndok/222304270396

- python3 인터프리터 종료(quit 또는 exit)
- `>>> quit()`
- `>>> exit()`

```bash
$ python3
Python 3.10.12 (main, Feb  4 2025, 14:57:36) [GCC 11.4.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 0b001
1
>>> 0b011
3
>>> 0o88
  File "<stdin>", line 1
    0o88
      ^
SyntaxError: invalid digit '8' in octal literal
>>> 0x0012
18
>>> oct(088)
  File "<stdin>", line 1
    oct(088)
        ^
SyntaxError: leading zeros in decimal integer literals are not permitted; use an 0o prefix for octal integers
>>> a = 88
>>> oct(a)
'0o130'
>>> oct(99)
'0o143'
>>> 0x10
16
>>>
```
<hr />

# ABC108 考察

## A
### 問題
1以上K以下の正数から偶奇のペアは何通りできるか

### 考察

K % 2 != 0の場合,
- 偶数: k - k / 2
- 奇数: k / 2

K % 2 == 0の場合,
- 偶数: k / 2
- 奇数: k / 2

偶数の場合, k - k / 2 == k / 2だから
(k - k / 2) * (k / 2)


## B
### 問題
(x1, y1), (x2, y2)から残りの正方形の座標を求める

### 考察
\vec{x} = x2 - x1, \vec{y} = y2 - y1
残りの２点(x3, y3), (x4, y4)はそれぞれ法線をひいて
(x3, y3) = (x2 - y, y2 - x)
(x4, y4) = (x1 - y, y1 - x)

## C
### 問題
a, b, c <= Nに対し, a + b, b + c, c + a 全てがKの倍数になるようなものの個数

### 制約
1 <= N <= 10^5

### 考察
a = iとするとa + b, a + cがkの倍数となる為にはn, mを用いて  
b = n * k - i, c = m * k - i
このときb + c = (n + m)k - 2i
(b + c) mod k = 2iがKの倍数となるのは

A. K % 2 == 0の場合, k = 2iとなるiが存在するから,
- i % k == 0となるi
- i % k == k / 2となるi

B. K % 2 != の場合, k = 2iとなるiは存在しないから,
- i % k == 0となるi

をそれぞれ数え上げる.


## D
わからん

## 所感
- 整数問題が苦手

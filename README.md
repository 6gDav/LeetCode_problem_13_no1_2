# <a href="https://leetcode.com/problems/roman-to-integer/description/">13. Roman to Integer</a>

## 📝 Description

Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

<table>
  <thead>
    <tr>
      <th>Symbol</th>
      <th>Value</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>I</td>
      <td>1</td>
    </tr>
    <tr>
      <td>V</td>
      <td>5</td>
    </tr>
    <tr>
      <td>X</td>
      <td>10</td>
    </tr>
    <tr>
      <td>L</td>
      <td>50</td>
    </tr>
    <tr>
      <td>C</td>
      <td>100</td>
    </tr>
    <tr>
      <td>D</td>
      <td>500</td>
    </tr>
    <tr>
      <td>M</td>
      <td>1000</td>
    </tr>
  </tbody>
</table>

For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

<ul>
    <li>I can be placed before V (5) and X (10) to make 4 and 9. </li>
    <li>X can be placed before L (50) and C (100) to make 40 and 90. </li>
    <li>C can be placed before D (500) and M (1000) to make 400 and 900.</li>
</ul>

Given a roman numeral, convert it to an integer.

## 🧠 How I solved the problem 

First of all, I implemented a reversed for loop that passes each character of the given string to a switch statement, which gives back the exact value of the Roman numeral. For instance, 'I' is equal to 1.
After this, I use an if statement where I subtract or add the previously processed value according to the position of the previous number.

## ➗ Complexity

* **Time complexity**: *O(n)* - I iterate trought the sting only once.
* **Space complexity**: *O(1)* - No extra data structures are used.

## 📊 Benchmark

I made it in release mode for more accurate results:
```bash
cargo run --release
```

Hardware: *Apple Mac Mini M4*

### 🤏 Small Input Test

* **Execution Time**: *1.292µs*
* **Memory Delta**: *0 bytes*
* **Current Memory**: *1572864 bytes*

### 😖 Stress Test (Large Input)

* **Execution Time**: *6.416µs*
* **Memory Delta**: *16384 bytes*
* **Current Memory**: *1605632 bytes*

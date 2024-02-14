# 한글로 잘 된 이산 수학
- 이산수학은 컴퓨터공학에서 필요한 기초 교과목으로 논리 및 명제, 집합 이론, 관계, 순열 및 조합, 순환 관계, 그래프 및 트리등의 개념을 학습한다. 이산수학의 개념은 향후 알고리즘 설계 및 분석, 데이타베이스 설계, 프로그래밍 원리 등 컴퓨터 전반에 걸쳐 필요한 수리적 토대가 된다.
- http://bigdata.dongguk.ac.kr/lectures/disc_math/_book/
- 1.1 이산수학(Discrete Mathematics)이란?

- 이산수학은 연속적(continuous)이 아닌 불연속(discrete) 객체를 다루는 수학의 한 분야임
  - 예를 들어, 미적분학은 주로 연속적인 대상을 다루며 이산수학에서는 다루지 않음
- discrete objects의 예:
  - 정수,
  - 컴퓨터 프로그램에서 각 단계,
  - 도로망에서 A지점에서 B지점으로 이동하는 서로 다른 경로,
  - 로또복권에서 당첨이 될 경우의 수.
- 이산수학에서는 컴퓨터 과학에서 필요로 하는 수학적 토대를 제공함

- http://bigdata.dongguk.ac.kr/lectures/disc_math/_book/%EC%9D%B4%EC%82%B0%EC%88%98%ED%95%99%EA%B0%9C%EC%9A%94.html
- Complementary Relations (보수 관계)
  - http://bigdata.dongguk.ac.kr/lectures/disc_math/_book/%EA%B4%80%EA%B3%84.html

<hr>

<table>
<colgroup>
<col width="42%">
<col width="31%">
<col width="26%">
</colgroup>
<thead>
<tr class="header">
<th>명칭</th>
<th>별명</th>
<th align="center">논리연산자</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>부정(negation)</td>
<td>NOT</td>
<td align="center"><span class="math inline"><span class="MathJax_Preview" style="color: inherit;"></span><span id="MathJax-Element-17-Frame" class="mjx-chtml MathJax_CHTML" tabindex="0" style="font-size: 117%; position: relative;" data-mathml="<math xmlns=&quot;http://www.w3.org/1998/Math/MathML&quot;><mi mathvariant=&quot;normal&quot;>&amp;#x00AC;</mi></math>" role="presentation"><span id="MJXc-Node-88" class="mjx-math" aria-hidden="true"><span id="MJXc-Node-89" class="mjx-mrow"><span id="MJXc-Node-90" class="mjx-mi"><span class="mjx-char MJXc-TeX-main-R" style="padding-top: 0.056em; padding-bottom: 0.323em;">¬</span></span></span></span><span class="MJX_Assistive_MathML" role="presentation"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi mathvariant="normal">¬</mi></math></span></span><script type="math/tex" id="MathJax-Element-17">\neg</script></span> or ~</td>
</tr>
<tr class="even">
<td>논리곱(conjunction)</td>
<td>AND</td>
<td align="center"><span class="math inline"><span class="MathJax_Preview" style="color: inherit;"></span><span id="MathJax-Element-18-Frame" class="mjx-chtml MathJax_CHTML" tabindex="0" style="font-size: 117%; position: relative;" data-mathml="<math xmlns=&quot;http://www.w3.org/1998/Math/MathML&quot;><mo>&amp;#x2227;</mo></math>" role="presentation"><span id="MJXc-Node-91" class="mjx-math" aria-hidden="true"><span id="MJXc-Node-92" class="mjx-mrow"><span id="MJXc-Node-93" class="mjx-mo"><span class="mjx-char MJXc-TeX-main-R" style="padding-top: 0.323em; padding-bottom: 0.377em;">∧</span></span></span></span><span class="MJX_Assistive_MathML" role="presentation"><math xmlns="http://www.w3.org/1998/Math/MathML"><mo>∧</mo></math></span></span><script type="math/tex" id="MathJax-Element-18">\land</script></span></td>
</tr>
<tr class="odd">
<td>논리합(disjunction)</td>
<td>OR</td>
<td align="center"><span class="math inline"><span class="MathJax_Preview" style="color: inherit;"></span><span id="MathJax-Element-19-Frame" class="mjx-chtml MathJax_CHTML" tabindex="0" style="font-size: 117%; position: relative;" data-mathml="<math xmlns=&quot;http://www.w3.org/1998/Math/MathML&quot;><mo>&amp;#x2228;</mo></math>" role="presentation"><span id="MJXc-Node-94" class="mjx-math" aria-hidden="true"><span id="MJXc-Node-95" class="mjx-mrow"><span id="MJXc-Node-96" class="mjx-mo"><span class="mjx-char MJXc-TeX-main-R" style="padding-top: 0.323em; padding-bottom: 0.377em;">∨</span></span></span></span><span class="MJX_Assistive_MathML" role="presentation"><math xmlns="http://www.w3.org/1998/Math/MathML"><mo>∨</mo></math></span></span><script type="math/tex" id="MathJax-Element-19">\lor</script></span></td>
</tr>
<tr class="even">
<td>배타적 논리합(exclusive OR)</td>
<td>XOR</td>
<td align="center"><span class="math inline"><span class="MathJax_Preview" style="color: inherit;"></span><span id="MathJax-Element-20-Frame" class="mjx-chtml MathJax_CHTML" tabindex="0" style="font-size: 117%; position: relative;" data-mathml="<math xmlns=&quot;http://www.w3.org/1998/Math/MathML&quot;><mo>&amp;#x2295;</mo></math>" role="presentation"><span id="MJXc-Node-97" class="mjx-math" aria-hidden="true"><span id="MJXc-Node-98" class="mjx-mrow"><span id="MJXc-Node-99" class="mjx-mo"><span class="mjx-char MJXc-TeX-main-R" style="padding-top: 0.323em; padding-bottom: 0.43em;">⊕</span></span></span></span><span class="MJX_Assistive_MathML" role="presentation"><math xmlns="http://www.w3.org/1998/Math/MathML"><mo>⊕</mo></math></span></span><script type="math/tex" id="MathJax-Element-20">\oplus</script></span></td>
</tr>
<tr class="odd">
<td>함축(implication), 조건(conditional)</td>
<td>IMPLY</td>
<td align="center"><span class="math inline"><span class="MathJax_Preview" style="color: inherit;"></span><span id="MathJax-Element-21-Frame" class="mjx-chtml MathJax_CHTML" tabindex="0" style="font-size: 117%; position: relative;" data-mathml="<math xmlns=&quot;http://www.w3.org/1998/Math/MathML&quot;><mo stretchy=&quot;false&quot;>&amp;#x2192;</mo></math>" role="presentation"><span id="MJXc-Node-100" class="mjx-math" aria-hidden="true"><span id="MJXc-Node-101" class="mjx-mrow"><span id="MJXc-Node-102" class="mjx-mo"><span class="mjx-char MJXc-TeX-main-R" style="padding-top: 0.216em; padding-bottom: 0.377em;">→</span></span></span></span><span class="MJX_Assistive_MathML" role="presentation"><math xmlns="http://www.w3.org/1998/Math/MathML"><mo stretchy="false">→</mo></math></span></span><script type="math/tex" id="MathJax-Element-21">\to</script></span></td>
</tr>
<tr class="even">
<td>상호조건(biconditional)</td>
<td>IFF(if and only if)</td>
<td align="center"><span class="math inline"><span class="MathJax_Preview" style="color: inherit;"></span><span id="MathJax-Element-22-Frame" class="mjx-chtml MathJax_CHTML" tabindex="0" style="font-size: 117%; position: relative;" data-mathml="<math xmlns=&quot;http://www.w3.org/1998/Math/MathML&quot;><mo stretchy=&quot;false&quot;>&amp;#x2194;</mo></math>" role="presentation"><span id="MJXc-Node-103" class="mjx-math" aria-hidden="true"><span id="MJXc-Node-104" class="mjx-mrow"><span id="MJXc-Node-105" class="mjx-mo"><span class="mjx-char MJXc-TeX-main-R" style="padding-top: 0.216em; padding-bottom: 0.377em;">↔</span></span></span></span><span class="MJX_Assistive_MathML" role="presentation"><math xmlns="http://www.w3.org/1998/Math/MathML"><mo stretchy="false">↔</mo></math></span></span><script type="math/tex" id="MathJax-Element-22">\leftrightarrow</script></span></td>
</tr>
</tbody>
</table>

<hr>

# Discrete Mathematics Final Review Part 1: Structures (Fall 2022) | A Yang

2022년 12월 7일
CS 2800 Final Exam Review Session
Ambrose Yang, Cornell University

Part 1: Propositional logic, sets, functions, relations, automata

1:30 Propositional and predicate logic<br>
19:15 Set theory<br>
26:35 Functions<br>
36:50 Cardinality of sets<br>
49:25 Relations<br>
1:09:18 Finite automata<br>

- https://youtu.be/B4-cJUoEwjA?si=jiheKg2V-vyupaaH

- part2
 - Discrete Mathematics Final Review Part 2: Combinatorics and Probability (Fall 2022) | A Yang
   - https://youtu.be/pO2F9BagUvA?si=ICT5zPfEwiYqZ0Ks

2022년 12월 8일
CS 2800 Final Exam Review Session
Ambrose Yang, Cornell University

Part 2: Combinations, Stirling numbers of the second kind, occupancy problems, combinatorial proofs, Principle of Inclusion-Exclusion, conditional probability, Bayes' Rule, random variables
<br>

0:30 Combinatorial formulas<br>
10:40 Distinguishable and indistinguishable balls and bins<br>
34:10 Combinatorial proofs<br>
41:50 Principle of Inclusion-Exclusion<br>
45:25 Probability basics: outcomes, sample spaces, events, conditional probability<br>
52:05 Bayes' Rule<br>
58:45 Random variables<br>


<hr>

- 모아보기 
  - https://youtube.com/playlist?list=PL7Wri0mncI36dTjRfvuFikrA0iMOGQrzV&si=44xpDVNsnpoKxMGO

<hr>

# Discrete Mathematics (Full Course) | My Lesson

- https://youtu.be/UwYJUKVc-Hs?si=9MMFRcxAN99pVbXF

<hr>

# Combinatorics and Probability (Complete Course) | Discrete Mathematics for Computer Science | My Lesson

- https://youtu.be/0GIwDazlUHs?si=JFis7W5T_jbCZqAh

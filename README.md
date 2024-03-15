# Algorithm_Training

<p align="center">
    <img align="center" alt="algorithm" width="180px" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/0dbe2ef1-78c3-458b-9893-4abaed1d8080">
</p>


<hr>

# 유료강의
- fastcampus(현실 세상의 컴퓨터공학 지식 with 30가지 실무 시나리오 초격차 패키지 Online
  - https://fastcampus.co.kr/dev_online_newcomputer

<hr>

# 사람의 눈보다 더 빨리 움직이는 걸 볼 수 있다면? | 우리 눈이 볼 수 없는 세계 (1) 초고속의 순간 #BBC | 세상의 모든 다큐

- https://youtu.be/VCvS4MV0hEM?si=AYIByJl4ySYC2efv

  - part2
    -  https://youtu.be/PWEVqrhTLro?si=TcDFRGpMOcb_uVOj

  - part3
    - https://youtu.be/jURoQtKgjqY?si=y7TF2kRO0I5uJh0P
<hr>
 
# 프로그래머가 알아야 할 지연 시간 숫자를 시각적으로 표현  
**[GN⁺: 모든 프로그래머가 알아야 할 필수 숫자들](<https://news.hada.io/topic?id=13749&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
  - https://samwho.dev/numbers/?fo
- **L1 캐시 참조**: 1나노초  
- **분기 예측 실패**: 3나노초  
- **L2 캐시 참조**: 4나노초  
- **뮤텍스 잠금/해제**: 17나노초  
- **1 Gbps 네트워크를 통한 1KB 데이터 전송**: 44나노초  
- **주 메모리 참조**: 100나노초  
- **Zippy를 이용한 1KB 데이터 압축**: 2마이크로초
- **메모리에서 1MB 순차 읽기**: 3마이크로초
- **SSD에서 4K 무작위 읽기**: 16마이크로초
- **SSD에서 1MB 순차 읽기**: 49마이크로초
- **동일 데이터센터 내 왕복 시간**: 500마이크로초
- **디스크에서 1MB 순차 읽기**: 825마이크로초
- **디스크 탐색**: 2밀리초
- **캘리포니아에서 네덜란드까지 패킷 전송 후 돌아오기**: 150밀리초

```
Latency Comparison Numbers (~2012)
----------------------------------
L1 cache reference                           0.5 ns
Branch mispredict                            5   ns
L2 cache reference                           7   ns                      14x L1 cache
Mutex lock/unlock                           25   ns
Main memory reference                      100   ns                      20x L2 cache, 200x L1 cache
Compress 1K bytes with Zippy             3,000   ns        3 us
Send 1K bytes over 1 Gbps network       10,000   ns       10 us
Read 4K randomly from SSD*             150,000   ns      150 us          ~1GB/sec SSD
Read 1 MB sequentially from memory     250,000   ns      250 us
Round trip within same datacenter      500,000   ns      500 us
Read 1 MB sequentially from SSD*     1,000,000   ns    1,000 us    1 ms  ~1GB/sec SSD, 4X memory
Disk seek                           10,000,000   ns   10,000 us   10 ms  20x datacenter roundtrip
Read 1 MB sequentially from disk    20,000,000   ns   20,000 us   20 ms  80x memory, 20X SSD
Send packet CA->Netherlands->CA    150,000,000   ns  150,000 us  150 ms

Notes
-----
1 ns = 10^-9 seconds
1 us = 10^-6 seconds = 1,000 ns
1 ms = 10^-3 seconds = 1,000 us = 1,000,000 ns

Credit
------
By Jeff Dean:               http://research.google.com/people/jeff/
Originally by Peter Norvig: http://norvig.com/21-days.html#answers

Contributions
-------------
'Humanized' comparison:  https://gist.github.com/hellerbarde/2843375
Visual comparison chart: http://i.imgur.com/k0t1e.png
```
- https://gist.github.com/jboner/2841832

<img src="https://i.imgur.com/k0t1e.png" />



<hr>

- https://norvig.com/21-days.html#answers

<a name="answers"><h2>Answers</h2></a>

Approximate timing for various operations on a typical PC:<p>
<table border=1 cellpadding=2 cellspacing=2>
<tr><td>execute typical instruction<td align=right> 1/1,000,000,000 sec = 1 nanosec
<tr><td>fetch from L1 cache memory<td align=right> 0.5 nanosec
<tr><td>branch misprediction<td align=right> 5 nanosec
<tr><td>fetch from L2 cache memory<td align=right> 7 nanosec
<tr><td>Mutex lock/unlock<td align=right> 25 nanosec
<tr><td>fetch from main memory<td align=right> 100 nanosec 
<tr><td>send 2K bytes over 1Gbps network<td align=right> 20,000 nanosec
<tr><td>read 1MB sequentially from memory<td align=right> 250,000 nanosec
<tr><td>fetch from new disk location (seek)<td align=right> 8,000,000 nanosec
<tr><td>read 1MB sequentially from disk<td align=right> 20,000,000 nanosec
<tr><td>send packet US to Europe and back<td align=right> 150 milliseconds = 150,000,000 nanosec
    
</table>

- https://ko.m.wikipedia.org/wiki/%EB%B0%80%EB%A6%AC

<table class="toccolours"><caption><b><a href="/wiki/SI_%EC%A0%91%EB%91%90%EC%96%B4" title="SI 접두어">SI 접두어</a></b><br><span style="font-size:small;"><span class="noprint plainlinks plainlinksneverexpand" style="white-space:nowrap; font-weight:normal; font-size:smaller; ;"><a href="/wiki/%ED%8B%80:SI_%EC%A0%91%EB%91%90%EC%96%B4" title="틀:SI 접두어"><span title="이 틀을 보기" style=";">v</span></a> <span style="font-size:smaller;">•</span> <a href="/w/index.php?title=%ED%8B%80%ED%86%A0%EB%A1%A0:SI_%EC%A0%91%EB%91%90%EC%96%B4&amp;action=edit&amp;redlink=1" class="new" title="틀토론:SI 접두어 (없는 문서)"><span title="이 틀에 대한 토론" style=";">d</span></a> <span style="font-size:smaller;">•</span> <a class="external text" href="https://ko.wikipedia.org/w/index.php?title=%ED%8B%80:SI_%EC%A0%91%EB%91%90%EC%96%B4&amp;action=edit"><span title="이 틀을 편집할 수 있습니다. 단, 저장하기 전에 미리 보기로 결과를 확인하여 주십시오." style=";">e</span></a> <span style="font-size:smaller;">•</span> <a class="external text" href="https://ko.wikipedia.org/w/index.php?title=%ED%8B%80:SI_%EC%A0%91%EB%91%90%EC%96%B4&amp;action=history"><span title="이 틀의 역사를 볼 수 있습니다." style=";">h</span></a></span></span>
</caption>
<tbody><tr style="background:#ccccff"><th>10<sup>n</sup></th>
<th>접두어
</th>
<th>기호
</th>
<th>배수
</th>
<th>십진수
</th></tr><tr><td>10<sup>30</sup></td>
<td><a href="/wiki/%ED%80%98%ED%83%80_(SI_%EC%A0%91%EB%91%90%EC%96%B4)" title="퀘타 (SI 접두어)">퀘타</a> (quetta)
</td>
<td>Q
</td>
<td>백<a href="/wiki/10000000000000000000000000000" class="mw-redirect" title="10000000000000000000000000000">양</a>
</td>
<td>1 000 000 000 000 000 000 000 000 000 000
</td></tr><tr><td>10<sup>27</sup></td>
<td><a href="/wiki/%EB%A1%A0%EB%82%98" title="론나">론나</a> (ronna)
</td>
<td>R
</td>
<td>천<a href="/wiki/1000000000000000000000000" class="mw-redirect" title="1000000000000000000000000">자</a>
</td>
<td>1 000 000 000 000 000 000 000 000 000
</td></tr><tr><td>10<sup>24</sup></td>
<td><a href="/wiki/%EC%9A%94%ED%83%80" title="요타">요타</a> (yotta)
</td>
<td>Y
</td>
<td>일<a href="/wiki/1000000000000000000000000" class="mw-redirect" title="1000000000000000000000000">자</a>
</td>
<td>1 000 000 000 000 000 000 000 000
</td></tr><tr><td>10<sup>21</sup></td>
<td><a href="/wiki/%EC%A0%9C%ED%83%80" title="제타">제타</a> (zetta)
</td>
<td>Z
</td>
<td>십<a href="/wiki/100000000000000000000" class="mw-redirect" title="100000000000000000000">해</a>
</td>
<td>1 000 000 000 000 000 000 000
</td></tr><tr><td>10<sup>18</sup></td>
<td><a href="/wiki/%EC%97%91%EC%82%AC" title="엑사">엑사</a> (exa)
</td>
<td>E
</td>
<td>백<a href="/wiki/10000000000000000" title="10000000000000000">경</a>
</td>
<td>1 000 000 000 000 000 000
</td></tr><tr><td>10<sup>15</sup></td>
<td><a href="/wiki/%ED%8E%98%ED%83%80_(SI_%EC%A0%91%EB%91%90%EC%96%B4)" title="페타 (SI 접두어)">페타</a> (peta)
</td>
<td>P
</td>
<td>천<a href="/wiki/1000000000000" title="1000000000000">조</a>
</td>
<td>1 000 000 000 000 000
</td></tr><tr><td>10<sup>12</sup></td>
<td><a href="/wiki/%ED%85%8C%EB%9D%BC_(SI_%EC%A0%91%EB%91%90%EC%96%B4)" title="테라 (SI 접두어)">테라</a> (tera)
</td>
<td>T
</td>
<td>일<a href="/wiki/1000000000000" title="1000000000000">조</a>
</td>
<td>1 000 000 000 000
</td></tr><tr><td>10<sup>9</sup></td>
<td><a href="/wiki/%EA%B8%B0%EA%B0%80" title="기가">기가</a> (giga)
</td>
<td>G
</td>
<td><a href="/wiki/1000000000" title="1000000000">십억</a>
</td>
<td>1 000 000 000
</td></tr><tr><td>10<sup>6</sup></td>
<td><a href="/wiki/%EB%A9%94%EA%B0%80" title="메가">메가</a> (mega)
</td>
<td>M
</td>
<td><a href="/wiki/1000000" title="1000000">백만</a>
</td>
<td>1 000 000
</td></tr><tr><td>10<sup>3</sup></td>
<td><a href="/wiki/%ED%82%AC%EB%A1%9C" title="킬로">킬로</a> (kilo)
</td>
<td>k
</td>
<td><a href="/wiki/1000" title="1000">천</a>
</td>
<td>1 000
</td></tr><tr><td>10<sup>2</sup></td>
<td><a href="/wiki/%ED%97%A5%ED%86%A0" title="헥토">헥토</a> (hecto)
</td>
<td>h
</td>
<td><a href="/wiki/100" title="100">백</a>
</td>
<td>100
</td></tr><tr><td>10<sup>1</sup></td>
<td><a href="/wiki/%EB%8D%B0%EC%B9%B4" title="데카">데카</a> (deca)
</td>
<td>da
</td>
<td><a href="/wiki/10" title="10">십</a>
</td>
<td>10
</td></tr><tr><td>10<sup>0</sup></td>
<td>
</td>
<td>
</td>
<td><a href="/wiki/1" title="1">일</a>
</td>
<td>1
</td></tr><tr><td>10<sup>−1</sup></td>
<td><a href="/wiki/%EB%8D%B0%EC%8B%9C" title="데시">데시</a> (deci)
</td>
<td>d
</td>
<td>십분의 일
</td>
<td>0.1
</td></tr><tr><td>10<sup>−2</sup></td>
<td><a href="/wiki/%EC%84%BC%ED%8B%B0" title="센티">센티</a> (centi)
</td>
<td>c
</td>
<td>백분의 일
</td>
<td>0.01
</td></tr><tr><td>10<sup>−3</sup></td>
<td><a class="mw-selflink selflink">밀리</a> (milli)
</td>
<td>m
</td>
<td>천분의 일
</td>
<td>0.001
</td></tr><tr><td>10<sup>−6</sup></td>
<td><a href="/wiki/%EB%A7%88%EC%9D%B4%ED%81%AC%EB%A1%9C" title="마이크로">마이크로</a> (micro)
</td>
<td>µ
</td>
<td>백만분의 일
</td>
<td>0.000 001
</td></tr><tr><td>10<sup>−9</sup></td>
<td><a href="/wiki/%EB%82%98%EB%85%B8" title="나노">나노</a> (nano)
</td>
<td>n
</td>
<td>십억분의 일
</td>
<td>0.000 000 001
</td></tr><tr><td>10<sup>−12</sup></td>
<td><a href="/wiki/%ED%94%BC%EC%BD%94" title="피코">피코</a> (pico)
</td>
<td>p
</td>
<td>일조분의 일
</td>
<td>0.000 000 000 001
</td></tr><tr><td>10<sup>−15</sup></td>
<td><a href="/wiki/%ED%8E%A8%ED%86%A0" title="펨토">펨토</a> (femto)
</td>
<td>f
</td>
<td>천조분의 일
</td>
<td>0.000 000 000 000 001
</td></tr><tr><td>10<sup>−18</sup></td>
<td><a href="/wiki/%EC%95%84%ED%86%A0" title="아토">아토</a> (atto)
</td>
<td>a
</td>
<td>백경분의 일
</td>
<td>0.000 000 000 000 000 001
</td></tr><tr><td>10<sup>−21</sup></td>
<td><a href="/wiki/%EC%A0%AD%ED%86%A0" title="젭토">젭토</a> (zepto)
</td>
<td>z
</td>
<td>십해분의 일
</td>
<td>0.000 000 000 000 000 000 001
</td></tr><tr><td>10<sup>−24</sup></td>
<td><a href="/wiki/%EC%9A%95%ED%86%A0" title="욕토">욕토</a> (yocto)
</td>
<td>y
</td>
<td>일자분의 일
</td>
<td>0.000 000 000 000 000 000 000 001
</td></tr><tr><td>10<sup>−27</sup></td>
<td><a href="/wiki/%EB%A1%A0%ED%86%A0" title="론토">론토</a> (ronto)
</td>
<td>r
</td>
<td>천자분의 일
</td>
<td>0.000 000 000 000 000 000 000 000 001
</td></tr><tr><td>10<sup>−30</sup></td>
<td><a href="/wiki/%ED%80%99%ED%86%A0" title="퀙토">퀙토</a> (quecto)
</td>
<td>q
</td>
<td>백양분의 일
</td>
<td>0.000 000 000 000 000 000 000 000 000 001
</td></tr></tbody></table>


# 트위터 추천 알고리즘(scala로 작성됨)

- https://github.com/twitter/the-algorithm

<hr>

# 애니매이션으로 모든 물리학 공식과 같이 연관 되어 보기.. 진짜 대박 최고 !!❤
- Animation vs. Physics | Alan Becker
  - https://youtu.be/ErMSHiQRnc8?si=mG-sttkOox6CS7Oq
    - 한글 버젼 애니메이션 vs 물리학 한글 자막
      - https://youtu.be/qYJbrCQovzE?si=pBsAExRd1E3sVXBO
물리학과
구독자 458명
- Animation vs. Math | Alan Becker
  - https://youtu.be/B1J6Ou4q8vE?si=53zJzMx2_-mTXdbS



<hr>


<br>

# 자료 구조

https://github.com/YoungHaKim7/c_project/tree/main/exercise/002stack

- 영어 출처
  https://en.wikipedia.org/wiki/Association_list

<table border="1">
    <tr>
    <td colspan="2" align="center">자료 구조(Well-known data structures)</td>
    </tr>
    <tr align="center">
        <td>유형(Type) </td>
        <td> 컬렉션(Collection) , 컨테이너(Container)</td>
    </tr>
    <tr align="center">
        <td> 추상ADT<br>Abstract Data Type </td>
        <td> 연관 배열(Associative array), 우선 순위 덱(Priority Deque), 덱(Deque), 리스트(List),<br> 멀티맵, 우선순위 큐(Priority Queue), 큐(Queue), <br>집합 (멀티셋, 분리 집합),<br> 스택(stack) <br>
Associative array(Multimap, Retrieval Data Structure), List, StackQueue(Double-ended queue), Priority queue(Double-ended priority queue), Set(Multiset, Disjoint-set)
    </td>
    </tr>
    <tr align="center">
        <td>배열(Array) </td>
        <td> 
비트 배열(Bit Array), 환형 배열(Circular array), 동적 배열(Dynamic Array),<br> 해시 테이블(Hash Table), 해시드 어레이 트리(Hashed Array Tree), 희소 배열(Sparse array)
        </td>
    </tr>
    <tr align="center">
        <td>연결형(Linked) </td>
        <td> 연관 리스트(Association list), 
        <br>
        <br>연결 리스트(Linked List) - 단일연결(Singly Linked List), 이중연결(Doubly Linked List), 원형 연결(Circular Linked List)<br><br>Association list,<br> Linked list, Skip list, Unrolled linked list, XOR linked list</td>
    </tr>
    <tr align="center">
        <td>트리(Trees) </td>
        <td>B 트리,<br> 이진 탐색 트리(AA, AVL, 레드-블랙, 자가 균형, splay) <br> 힙(이진 힙, 피보나치) ,<br> R 트리( R*, R+, 힐버트),<br> 트리(해시 트리)<br><br>B-tree, Binary search tree(AA tree, AVL tree, Red–black tree, Self-balancing tree, Splay tree),<br> Heap(Binary heap, Binomial heap, Fibonacci heap),<br> R-tree(R* tree, R+ tree, Hilbert R-tree), Trie Hash tree
    </td>
    </tr>
    <tr align="center">
        <td>그래프(Graphs) </td>
        <td>이진 결정 다이어그램<br>Binary decision diagram, Directed acyclic graph, Directed acyclic word graph </td>
    </tr>
</table>

<br>

<hr>

# 대표적인 알고리즘 정리

1. 정렬(Sort)
2. 검색(Search)
3. 문자열 패턴 매칭(SPM: String Pattern Matching)
4. 계산(Calculation)

<hr>

# 알고리즘 정렬 이미지

<img src="https://t1.daumcdn.net/cfile/tistory/27123F41549E2B2D2B?original" />

<hr>

# StructuredProgramming
- 알고리즘을 기술할 목적으로 만들어진 언어 중 하나가 SPARKS(Structured Programming: A Reasonably Kimplete Set)
- 구성
```
- 선언문
- 지정문
- 조건문
- 반복문
- Procedure문
- 프로시져 사이 자료 전달 방법(process transaction)
- 입출력문
- 주석문
```

- https://en.wikipedia.org/wiki/Flow-based_programming

- https://web.archive.org/web/20180308114549/http://www.hit.ac.il:80/staff/leonidM/information-systems/ch63.html

- https://en.wikipedia.org/wiki/Structured_programming

- http://users.csc.calpoly.edu/~jdalbey/103/Lectures/StructuredProgramming/

<hr>

# Amazon관련책 Algorithms 4th Edition, Kindle Edition
https://www.amazon.com/Algorithms-Algorithms_4-Robert-Sedgewick-ebook/dp/B004P8J1NA/ref=tmm_kin_swatch_0?_encoding=UTF8&qid=&sr=

# Algorithms & Algorithms design and analysis
- part 1 complete(12hr)
  - https://youtu.be/9diDWV-fOnE?si=r1YlAfOcIz0w5z_i
- Algorithms part 2 (1/2)
  - https://youtu.be/0qF7tPSQdCg?si=YuLJeuBTfgz_RWM9
- Algorithms part 2 (2/2)
  - https://youtu.be/6TW3JSVEJQE?si=sPBO7A70DYQcbhz9
    - Algorithms design and analysis part 1(1/2)
      - https://youtu.be/NqKkxQamroo?si=P5qxYCsWJ2hZEOqA
    - Algorithms design and analysis part 1(2/2)
      - https://youtu.be/ahvrc4tZbTE?si=RRO0j71_Pskv24rn
    - Algorithms design and analysis part 2(1/2)
      - https://youtu.be/nUIbHblMop4?si=2UHh66W__YNTWpKd
    - Algorithms design and analysis part 2(2/2)
      - https://youtu.be/M5JzG8oDicA?si=8bEI94JHkkcOAUA7

<hr>

# Big Picture of Calculus | MIT OpenCourseWare

https://youtu.be/UcWsDwg1XwM


# 파이썬으로 알고리즘 구조 이해하기

- 파이썬으로 알고리즘 공부하기 https://academy.cs.cmu.edu/

# 수학 그래프를 그리는 도구

- Demos & GeoGebra
  - https://www.desmos.com/
   
  - GeoGebra 소개 영상 https://youtu.be/X_YT335w8Mc
    - https://www.geogebra.org/

- 비 전문가들이 고품질의 아름다운 다이어그램을 작성할 수 있게 도와주는 것을 목표로 개발
  - 세가지 프로그램을 이용하여 구성
    - Domain (.domain) 프로그램 : 해당 도메인의 다이어그램을 구성하는 객체, Predicates, 함수의 유형을 설명
    - Substance (.substance) 프로그램 : 다이어그램의 객체와 관계를 정의
    - Style (.style) 프로그램 : 객체와 관계를 디스플레이하는 방법을 지시
  - https://penrose.cs.cmu.edu/examples

- 러스트로 만든 LaTex업그레이드 버젼 A new markup-based typesetting system that is powerful and easy to learn.
  - https://github.com/typst/typst

- ALL of calculus 3 in 8 minutes.수학공식 전부 다 그래프로 그려보기 https://youtu.be/5kwz7ajxfyA

# 알고리즘 관련

- Rust https://github.com/TheAlgorithms/Rust
- 알고리즘 (1^8) 1초안에 승부를 보는 알고리즘의 세계 https://news.hada.io/topic?id=9459
  - 1억 = 1초 https://zoosso.tistory.com/883
  - MIT강의 MIT 6.006 Introduction to Algorithms, Spring 2020 https://youtube.com/playlist?list=PLUl4u3cNGP63EdVPNLG3ToM6LaEUuStEY
    - 1. Algorithms and Computation | MIT ♡Algorithm♡ https://youtu.be/ZA-tUyM_y7s
      2. Data Structures and Dynamic Arrays | MIT https://youtu.be/CHhwJjR0mZA
  - 서울대학교  SNUON_컴퓨터과학이 여는 세계_9.2 알고리즘의 예_이광근 https://youtu.be/39sJYroBZLs
  - 그림으로 공부❤️8Data Structures That Power Database https://youtu.be/W_v05d_2RTo

<hr>

# Quantum Programming 

- Quantum Programming Part 1(설명 굿👍)
  - https://youtu.be/2Eswqed8agg

# Visualization of Quantum Physics (Quantum Mechanics)

- 2분 21초 https://youtu.be/p7bzE1E5PMY
- 슈뢰딩거 방정식(영어: Schrödinger equation) 그림으로 이해하기
- 마크다운 수학 공식 정리 https://rayc20.tistory.com/151
- TeX_및_LaTeX_수식_문법 http://tomoyo.ivyro.net/123/wiki.php/TeX_%EB%B0%8F_LaTeX_%EC%88%98%EC%8B%9D_%EB%AC%B8%EB%B2%95
- **Schrödinger equation**
```math
$$i \hbar \frac{\partial}{\partial t}\Psi(\mathbf{r},t) = \hat H \Psi(\mathbf{r},t)$$
```

- 동영상에 나오는 공식
```math
$$i \hbar \frac{\partial}{\partial t}\Psi = -{\hbar^2 \over2m}\Delta\Psi$$
```

https://www.siue.edu/~mnorton/quantum.pdf

![Screenshot 2023-08-13 at 6 56 22 PM](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/8228b29d-09a3-4e75-80c6-4fc7ff12d869)

- 자바스크립트로 구현한 슈뢰딩거 방정식(-方程式, 영어: Schrödinger equation)
  - https://github.com/rreusser/schrodinger-equation-1d-demo

  - 슈뢰딩거 방정식(-方程式, 영어: Schrödinger equation) 정의
 
    - https://ko.wikipedia.org/wiki/%EC%8A%88%EB%A2%B0%EB%94%A9%EA%B1%B0_%EB%B0%A9%EC%A0%95%EC%8B%9D

<hr>

# 마크 다운에 수학 공식 넣는 방법

https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/writing-mathematical-expressions


# 수학 공식 테스트 하기(live web - latex)

https://www.mathjax.org/

**The Cauchy-Schwarz Inequality**

```math
\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)
```


<hr>

<hr>

# 무작위 속 질서, 중심극한정리 | 확률과 통계 | 3Blue1Brown 한국어

https://youtu.be/SoKjCUcDBf0?si=wcx9mPCZf_MYG9_9



# Algorithm_Training

<p align="center">
    <img align="center" alt="algorithm" width="180px" src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/0dbe2ef1-78c3-458b-9893-4abaed1d8080">
</p>

# link

- [latex-문법](#latex-문법)
  - [수학-공식-테스트-하기live-web---latex](#수학-공식-테스트-하기live-web---latex)
  - [마크-다운에-수학-공식-넣는-방법](#마크-다운에-수학-공식-넣는-방법)

- [사람의-눈보다-더-빨리-움직이는-걸-볼-수-있다면--우리-눈이-볼-수-없는-세계-1-초고속의-순간-bbc--세상의-모든-다큐](#사람의-눈보다-더-빨리-움직이는-걸-볼-수-있다면--우리-눈이-볼-수-없는-세계-1-초고속의-순간-bbc--세상의-모든-다큐)
  - [지그-창시자가-설명해-주는-operation-cost-in-cpu-cycles--andrew-kelley-practical-data-oriented-design-dod](#지그-창시자가-설명해-주는-operation-cost-in-cpu-cycles--andrew-kelley-practical-data-oriented-design-dod) 
  - [프로그래머가-알아야-할-지연-시간-숫자를-시각적으로-표현](#프로그래머가-알아야-할-지연-시간-숫자를-시각적으로-표현)
  - [무작위-속-질서-중심극한정리--확률과-통계--3blue1brown-한국어](#무작위-속-질서-중심극한정리--확률과-통계--3blue1brown-한국어)
  - [애니매이션으로-모든-물리학-공식과-같이-연관-되어-보기-진짜-대박-최고](#애니매이션으로-모든-물리학-공식과-같이-연관-되어-보기-진짜-대박-최고-)

- [수학-그래프를-그리는-도구](#수학-그래프를-그리는-도구)

- [알고리즘-관련](#알고리즘-관련)
  - [트위터-추천-알고리즘scala로-작성됨](#트위터-추천-알고리즘scala로-작성됨)
  - [자료-구조Data Structure](#자료-구조)
    - [서울대 Data Structure자료 굿](https://datalab.snu.ac.kr/~ukang/courses/24S-DS/)
  - [대표적인-알고리즘-정리](#대표적인-알고리즘-정리)
  - [알고리즘-정렬-이미지](#알고리즘-정렬-이미지)
  - [structuredprogramming](#structuredprogramming)
  - [파이썬으로-알고리즘-구조-이해하기](#파이썬으로-알고리즘-구조-이해하기)

- [big-picture-of-calculus--mit-opencourseware](#big-picture-of-calculus--mit-opencourseware)

- [amazon관련책-algorithms-4th-edition-kindle-edition](#amazon관련책-algorithms-4th-edition-kindle-edition)

- [algorithms--algorithms-design-and-analysis](#algorithms--algorithms-design-and-analysis)

<hr>

- [quantum-programming](#quantum-programming)
  - [visualization-of-quantum-physics-quantum-mechanics](#visualization-of-quantum-physics-quantum-mechanics)
  - [양자 코딩 여기에 정리  중...](https://github.com/YoungHaKim7/silq_project)

<hr>

# 유료강의[[🔝]](#link)
- fastcampus(현실 세상의 컴퓨터공학 지식 with 30가지 실무 시나리오 초격차 패키지 Online
  - https://fastcampus.co.kr/dev_online_newcomputer
    - (맛보기무료)컴퓨터 구조 몰아보기(7hr)
      - https://youtu.be/kFWP6sFKyp0?si=IXbGZgn6jp_RRkg_

<hr>

# Latex 문법[[🔝]](#link)

- https://junia3.github.io/blog/latex_symbols

<hr>

# 사람의 눈보다 더 빨리 움직이는 걸 볼 수 있다면? | 우리 눈이 볼 수 없는 세계 (1) 초고속의 순간 #BBC | 세상의 모든 다큐[[🔝]](#link)

- https://youtu.be/VCvS4MV0hEM?si=AYIByJl4ySYC2efv

  - part2
    -  https://youtu.be/PWEVqrhTLro?si=TcDFRGpMOcb_uVOj

  - part3
    - https://youtu.be/jURoQtKgjqY?si=y7TF2kRO0I5uJh0P
<hr>
 
# 프로그래머가 알아야 할 지연 시간 숫자를 시각적으로 표현[[🔝]](#link)
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

- 이게 고화질 https://colin-scott.github.io/personal_website/research/interactive_latency.html

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

<hr>

|Operation|ns|µs|ms|note|
|-|-|-|-|-|
|L1 cache reference|0.5 ns||||
|Branch mispredict|5 ns||||
|L2 cache reference|7 ns|||14x L1 cache|
|Mutex lock/unlock|25 ns||||
|Main memory reference|100 ns|||20x L2 cache, 200x L1 cache|
|Compress 1K bytes with Zippy|3,000 ns|3 µs|||
|Send 1K bytes over 1 Gbps network|10,000 ns|10 µs|||
|Read 4K randomly from SSD*|150,000 ns|150 µs||~1GB/sec SSD|
|Read 1 MB sequentially from memory|250,000 ns|250 µs|||
|Round trip within same datacenter|500,000 ns|500 µs|||
|Read 1 MB sequentially from SSD*|1,000,000 ns|1,000 µs|1 ms|~1GB/sec SSD, 4X memory|
|Disk seek|10,000,000 ns|10,000 µs|10 ms|20x datacenter roundtrip|
|Read 1 MB sequentially from disk|20,000,000 ns|20,000 µs|20 ms|80x memory, 20X SSD|
|Send packet CA -> Netherlands -> CA|150,000,000 ns|150,000 µs|150 ms||


# 지그 창시자가 설명해 주는 Operation Cost in CPU Cycles & Andrew Kelley Practical Data Oriented Design (DoD)[[🔝]](#link)
![Screenshot 2024-07-19 at 9 24 35 PM](https://github.com/user-attachments/assets/e863900c-6bf1-4a83-8585-0150d514c5f3)

- Andrew Kelley Practical Data Oriented Design (DoD) | ChimiChanga(5min50sec)
  - https://youtu.be/IroPQ150F6c?si=tOxqzFtk5hkuWwYt 

<hr>

# 시대별로 단위가 생긴거 표로 잘 정리됨(Mertic_prefix_pico_kilo_nano..etc.[[🔝]](#link)

- https://en.wikipedia.org/wiki/Metric_prefix

<table class="wikitable" style="padding: 0; text-align: center; width: 0">
<caption><style data-mw-deduplicate="TemplateStyles:r1045256916">.mw-parser-output .navbar-header{text-align:center;position:relative;white-space:nowrap}.mw-parser-output .navbar-header .navbar{position:absolute;right:0;top:0;margin:0 5px}</style><div class="navbar-header"><a class="mw-selflink selflink">SI prefixes</a><style data-mw-deduplicate="TemplateStyles:r1129693374">.mw-parser-output .hlist dl,.mw-parser-output .hlist ol,.mw-parser-output .hlist ul{margin:0;padding:0}.mw-parser-output .hlist dd,.mw-parser-output .hlist dt,.mw-parser-output .hlist li{margin:0;display:inline}.mw-parser-output .hlist.inline,.mw-parser-output .hlist.inline dl,.mw-parser-output .hlist.inline ol,.mw-parser-output .hlist.inline ul,.mw-parser-output .hlist dl dl,.mw-parser-output .hlist dl ol,.mw-parser-output .hlist dl ul,.mw-parser-output .hlist ol dl,.mw-parser-output .hlist ol ol,.mw-parser-output .hlist ol ul,.mw-parser-output .hlist ul dl,.mw-parser-output .hlist ul ol,.mw-parser-output .hlist ul ul{display:inline}.mw-parser-output .hlist .mw-empty-li{display:none}.mw-parser-output .hlist dt::after{content:": "}.mw-parser-output .hlist dd::after,.mw-parser-output .hlist li::after{content:" · ";font-weight:bold}.mw-parser-output .hlist dd:last-child::after,.mw-parser-output .hlist dt:last-child::after,.mw-parser-output .hlist li:last-child::after{content:none}.mw-parser-output .hlist dd dd:first-child::before,.mw-parser-output .hlist dd dt:first-child::before,.mw-parser-output .hlist dd li:first-child::before,.mw-parser-output .hlist dt dd:first-child::before,.mw-parser-output .hlist dt dt:first-child::before,.mw-parser-output .hlist dt li:first-child::before,.mw-parser-output .hlist li dd:first-child::before,.mw-parser-output .hlist li dt:first-child::before,.mw-parser-output .hlist li li:first-child::before{content:" (";font-weight:normal}.mw-parser-output .hlist dd dd:last-child::after,.mw-parser-output .hlist dd dt:last-child::after,.mw-parser-output .hlist dd li:last-child::after,.mw-parser-output .hlist dt dd:last-child::after,.mw-parser-output .hlist dt dt:last-child::after,.mw-parser-output .hlist dt li:last-child::after,.mw-parser-output .hlist li dd:last-child::after,.mw-parser-output .hlist li dt:last-child::after,.mw-parser-output .hlist li li:last-child::after{content:")";font-weight:normal}.mw-parser-output .hlist ol{counter-reset:listitem}.mw-parser-output .hlist ol>li{counter-increment:listitem}.mw-parser-output .hlist ol>li::before{content:" "counter(listitem)"\a0 "}.mw-parser-output .hlist dd ol>li:first-child::before,.mw-parser-output .hlist dt ol>li:first-child::before,.mw-parser-output .hlist li ol>li:first-child::before{content:" ("counter(listitem)"\a0 "}</style><style data-mw-deduplicate="TemplateStyles:r1236085633">.mw-parser-output .navbar{display:inline;font-size:88%;font-weight:normal}.mw-parser-output .navbar-collapse{float:left;text-align:left}.mw-parser-output .navbar-boxtext{word-spacing:0}.mw-parser-output .navbar ul{display:inline-block;white-space:nowrap;line-height:inherit}.mw-parser-output .navbar-brackets::before{margin-right:-0.125em;content:"[ "}.mw-parser-output .navbar-brackets::after{margin-left:-0.125em;content:" ]"}.mw-parser-output .navbar li{word-spacing:-0.125em}.mw-parser-output .navbar a>span,.mw-parser-output .navbar a>abbr{text-decoration:inherit}.mw-parser-output .navbar-mini abbr{font-variant:small-caps;border-bottom:none;text-decoration:none;cursor:inherit}.mw-parser-output .navbar-ct-full{font-size:114%;margin:0 7em}.mw-parser-output .navbar-ct-mini{font-size:114%;margin:0 4em}html.skin-theme-clientpref-night .mw-parser-output .navbar li a abbr{color:var(--color-base)!important}@media(prefers-color-scheme:dark){html.skin-theme-clientpref-os .mw-parser-output .navbar li a abbr{color:var(--color-base)!important}}@media print{.mw-parser-output .navbar{display:none!important}}</style><div class="navbar plainlinks hlist navbar-mini"><ul><li class="nv-view"><a href="/wiki/Template:SI_prefixes_(infobox)" title="Template:SI prefixes (infobox)"><abbr title="View this template">v</abbr></a></li><li class="nv-talk"><a href="/wiki/Template_talk:SI_prefixes_(infobox)" title="Template talk:SI prefixes (infobox)"><abbr title="Discuss this template">t</abbr></a></li><li class="nv-edit"><a href="/wiki/Special:EditPage/Template:SI_prefixes_(infobox)" title="Special:EditPage/Template:SI prefixes (infobox)"><abbr title="Edit this template">e</abbr></a></li></ul></div></div>
</caption>
<tbody><tr>
<th colspan="2">Prefix</th>
<th rowspan="2">Base 10
</th>
<th rowspan="2"><a href="/wiki/Decimal" title="Decimal">Decimal</a>
</th>
<th rowspan="2">Adoption<br /><sup id="cite_ref-3" class="reference"><a href="#cite_note-3">&#91;nb 1&#93;</a></sup>
</th></tr>
<tr>
<th>Name</th>
<th>Symbol
</th></tr>
<tr>
<td>quetta</td>
<td>Q</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#1030" title="Orders of magnitude (numbers)">10<sup>30</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span></td>
<td rowspan="2">2022<sup id="cite_ref-newUnitAdoption_4-0" class="reference"><a href="#cite_note-newUnitAdoption-4">&#91;3&#93;</a></sup>
</td></tr>
<tr>
<td>ronna</td>
<td>R</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#1027" title="Orders of magnitude (numbers)">10<sup>27</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span>
</td></tr>
<tr>
<td>yotta</td>
<td>Y</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#1024" title="Orders of magnitude (numbers)">10<sup>24</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span></td>
<td rowspan="2">1991
</td></tr>
<tr>
<td>zetta</td>
<td>Z</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#1021" title="Orders of magnitude (numbers)">10<sup>21</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span>
</td></tr>
<tr>
<td>exa</td>
<td>E</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#1018" title="Orders of magnitude (numbers)">10<sup>18</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span></td>
<td rowspan="2">1975<sup id="cite_ref-5" class="reference"><a href="#cite_note-5">&#91;4&#93;</a></sup>
</td></tr>
<tr>
<td>peta</td>
<td>P</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#1015" title="Orders of magnitude (numbers)">10<sup>15</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span>
</td></tr>
<tr>
<td>tera</td>
<td>T</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#1012" title="Orders of magnitude (numbers)">10<sup>12</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span></td>
<td rowspan="2">1960
</td></tr>
<tr>
<td><a href="/wiki/Giga-" title="Giga-">giga</a></td>
<td>G</td>
<td style="text-align:left;"><a href="/wiki/1,000,000,000" title="1,000,000,000">10<sup>9</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span>
</td></tr>
<tr>
<td><a href="/wiki/Mega-" title="Mega-">mega</a></td>
<td>M</td>
<td style="text-align:left;"><a href="/wiki/1,000,000" title="1,000,000">10<sup>6</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span></span></td>
<td>1873
</td></tr>
<tr>
<td><a href="/wiki/Kilo-" title="Kilo-">kilo</a></td>
<td>k</td>
<td style="text-align:left;"><a href="/wiki/1000_(number)" title="1000 (number)">10<sup>3</sup></a>
</td>
<td style="text-align:right; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">1<span style="margin-left:0.25em">000</span></span></td>
<td rowspan="3">1795
</td></tr>
<tr>
<td><a href="/wiki/Hecto-" title="Hecto-">hecto</a></td>
<td>h</td>
<td style="text-align:left;"><a href="/wiki/100" title="100">10<sup>2</sup></a></td>
<td style="text-align:right; font-variant-numeric: tabular-nums;">100
</td></tr>
<tr>
<td><a href="/wiki/Deca-" title="Deca-">deca</a></td>
<td>da</td>
<td style="text-align:left;"><a href="/wiki/10" title="10">10<sup>1</sup></a></td>
<td style="text-align:right; font-variant-numeric: tabular-nums;">10
</td></tr>
<tr style="background:#EAECF0;color:black;">
<td>—</td>
<td>—</td>
<td style="text-align:left;"><a href="/wiki/1" title="1">10<sup>0</sup></a></td>
<td style="font-variant-numeric: tabular-nums;">1</td>
<td>—
</td></tr>
<tr>
<td><a href="/wiki/Deci-" title="Deci-">deci</a></td>
<td>d</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10-1" title="Orders of magnitude (numbers)">10<sup>−1</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;">0.1</td>
<td rowspan="3">1795
</td></tr>
<tr>
<td><a href="/wiki/Centi-" title="Centi-">centi</a></td>
<td>c</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−2" title="Orders of magnitude (numbers)">10<sup>−2</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;">0.01
</td></tr>
<tr>
<td><a href="/wiki/Milli-" title="Milli-">milli</a></td>
<td>m</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−3" title="Orders of magnitude (numbers)">10<sup>−3</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;">0.001
</td></tr>
<tr>
<td><a href="/wiki/Micro-" title="Micro-">micro</a></td>
<td>μ</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−6" title="Orders of magnitude (numbers)">10<sup>−6</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">001</span></span></td>
<td>1873
</td></tr>
<tr>
<td><a href="/wiki/Nano-" title="Nano-">nano</a></td>
<td>n</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−9" title="Orders of magnitude (numbers)">10<sup>−9</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">001</span></span></td>
<td rowspan="2">1960
</td></tr>
<tr>
<td>pico</td>
<td>p</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−12" title="Orders of magnitude (numbers)">10<sup>−12</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">001</span></span>
</td></tr>
<tr>
<td>femto</td>
<td>f</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−15" title="Orders of magnitude (numbers)">10<sup>−15</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">001</span></span></td>
<td rowspan="2">1964
</td></tr>
<tr>
<td>atto</td>
<td>a</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−18" title="Orders of magnitude (numbers)">10<sup>−18</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">001</span></span>
</td></tr>
<tr>
<td>zepto</td>
<td>z</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−21" title="Orders of magnitude (numbers)">10<sup>−21</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">001</span></span></td>
<td rowspan="2">1991
</td></tr>
<tr>
<td>yocto</td>
<td>y</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−24" title="Orders of magnitude (numbers)">10<sup>−24</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">001</span></span>
</td></tr>
<tr>
<td>ronto</td>
<td>r</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−27" title="Orders of magnitude (numbers)">10<sup>−27</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">001</span></span></td>
<td rowspan="2">2022<sup id="cite_ref-newUnitAdoption_4-1" class="reference"><a href="#cite_note-newUnitAdoption-4">&#91;3&#93;</a></sup>
</td></tr>
<tr>
<td>quecto</td>
<td>q</td>
<td style="text-align:left;"><a href="/wiki/Orders_of_magnitude_(numbers)#10−30" title="Orders of magnitude (numbers)">10<sup>−30</sup></a>
</td>
<td style="text-align:left; font-variant-numeric: tabular-nums;"><span style="white-space:nowrap">0.000<span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">000</span><span style="margin-left:0.25em">001</span></span>
</td></tr>
<tr>
<td style="text-align: left;" colspan="5">
<dl><dt>Notes</dt></dl>
<style data-mw-deduplicate="TemplateStyles:r1217336898">.mw-parser-output .reflist{font-size:90%;margin-bottom:0.5em;list-style-type:decimal}.mw-parser-output .reflist .references{font-size:100%;margin-bottom:0;list-style-type:inherit}.mw-parser-output .reflist-columns-2{column-width:30em}.mw-parser-output .reflist-columns-3{column-width:25em}.mw-parser-output .reflist-columns{margin-top:0.3em}.mw-parser-output .reflist-columns ol{margin-top:0}.mw-parser-output .reflist-columns li{page-break-inside:avoid;break-inside:avoid-column}.mw-parser-output .reflist-upper-alpha{list-style-type:upper-alpha}.mw-parser-output .reflist-upper-roman{list-style-type:upper-roman}.mw-parser-output .reflist-lower-alpha{list-style-type:lower-alpha}.mw-parser-output .reflist-lower-greek{list-style-type:lower-greek}.mw-parser-output .reflist-lower-roman{list-style-type:lower-roman}</style><div class="reflist">
<div class="mw-references-wrap"><ol class="references">
<li id="cite_note-3"><span class="mw-cite-backlink"><b><a href="#cite_ref-3">^</a></b></span> <span class="reference-text">Prefixes adopted before 1960 already existed before SI. The introduction of the <a href="/wiki/Centimetre%E2%80%93gram%E2%80%93second_system_of_units" title="Centimetre–gram–second system of units">CGS system</a> was in 1873.</span>
</li>
</ol></div></div>
</td></tr></tbody></table>

- https://en.wikipedia.org/wiki/Metric_prefix

<hr>


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


# 트위터 추천 알고리즘(scala로 작성됨)[[🔝]](#link)

- https://github.com/twitter/the-algorithm

<hr>

# 애니매이션으로 모든 물리학 공식과 같이 연관 되어 보기.. 진짜 대박 최고 !!❤[[🔝]](#link)
- Animation vs. Physics | Alan Becker
  - https://youtu.be/ErMSHiQRnc8?si=mG-sttkOox6CS7Oq
    - 한글 버젼 애니메이션 vs 물리학 한글 자막 | 물리학과
      - https://youtu.be/qYJbrCQovzE?si=pBsAExRd1E3sVXBO
- Animation vs. Math | Alan Becker
  - https://youtu.be/B1J6Ou4q8vE?si=53zJzMx2_-mTXdbS



<hr>


<br>

# 자료 구조[[🔝]](#link)

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

# 대표적인 알고리즘 정리[[🔝]](#link)

1. 정렬(Sort)
2. 검색(Search)
3. 문자열 패턴 매칭(SPM: String Pattern Matching)
4. 계산(Calculation)

<hr>

# 알고리즘 정렬 이미지[[🔝]](#link)

<img src="https://t1.daumcdn.net/cfile/tistory/27123F41549E2B2D2B?original" />

- Random(Shell & Heap . 가 젤 빠름)
- Nearly Sorted(Insert 가 젤 빠름)
- Reverse(Shell 가 젤 빠름)
- Few Unique(Quick3 가 젤 빠름) 

<hr>

# StructuredProgramming[[🔝]](#link)
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

# Big Picture of Calculus | MIT OpenCourseWare[[🔝]](#link)

https://youtu.be/UcWsDwg1XwM


# 파이썬으로 알고리즘 구조 이해하기[[🔝]](#link)

- 파이썬으로 알고리즘 공부하기 https://academy.cs.cmu.edu/

# 수학 그래프를 그리는 도구[[🔝]](#link)

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

# Quantum Programming[[🔝]](#link)

- Quantum Programming Part 1(설명 굿👍)
  - https://youtu.be/2Eswqed8agg

# Visualization of Quantum Physics (Quantum Mechanics)[[🔝]](#link)

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

# 마크 다운에 수학 공식 넣는 방법[[🔝]](#link)

https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/writing-mathematical-expressions


# 수학 공식 테스트 하기(live web - latex)[[🔝]](#link)

https://www.mathjax.org/

**The Cauchy-Schwarz Inequality**

```math
\left( \sum_{k=1}^n a_k b_k \right)^2 \leq \left( \sum_{k=1}^n a_k^2 \right) \left( \sum_{k=1}^n b_k^2 \right)
```


<hr>

<hr>

# 무작위 속 질서, 중심극한정리 | 확률과 통계 | 3Blue1Brown 한국어

https://youtu.be/SoKjCUcDBf0?si=wcx9mPCZf_MYG9_9



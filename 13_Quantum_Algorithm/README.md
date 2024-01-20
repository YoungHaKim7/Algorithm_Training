# 양자 코딩 정리 

https://github.com/YoungHaKim7/silq_project

<hr>

# 빛은 입자인가 파동인가? 양자역학 1편! (KAIST 김갑진 교수의 물리학 특강 6/8) | 안될과학 Unrealscience
- https://youtu.be/RglES21LdxE?si=JW6gKYca97QIZvM6

- 입자의 스핀과 불확정성의 원리란? 양자역학 2편! (KAIST 김갑진 교수의 물리학 특강 7/8) | 안될과학 Unrealscience
  - https://youtu.be/As2tGiGwjl4?si=KyJxHliSSOQioqja

- 양자컴퓨터란 무엇인가? 양자역학 3편! (KAIST 김갑진 교수의 물리학 특강 8/8) | 안될과학 Unrealscience
  - https://youtu.be/gjp9301in0U?si=3RDEvy26mChsOa0P

- 초끈이론 이해하기(끈이론) 타우, 쿼크, 뮤온, 초대칭 이론
  - https://economiceco.tistory.com/m/19084
<hr>

# 서의린 (KAIST) / Introduction to vertex algebras V / 2016-09-03 | Mathnet Korea

- https://youtu.be/NwGnAYyGVsY?si=9xdDL-iXZK_Q0a1o

- 알기 쉽게 해설해드림. 배경설명이 길수도 있음.

<p> 리뷰부터 보면
wightman axiom(와이트먼 공리)

와이트먼이라는 수학 겸 물리학자가 있는데 이 사람이 1980년도에

quantum field theory(양자장론)

을 수학적으로 기술하기 위해서 여러 수학적 공리(axiom)들을 정해놈.

(참고) 공리란 주어진 이론체계에서 증명없이 참으로 받아들이는 명제.
논리,수학 이론 체계에서 가장 기초가 되는 규칙. 예) a=b 이면 a+c = b+c 이다. 같은 규칙.

양자장론이란 간단하게 말하면 장(필드)이란 것은 공간상에 각 지점마다 물리량을 표현하는 것인데
(예를 들자면 대표적으로 자기장이 있음. 자석이 존재하면 그 공간에 자기력이라는 힘을 가하는데 그 공간 지점마다 자기력이라는 물리량이 생김)
양자장론에서는 원자보다 작은 입자(쿼크, 전자 등)와 준입자(광자, 글루온 등의 보손, 입자가 아니지만 입자성을 띄는 객체) 같은 녀석들이
양자화된 장에서 어떻게 수학적으로 물리량으로서 표현되고 기술되는지에 대한 이야기임.
- 양자화된 장이란 걸 이해하기 위해선 양자화, 양자역학 개념을 좀 알아야 하는데 먼저 양자화란 연속적인 자연의 모든 것들을 디지털화 하는 것임.
예를 들어서 성장하는 사람의 키를 측정한다고 해보자. 100cm~160cm까지 크는 동안 155.58372647....... cm 인 상태가 분명 존재할 것임. 사람의 키는 연속적으로 크니까. 하지만 우리는 저걸 측정하는 것도 불가능하고 기록도 저렇게 안함. 155.5 155.6 이렇게 딱딱 끊어서 기록함. 이걸 바로 사람의 키를 양자화 했다고 할 수 있음.

 중고등학교때 배운 보어의 원자모형처럼 전자가 원자 주변에 연속적으로 퍼져있지 않고 특정 궤도에서만 존재한다. 특정 에너지 준위를 가진다....라고 배운 적 있을 것임. 수소원자의 전자가 n= 몇 궤도에서 에너지 준위가 떨어지면서 빛을 발사하는데 복사스펙트럼이 한 줄로 띄엄띄엄 존재하는 것을 배운 적 있을 것임. 고전역학에서는 여기에 존재한다고 특정지어서 이야기 하는 것임.

양자역학에서는 불확정성의 원리로 존재의 위치를 특정할 수 없음.
따라서 전자의 위치에 대한 확률로 파동함수를 나타내 전자의 에너지 준위를 표현하는 것이 양자역학적 표현방법임.

즉 양자역학은 어떤 입자나 준입자의 존재성, 에너지 등을 확률론과 파동함수를 이용해서 수학적으로 표현하는 방법이라는 것임.

양자역학을 이해하기 힘든 이유는 이게 수학적 '표현'방법이라 그런거임. 절대적 진리도 아니고 자연계처럼 연속적이지도 않기 때문에 직관적으로 이해하기 힘듬.

그럼 양자장론, 양자화된 장이란 공간의 어떤 물리량, 에너지를 양자역학적으로 표현된 공간이라는 뜻이고 

와이트먼은 이런 양자장에 대한 수학적 표현방법을 규칙으로 정리했는데 그게 바로 와이트먼 공리이며 

여기서 파생된 것이 맨 처음에 나온 vertex algebra(꼭지점 연산자 대수)라는 것임.

여기서 연산자라는 것은 장이론에서 특정 연속적인, 고전역학적인 어떤 측정값을 연산자를 통해 변환, 치환해서  양자화 된 데이터를 뽑을 수 있는데 이게 중요한 이유는 양자화를 하면 연산자의 고윳값, 고유벡터를 알 수 있게 됨.

이제 vertex algebra를 보면
가장 간단한 경우. 공간이 1차원이다라는 말은 수직선이란 말이고 시간이 1차원이란 말은 연속적으로 흐르는 시간이란 말임.

즉 수직선위의 어떤 임의의 점이 시간에 따라 어떤 물리량을 갖는지 보는 연산자임.
</p>

- 이 연산자가 가지고 있는 물리량 즉 공간의 측정된 물리량을 이 연산자로 해석했을 때 나오는 4가지 물리량을 파악할 수 있게 라는 연산자임.
1. 벡터 스페이스
2. 베큠 벡터
3. translation(End V)
4. 컨텀필드

<p>
이 4가지 정보를 가진 연산자 VA로 입자과 준입자의 들뜸상태를 해석해버리겠다~는 내용이 대략 영상길이 4분정도 내용인데...

더이상 해설을 하기엔 딮한 부븐들이 너무 많아서 포기..

사실 이걸 이해할려면 수학적으로도.. 물리학적으로도... 학부수준 이상의 수년에 달하는 기초지식이 필요함.

</p>

<hr>

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

# MicroSoft Q# 내가 정리한 silq와 문법과 코딩이 다르다.

https://learn.microsoft.com/en-us/azure/quantum/

# 내가 공부하려고 만든 영상

한글양자코딩slq001*양자코딩*윈도우에서 리눅스 가상환경 설치 후 양자코딩준비하기 #quantum #slq #silq

https://youtu.be/klzS-ekfq0s

<br>

# Silq's language

https://silq.ethz.ch/documentation

<br>

# Quantum slq file compile

https://github.com/eth-sri/silq

<br>

# VSCode Insert Unicode(Extensions)

https://marketplace.visualstudio.com/items?itemName=brunnerh.insert-unicode

<br>

# Unicode Search

http://xahlee.info/comp/unicode_index.html?q=%E2%84%95

<br>

# SILQ Unicode

<table border="1">
    <tr>
    <td colspan="4" align="center">양자 코딩  자주 쓰는 Unicode</td>
    </tr>
    <tr align="center">
        <td>Symbol</td>
        <td>Unicode(Hex)</td>
        <td>LaTeX ShortCut</td>
        <td>Name</td>
    </tr>
    <tr align="center">
        <td>→</td>
        <td>2192</td>
        <td>\to</td>
        <td>Rightwards Arrow</td>
    </td>
    </tr>
    <tr align="center">
        <td>ℕ</td>
        <td>2115</td>
        <td>\bn</td>
        <td>Double-Struck Capital N</td>
    </td>
    </tr>
    <tr align="center">
        <td>ℝ</td>
        <td>211D</td>
        <td>\br</td>
        <td>Double-Struck Capital R</td>
    </td>
    </tr>
    <tr align="center">
        <td>π</td>
        <td>3A0</td>
        <td>\pi</td>
        <td>Greek Capital Letter Pi</td>
    </td>
    </tr>
    <tr align="center">
        <td>𝔹</td>
        <td>1D539</td>
        <td>\bb</td>
        <td>Mathematical Double-Struck Capital B</td>
    </td>
    </tr>
    <tr align="center">
        <td>⋅</td>
        <td>22C5</td>
        <td>\cdot</td>
        <td>Dot Operator</td>
    </td>
    </tr>
    <tr align="center">
        <td>θ</td>
        <td>398</td>
        <td>\theta</td>
        <td>Greek Capital Letter Theta</td>
    </td>
    </tr>
    <tr align="center">
        <td>ψ</td>
        <td>3A8</td>
        <td>\psi</td>
        <td>Greek Capital Letter PSI</td>
    </td>
    </tr>
    <tr align="center">
        <td>λ</td>
        <td>39B</td>
        <td>\lambda</td>
        <td>Greek Capital Letter LAMDA</td>
    </td>
    </tr>
    <tr align="center">
        <td>¬</td>
        <td>AC</td>
        <td>\neg</td>
        <td>Not Sign</td>
    </td>
    </tr>
    <tr align="center">
        <td>×</td>
        <td>D7</td>
        <td>\times</td>
        <td>Multiplication Sign</td>
    </td>
    </tr>
    <tr align="center">
        <td>±</td>
        <td>B1</td>
        <td>\pm</td>
        <td>Plus-Minus Sign</td>
    </td>
    </tr>
    <tr align="center">
        <td>𝟙</td>
        <td>1D7D9</td>
        <td>\b1</td>
        <td>Mathematical Double-Struck Digit One</td>
    </td>
    </tr>
</table>

<br>

# silq*project_Emacs * unicode-input

https://agda.readthedocs.io/en/latest/tools/emacs-mode.html#unicode-input

<br>

# 35 Quantum Computing Software Tools

https://thequantuminsider.com/2022/05/27/quantum-computing-tools/

<br>

# QuantumComputingReport/ Tools

https://quantumcomputingreport.com/tools/

<br>

# 빛과 썬글라스로 양자역학 이해하기\_실제test하면서ㅎㅎ❤️Bell's Theorem: The Quantum Venn Diagram Paradox

- Bell's Theorem: The Quantum Venn Diagram Paradox

  https://youtu.be/zcqZHYo7ONs

<br>

- 이건 오리지날 영상
  Some light quantum mechanics (with minutephysics)
  https://youtu.be/MzRCDLre1b4

<br>

<hr>

이 책에 silq 짠 코드가 친절하게 나온게 이거 먼저 구매 강추!!

# Quantum Computing with Silq Programming: Get up and running with quantum computing with the simplicity of this new high-level programming language 1st Edition, Kindle Edition

https://www.amazon.com/dp/B091D34X6K/ref=cm_sw_r_awdo_DY25YGP5GXR5C71QKK6E

<br>

# Essential Mathematics for Quantum Computing: A beginner's guide to just the math you need without needless complexities 1st Edition, Kindle Edition

https://www.amazon.com/Essential-Mathematics-Quantum-Computing-complexities-ebook/dp/B09TRQPYRS/ref=d_pd_sim_sccl_1_1/143-0855274-0933658?pd_rd_w=cB006&content-id=amzn1.sym.9125e5ab-ea95-44ef-9958-112d5f0f26f0&pf_rd_p=9125e5ab-ea95-44ef-9958-112d5f0f26f0&pf_rd_r=5M2BMV7J6P49HFMFC7P2&pd_rd_wg=LEpIy&pd_rd_r=8e71abf5-1664-4c8c-a82b-733113ceef0f&pd_rd_i=B09TRQPYRS&psc=1

<table border="1">
    <tr>
    <td colspan="2" align="center">Quantum Computing</td>
    </tr>
    <tr align="center">
        <td>Quantum<br>Mechanics</td>
        <td>Computer<br>Science</td>
    </tr>
    <tr>
    <td colspan="2" align="center">Math</td>
    </tr>
</table>

<br>
<br>

# Programming Quantum Computers: Essential Algorithms and Code Samples 1st Edition, Kindle Edition

https://www.amazon.com/Programming-Quantum-Computers-Essential-Algorithms-ebook/dp/B07TWTC739/ref=sr_1_5?crid=1EJCPO7C8HDZA&keywords=quantum+computers&qid=1665844313&qu=eyJxc2MiOiI0LjU1IiwicXNhIjoiMy43MSIsInFzcCI6IjMuNDMifQ%3D%3D&s=digital-text&sprefix=quantum+computers%2Cdigital-text%2C249&sr=1-5

<br>

<br>

<hr>

# 자석과 스핀트로닉스 1~3화, KAIST 김갑진 교수 | 안될과학

- 자석의 원리 아셨습니까? N극, S극의 근원은? 자석의 자기장 어떻게 나올까? [자석과 스핀트로닉스 1/3화, KAIST 김갑진 교수]
  - https://youtu.be/FU29W6B1eeE?si=4aa0BN9_HRpbP8Ot
  - 양자역학 탄생에 '스핀'이 있었다! '스핀'은 무엇이고 어떻게 등장했을까? [자석과 스핀트로닉스 2/3화, KAIST 김갑진 교수]
    - https://youtu.be/1tsAW1f-heE?si=M7jtW_8l0LrChKvx
  - 자석 이야기로 '상대성 이론'을 이해시켜드립니다. '상대성 이론'과 N극과 S극을 당기는 이유 [자석과 스핀트로닉스 3/3화, KAIST 김갑진 교수]
    - https://youtu.be/lFIIoFb40TE?si=0Yv4ikVqNnN6Lne9
  - 가장 강력한 자석을 만드는 과학적 방법?! [돌아온 자석과 스핀트로닉스 1/2화, KAIST 김갑진 교수]
    - https://youtu.be/2Gfv3kxoGfg?si=XZA_wfuONxv11mEt
  - 과연 스핀을 가진 전자의 흐름을 정의할 수 있을까?! [돌아온 자석과 스핀트로닉스 2/2화, KAIST 김갑진 교수]
    - https://youtu.be/xEEUVxnVw7c?si=MTsTXKS3Ipl8qiX7
  - 1차원과 2차원, 경계 기준을 어떻게 발견했을까?! 김갑진 교수의 스핀트로닉스 연구와 비하인드 스토리! [자석과 스핀트로닉스 3편 2/2화, KAIST 김갑진 교수]
    - https://youtu.be/JJHK-WYmeTU?si=gTuelJygjJpbQSaF


<hr>

# 회전 대칭은 스핀을 이해 할 수 있다.
<img src="https://particleadventure.pusan.ac.kr/images/page-elements/fermion_boson.jpg" />

- 보존(Bosons)이 스핀이 1 
  - 한바퀴 돌았을때 제자리 오면 스핀 1

- 페르미온(Fermions) 스핀이 2
  - 2바퀴 돌아서 제자리 오면 
    - 스핀 2 -> 스핀 1/2 이라고 부름

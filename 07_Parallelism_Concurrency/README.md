# What is the difference between concurrency and parallelism?

- https://stackoverflow.com/questions/1050222/what-is-the-difference-between-concurrency-and-parallelism
```
Concurrency                 Concurrency + parallelism
(Single-Core CPU)           (Multi-Core CPU)
 ___                         ___ ___
|th1|                       |th1|th2|
|   |                       |   |___|
|___|___                    |   |___
    |th2|                   |___|th2|
 ___|___|                    ___|___|
|th1|                       |th1|
|___|___                    |   |___
    |th2|                   |   |th2|
```
- Concurrency: If two or more problems are solved by a single processor.
<img src="https://i.stack.imgur.com/OdYWr.gif" />

<hr>

- Parallelism: If one problem is solved by multiple processors.
<img src="https://i.stack.imgur.com/RRF1J.gif" />
<hr>


# 동기화 상식 & process synchronization : 임계구역 (critical section) & 경쟁상태 (race condition)

- https://velog.io/@lovi0714/%EC%9E%84%EA%B3%84-%EC%98%81%EC%97%AD-Critical-Section
- OS27 - Synchronization | Race Condition | Critical Section | Producer-Consumer Problem | EZCSE
  - https://youtu.be/Pvkl0HD62yc?si=_Vrl9B_YvG6iOP4f

<hr>

- 공유 자원(Shared Resource)
  - 공동의 자원(e.g.  파일, 전역 변수, 입출력장치, ..)
- 임계 구역(Critical Section)
  - 동시에 접근하면 문제가 발생할 수 있는 공유 자원에 접근하는 코드

- 교착 상태(Dead Lock)
- 경쟁상태 (race condition)
  - https://en.wikipedia.org/wiki/Race_condition
  - https://yoongrammer.tistory.com/57
  - What is a Race Condition (Computer Programming)? | Eye on Tech
    - https://youtu.be/KF8dF1QS8Go?si=UFvrJ0hxJEef8Xzo

<hr>

- 해결책 Race condition / Critical Section / 하드웨어 동기화 / 동기화 기법 / Busy Waiting / 세마포어에서 Busy Waiting 문제 해결
  - https://m.blog.naver.com/PostView.naver?isHttpsRedirect=true&blogId=07911015&logNo=110186350959

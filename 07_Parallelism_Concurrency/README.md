# What is the difference between concurrency and parallelism?

- https://stackoverflow.com/questions/1050222/what-is-the-difference-between-concurrency-and-parallelism

<hr>


# 동기화 상식

https://velog.io/@lovi0714/%EC%9E%84%EA%B3%84-%EC%98%81%EC%97%AD-Critical-Section

- 공유 자원(Shared Resource)
  - 공동의 자원(e.g.  파일, 전역 변수, 입출력장치, ..)
- 임계 구역(Critical Section)
  - 동시에 접근하면 문제가 발생할 수 있는 공유 자원에 접근하는 코드

- 교착 상태(Dead Lock)
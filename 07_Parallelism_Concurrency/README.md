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

- 혼자 공부하는 컴퓨터 구조
  - https://velog.io/@mmodestaa/%ED%98%BC%EC%9E%90-%EA%B3%B5%EB%B6%80%ED%95%98%EB%8A%94-%EC%BB%B4%ED%93%A8%ED%84%B0-%EA%B5%AC%EC%A1%B0-%EC%9A%B4%EC%98%81%EC%B2%B4%EC%A0%9C-Section-12.-%ED%94%84%EB%A1%9C%EC%84%B8%EC%8A%A4-%EB%8F%99%EA%B8%B0%ED%99%94

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

- 1. 경쟁상태 (race condition)
<img src="https://junhyunny.github.io/images/race-condition-1.JPG" />
- https://junhyunny.github.io/information/operating-system/junit/race-condition/

<hr>

- 2. Solve the problem of Critical SectionPermalink
  - 2.1.1. Mutual exclusionPermalink
<img src="https://junhyunny.github.io/images/race-condition-2.JPG" />

출처 : https://junhyunny.github.io/information/operating-system/junit/race-condition/

<hr>

# 해결책 Race condition / Critical Section / 하드웨어 동기화 / 동기화 기법 / Busy Waiting / 세마포어에서 Busy Waiting 문제 해결
- https://m.blog.naver.com/PostView.naver?isHttpsRedirect=true&blogId=07911015&logNo=110186350959
- 상호 배제를 위한 동기화를 위한 세 가지 원칙
- https://lealea.tistory.com/243
  - 1. 상호 배제(mutual exclusion)
    - 한 프로세가 임계 구역에 진입했다면 다른 프로세스는 들어올 수 없다.
  - 2. 진행(Progress)
    - 임계 구역에 어떤 프로세서도 진입하지 않았다면 진입하고자 하는 프로세스는 들어갈 수 있어야 한다.
  - 3. 유한 대기(bounded waiting)
    - 한 프로세스가 임계 구역에 진입하고 싶다면 <I><strong><em>언젠가는 임계 구역에 들어올 수 있어야 한다.</em></strong></I>
 
 <hr>
 
 - 동기화 기법 중 대표적인 3가지
   - 뮤텍스 락(Mutex lock)
     - c++개념 mutex https://en.cppreference.com/w/cpp/thread/mutex
   - 세마포(semaphore)
     - https://en.cppreference.com/w/cpp/thread/counting_semaphore
   - 모니터
  
  <hr>

- 세마포(semaphore)
  - https://cseweb.ucsd.edu/classes/sp17/cse120-a/applications/ln/lecture7.html

    - P(x)
    - V(x) 
- P(x) was named from the Dutch word proberen, which means to test.
- V(x) was named from the Dutch word verhogen, which means to increment.

- The pseudo-code below illustrates the semantics of the two semaphore operations. This time the operations are made to be atomic outside of hardware using the hardware support that we discussed earlier -- but more on that soon.

```
    /* proberen - test *.
    P(sem)
    {
       while (sem <= 0)
       ;
       sem = sem - 1;
    }


    /* verhogen - to increment */
    V(sem)
    {
       sem = sem + 1;
    }
    
```

- Condition variables support three operations:

  - wait - add calling thread to the queue and put it to sleep
  - signal - remove a thread form the queue and wake it up
  - broadcast - remove and wake-up all threads on the queue
    - Condition Variables - Typical Use
    ```
      spin_lock s;
  
    GetLock (condition cv, mutex mx)
    {
      mutex_acquire (mx);
      while (LOCKED)
        wait (c, mx);
      
      lock=LOCKED;
      mutex_release (mx);
    }
  
  
    ReleaseLock (condition cv, mutex mx)
    {
      mutex_acquire (mx);
        lock = UNLOCKED;
        signal (cv);
      mutex_release (mx);
    }
    ```

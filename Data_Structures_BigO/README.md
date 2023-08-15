# 자료 구조Data Structures

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

# Big-O Cheat Sheet(그림으로 이쁘게)

https://dev.to/deciduously/big-o-cheat-sheet-3i7d

<br>

<br>

<hr>

<br>

<hr>

# Big-o-Cheat-sheet

![big-o-cheat-sheet-poster](https://user-images.githubusercontent.com/67513038/236633081-7365b27e-a749-4b27-b8bd-717dd3911e76.png)

![bio-o-](https://user-images.githubusercontent.com/67513038/236633173-26995f68-24df-46ec-92df-ef6204d7ec7c.png)

## Big-O Cheat Sheet(그림으로 이쁘게) https://dev.to/deciduously/big-o-cheat-sheet-3i7d

# 유료) 그림으로 알고리즘 이랑 코드 비교해서 알려줌 최고

https://log2base2.com/

```c
struct node *head = NULL;
void addFirst(int val)
{
    struct node *newNode = malloc(sizeof(struct node));
    newNode->data = val;
    newNode->next = head;

    head = newNode;
}

```

<img src="https://github.com/YoungHaKim7/c_project/blob/main/algorithm/images/test1.gif" />

<br>

<hr>

```c
void mirror(struct node *root){
    if(root == NULL)
        return;
    else
    {
        mirror(root->left);
        mirror(root->right);

        struct node *temp = root->left;
        root->left = root->right;
        root->right = temp;
    }
}
    
```

<img src="https://github.com/YoungHaKim7/c_project/blob/main/algorithm/images/test2.gif" />

<br>

<hr>


<br>

<hr>

```c
int *prt;

ptr = malloc(5 * sizeof(int));

ptr = realloc(ptr, 2 * sizeof(int));
ptr = realloc(ptr, 6 * sizeof(int));
    
```

<img src="https://github.com/YoungHaKim7/c_project/blob/main/algorithm/images/test3.gif" />


<br>

<hr>


<img src="https://github.com/YoungHaKim7/c_project/blob/main/algorithm/images/test4.gif" />

<br>

<hr>

<img src="https://github.com/YoungHaKim7/c_project/blob/main/algorithm/images/test5.gif" />

<br>

<hr>




https://log2base2.com/

# Introduction to Programming and Data Structures

https://youtu.be/4OGMB4Fhh50


# C Programming & Data Structrues(Series)

https://youtube.com/playlist?list=PLBlnK6fEyqRhX6r2uhhlubuF5QextdCSM

## C Programming

C Programming:
1) Introduction to the course.
2) Variables 
3) Global vs Local variables. 
4) Data types  
5) Operators in C  
6) Conditionals and loops 
7) Functions 
8) Recursion  
9) Pointers and arrays 
10) Strings
11) Structure and union 
12) File Handling

Data Structures:
1) Stacks  
2) Queues  
3) Linked list 
4) Trees  
5) Binary search trees  
6) Binary Heaps  
7) Graphs  
8) Tree Traversals  
 
X - X - X

<br>

<hr>

<br>

# c언어로 만든 싱글 링크드 리스트 linked list ( 추가, 삽입 , 삭제) 백지부터 설명 시작

https://youtu.be/3ZdafcIvREw

<br>

<hr>

# [📌연결 리스트 완전 정복 10] 이중 연결 리스트(doubly linked list) 역순 연결

<br>

https://youtu.be/bWJma-gywpQ



# (C언어 개발자)네트워크 기초 이론 | 널널한 개발자 TV
- https://youtube.com/playlist?list=PLXvgR_grOs1BFH-TuqFsfHqbh-gpMbFoy&si=oe3t1S-HFtPxw0y3

# 네트워크 기초 무료 강의 | 새내기 개발자들을 위한 필수 가이드 ,ARP  | 크래프트맨 멘탈리티
- https://youtu.be/dsoAkoxZ13o?si=OaqPkyXtvybagCnP

# Computer Networking Course - Network Engineering [CompTIA Network+ Exam Prep] | freeCodeCamp.org
- https://youtu.be/qiQR5rTSshw?si=9LAvwOQybPLVh3nN


<hr>

# Ip-Subnet-calculator

- https://www.calculator.net/ip-subnet-calculator.html

# Routing table

- https://en.wikipedia.org/wiki/Routing_table
  - ```netstate```
    - https://www.linux.co.kr/bbs/board.php?bo_table=lecture&wr_id=4473
  - ```route -n```
    - https://ioflood.com/blog/route-linux-command/

# AS번호란?

- 망식별번호(AS번호)란?

  - 동일한 라우팅 정책으로 하나의 관리자에 의하여 운영되는 네트워크, 즉 한 회사나 단체에서 관리하는 라우터 집단을 자율 시스템(AS, Autonomous System)이라 하며, 각각의 자율 시스템을 식별하기 위한 인터넷 상의 고유한 숫자를 망식별번호(AS번호)라 합니다.
  - 현재 일반적으로 사용중인 AS번호는 2-byte의 체계로 65,536개의 AS번호 사용이 가능하나, IPv4주소와 마찬가지로 가까운 미래에 고갈될 것으로 예측됩니다. 국제 인터넷 표준화기구(IETF, Internet Engineering Task Force)는 기존의 2-byte AS번호 체계의 확장 형태인 4-byte AS번호를 정의하였고,이에 따라 약 43억개의 AS번호를 사용할 수 있게 되었습니다.

    - AS번호 사용자 목록 (2024.01.29 현재) 

      - https://한국인터넷정보센터.한국/jsp/business/management/asList.jsp


# Routing Information Protocol

<p align="center">
  <img src="https://www.ibm.com/docs/en/ssw_ibm_i_71/rzajw/rzajw520.gif" />	
</p>

- https://www.ibm.com/docs/en/i/7.1?topic=methods-dynamic-routing
- https://www.ibm.com/docs/en/i/7.1?topic=routing-information-protocol

<p align="center">
	<img src="https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbhvhde%2FbtrOgy1FJgM%2FbMwdqr1c3vZDPCwVqehtIK%2Fimg.png" />
</p>

- https://systemanswer.tistory.com/entry/Network-Routing-Protocol-%EB%B9%84%EA%B5%90-IGP-EGP

<hr>

# Blocking vs Non-Blocking & Sync vs Async  

- Poll 개념도 나온다. 굿 

<table border="1">
    <tr>
    <td colspan="3" align="center">Block VS. Non - Block</td>
    </tr>
    <tr align="center">
        <td>Sync<br>(Parallel)</td>
        <td>Read/ Write</td>
        <td>Read/Write<br>(Polling)</td>
    </tr>
    <tr align="center">
        <td>Async<br>(Concurrency)</td>
        <td>I/O Multiplexing<br>(Select / Poll)</td>
        <td>Asynchronous I/O</td>
    </tr>
</table>

- https://velog.io/@tjdgus3160/Blocking-vs-Non-Blocking-Sync-vs-Async

- Async/Poll or Sync in HA Architecture 
  - https://stackoverflow.com/questions/4473138/async-poll-or-sync-in-ha-architecture 

- Network에도 poll 개념이 나오고 parallel 에도 나온다. 

- Blocking
  - 호출된 함수가 자신의 작업이 끌날때 까지 제어권을 넘겨주지 않는것

- Non-Blocking
  - 호출된 함수가 바로 제어권을 넘겨주는것

## polling 
- 지속적으로 상태를 체크하여 데이터를 받는 방식

- Polling 과 busy wait의 차이 
  - https://wraithkim.wordpress.com/2020/08/23/busy-wait%EA%B3%BC-polling%EC%9D%98-%EC%B0%A8%EC%9D%B4/ 
  - https://stackoverflow.com/questions/10594426/what-is-the-difference-between-busy-wait-and-polling 


## await
- 
  - https://developer.apple.com/videos/play/wwdc2021/10132/

<hr>

# c#: what is a thread polling?

- https://stackoverflow.com/questions/3849697/c-what-is-a-thread-polling

# UDP Header

<img src="https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99B12B385BD6DC0F03" />

- 자세히 https://joycecoder.tistory.com/20

# TCP Header

<img src="http://www.ktword.co.kr/img_data/1889_1.jpg" />

http://www.ktword.co.kr/test/view/view.php?m_temp1=1889

<hr>

# OSI Model
https://www.geeksforgeeks.org/difference-between-osi-model-and-tcp-ip-model/

<img src="https://media.geeksforgeeks.org/wp-content/uploads/20210220204638/cn1.png" />

- 물리 계층(Physical Layer)
- 데이터링크 계층(Data Link Layer)
- 네트워크 계층(Network Layer)
- 전송 계층(Transport Layer)
- 세션 계층(Session Layer)
- 표현 계층(Presentation Layer)
- 응용 계층(Application Layer)

# TCP/IP Model
https://www.geeksforgeeks.org/difference-between-osi-model-and-tcp-ip-model/

<img src="https://media.geeksforgeeks.org/wp-content/uploads/20230411110343/tcp.jpg" />

# 캡슐화(Encapsulation (computer programming)) & Encapsulation_(networking)

https://en.wikipedia.org/wiki/Encapsulation_(networking)

<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/3b/UDP_encapsulation.svg/420px-UDP_encapsulation.svg.png" />

```mermaid
stateDiagram-v2
    [Message] --> Application_Layer
    Application_Layer --> Transport_Layer
    Transport_Layer --> Network_Layer
    Network_Layer --> Data_Link_Layer
    Data_Link_Layer --> Physical_Layer
    [Message] --> Physical_Layer : Encapsulation
    Physical_Layer --> [Message] : Decapsulation
    Physical_Layer --> Data_Link_Layer
    Data_Link_Layer --> Network_Layer
    Network_Layer --> Transport_Layer
    Transport_Layer --> [Message]
```

```
- 응용 계층
- 전송 계층 -> Port
- 네트워크 계층 -> IP , Routing , ICMP
- 데이터 링크 계층 -> 스위치Switch
- 물리 계층 -> 허브Hub
```

- https://en.wikipedia.org/wiki/Encapsulation_(computer_programming)

- https://en.wikipedia.org/wiki/IP_routing

# What is the OSI model?

- The Open Systems Interconnection (OSI) Model is a description of how the Internet works. It breaks down the functions involved in sending data over the Internet into seven layers. Each layer has some function that prepares the data to be sent over wires, cables, and radio waves as a series of bits.

- The seven layers of the OSI model are:

  7. ```Application layer```: <br>Data generated by and usable by software applications. The main protocol used at this layer is <em><strong>HTTP.</em></strong><br>
  6. ```Presentation layer```: <br>Data is translated into a form the application can accept. Some authorities consider HTTPS encryption and decryption to take place at this layer.<br>
  5. ```Session layer```: <br>Controls connections between computers (this can also be handled at layer 4 by the TCP protocol).<br>
  4. ```Transport layer```: <br>Provides the means for transmitting data between the two connected parties, as well as controlling the quality of service. The main protocols used here are <em><strong>TCP and UDP.</em></strong><br>
  3. ```Network layer```: <br>Handles the routing and sending of data between different networks. The most important protocols at this layer are <em><strong>IP and ICMP.</em></strong><br>
  2. ```Data link layer```: <br>Handles communications between devices on the same network. If layer 3 is like the address on a piece of mail, then layer 2 is like indicating the office number or apartment number at that address. Ethernet is the protocol most used here.<br>
  1. ```Physical layer```:<br>Packets are converted into electrical, radio, or optical pulses and transmitted as bits (the smallest possible units of information) over wires, radio waves, or cables.<br>

https://www.cloudflare.com/learning/network-layer/what-is-the-network-layer/

# OSI계층 & PDU

| OSI계층 | PDU(Protocol Data Unit) |
|-|-|
| 응용 계층<br>표현 계층<br>세션 계층 | 데이터(data) |
| 전송 계층  | 세그먼트(Segment), 데이터그램(datagram) | 
| 네트워크 계층  | 패킷(packet) | 
| 데이터 링크 계층  | 프레임(frame) | 
| 물리 계층  | 비트(bit) | 

- OSI model
https://en.wikipedia.org/wiki/Protocol_data_unit
  - Protocol data units of the OSI model are:[1]
    - The Layer 4: ```transport layer``` PDU is the ```segment``` or the ```datagram```.
    - The Layer 3: ```network layer``` PDU is the ```packet```.
    - The Layer 2: ```data link layer``` PDU is the ```frame```.
    - The Layer 1: ```physical layer``` PDU is the ```bit``` or, more generally, ```symbol```.
- Given a context pertaining to a specific OSI layer, PDU is sometimes used as a synonym for its representation at that layer. 

# 이더넷(Ethernet)

https://ko.wikipedia.org/wiki/%EC%9D%B4%EB%8D%94%EB%84%B7
- 이더넷은 48bit?
  - https://blog.naver.com/voice45/80204482928

| 표준 규격<br>(Ethernet<br>Standard) | 전송 속도 | 연결 매체 종류 |
|-|-|-|
|802.3ab|1 Gbit/s| twisted pair at 1 Gbit/s (125 MB/s) |
|802.3u|100 Mbit/s| Fast Ethernet at 100 Mbit/s (12.5 MB/s) with autonegotiation|
|802.3ae|10 Gbit/s|10 Gigabit Ethernet over fiber; |
|802.3cc|25 Gbit/s|25 Gbit/s over Single Mode Fiber |

- https://en.wikipedia.org/wiki/IEEE_802.3

- http://www.ktword.co.kr/test/view/view.php?m_temp1=412

# 이더켓 케이블

- UTP, FTP, STP 케이블 https://blog.naver.com/dlansduq/221007328226

- CAT.5 ~ CAT.7

|Category| 전송속도 | 대역폭 | 규 격 |
|-|-|-|-|
| CAT.5 | 100 Mbps| 100 MHz | 100 Base-TX |
| CAT.5E | 1 Gbps| 100 MHz | 1000 Base-TX |
| CAT.6 | 1 Gbps| 250 MHz | 1000 Base-TX |
| CAT.6A | 10 Gbps| 500 MHz | 10G Base |
| CAT.7 | 10 Gbps| 600 MHz | 10G Base |

- https://m.blog.naver.com/dlansduq/221007341419

- 최근 몇 년 동안 25GBase-T(25G 트위스트 와이어 이더넷) 기술은 높은 포트 밀도와 낮은 전력 소비 및 네트워크 배포 비용으로 인해 클라우드 컴퓨팅 데이터 센터에서 인기있는 기술이 되었습니다. 25GBase-T는 단일 채널에 걸쳐 25Gbps를 제공하고, 더 높은 대역폭 이더넷 업그레이드를 지원하며, 데이터 센터 및 서버룸에 적합합니다. 이 문서에서는 25Gbase-T에 대한 포괄적인 소개를 제공합니다
  - http://ko.oadm-cwdm-dwdm.com/info/what-s-25gbase-t-52190986.html

|추가 특성 표기<br>(전송매체의 종류)| 의미 |
|-|-|
|C| 동축 케이블(Coaxial cable) |
|T| 트위스티드 페어 케이블(Twisted pair) |
|S| 멀티 모드 광케이블(Multi-mode optical fiber) |
|L| 싱글 모드 광케이블(Single-mode optical fiber) |

- Coaxial cable https://en.wikipedia.org/wiki/Coaxial_cable
- Twisted pair https://en.wikipedia.org/wiki/Twisted_pair
- Multi-mode optical fiber https://en.wikipedia.org/wiki/Multi-mode_optical_fiber
- Single-mode optical fiber https://en.wikipedia.org/wiki/Single-mode_optical_fiber

# Ethernet frame

https://en.wikipedia.org/wiki/Ethernet_frame

<table>
<caption>802.3 Ethernet packet and frame structure
</caption>
<tbody><tr>
<th>Layer</th>
<th>Preamble</th>
<th>Start frame delimiter (SFD)</th>
<th>MAC destination</th>
<th>MAC source</th>
<th><a href="/wiki/802.1Q" class="mw-redirect" title="802.1Q">802.1Q</a> tag (optional)</th>
<th><a href="/wiki/Ethertype" class="mw-redirect" title="Ethertype">Ethertype</a> (<a href="/wiki/Ethernet_II" class="mw-redirect" title="Ethernet II">Ethernet&nbsp;II</a>) or&nbsp;length (<a href="/wiki/IEEE_802.3" title="IEEE 802.3">IEEE&nbsp;802.3</a>)</th>
<th>Payload</th>
<th><a href="/wiki/Frame_check_sequence" title="Frame check sequence">Frame check sequence</a> (32‑bit <a href="/wiki/Cyclic_redundancy_check" title="Cyclic redundancy check">CRC</a>)</th>
<th><a href="/wiki/Interpacket_gap" title="Interpacket gap">Interpacket&nbsp;gap (IPG)</a>
</th></tr>
<tr>
<td>
</td>
<td>7 <a href="/wiki/Octet_(computing)" title="Octet (computing)">octets</a></td>
<td>1 octet</td>
<td>6 octets</td>
<td>6&nbsp;octets</td>
<td>(4 octets)</td>
<td>2 octets</td>
<td>46-1500 octets</td>
<td><span class="nowrap">4 octets</span></td>
<td>12 octets
</td></tr>
<tr>
<td><a href="/wiki/Data_link_layer" title="Data link layer">Layer 2</a> Ethernet frame
</td>
<td colspan="2">(not part of the frame)</td>
<td colspan="6" style="background:#fdd;"><span class="nowrap">← 64–1522 octets →</span></td>
<td>(not part of the frame)
</td></tr>
<tr>
<td><a href="/wiki/Physical_layer" title="Physical layer">Layer 1</a> Ethernet packet &amp; IPG
</td>
<td colspan="8" style="background:#fdd;"><span class="nowrap">← 72–1530 octets →</span></td>
<td style="background:#fdd;">← 12 octets&nbsp;→
</td></tr></tbody>
</table>


- 이더넷 프레임 : 이더넷 네트워크에서 주고 받는 데이터 형식

<table border="1">
    <tr>
    <td colspan="6" align="center">Ethernet frame</td>
    </tr>
    <tr align="center">
        <td colspan="4" align="center">←----------- 헤더 -----------→</td>
        <td>←-- 페이로드 --→</td>
        <td>← 트레일러 →</td>
    </tr>
    <tr align="center">
        <td>프리앰블</td>
        <td>목적지 MAC<br>주소</td>
        <td>송신지 MAC<br>주소</td>
        <td>이더타입/<br>길이</td>
        <td>데이터</td>
        <td>FCS</td>
    </tr>
    <tr align="center">
        <td>8 바이트</td>
        <td>6 바이트</td>
        <td>6 바이트</td>
        <td>2 바이트</td>
        <td>46 ~ 1500 바이트</td>
        <td>4 바이트</td>
    </tr>
  
</table>

  - 이더넷 프레임
    - https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
<table border="1">
    <tr>
    <td colspan="6" align="center">Ethernet frame</td>
    </tr>
    <tr align="center">
        <td>Ethertype<br>(decimal)</td>
        <td>Ethertype<br>(hex)</td>
        <td>Exp.<br>Ethernet<br>(decimal)</td>
        <td>Exp.<br>Ethernet<br>(octal)</td>
        <td>Description</td>
        <td>Reference</td>
    </tr>
        <tr align="center">
        <td>2048</td>
        <td>0800</td>
        <td>513</td>
        <td>1001</td>
        <td>Internet Protocol version 4 (IPv4)</td>
        <td>[<a href="https://datatracker.ietf.org/doc/draft-ietf-intarea-rfc7042bis/11/">RFC-ietf-intarea-rfc7042bis-11</a>]</td>
    </tr>
        </tr>
        <tr align="center">
        <td>34525</td>
        <td>86DD</td>
        <td></td>
        <td></td>
        <td>Internet Protocol version 6 (IPv6)</td>
        <td>[<a href="https://datatracker.ietf.org/doc/draft-ietf-intarea-rfc7042bis/11/">RFC-ietf-intarea-rfc7042bis-11</a>]</td>
    </tr>
        </tr>
        <tr align="center">
        <td>2054</td>
        <td>0806</td>
        <td></td>
        <td></td>
        <td>Address Resolution Protocol (ARP)</td>
        <td>[<a href="https://datatracker.ietf.org/doc/draft-ietf-intarea-rfc7042bis/11/">RFC-ietf-intarea-rfc7042bis-11</a>]</td>
    </tr>
        </tr>
        <tr align="center">
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
</table>

- 출처 : https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
  - Full Duplex & Half Duplex
    - https://en.wikipedia.org/wiki/Duplex_(telecommunications)

<hr>

<hr>

# Hub

```
1. Flooding
2. Forwarding & Filtering
3. Aging
```
- 1. https://en.wikipedia.org/wiki/MAC_flooding
- https://hihighlinux.tistory.com/95

<hr>

- 네트워크 총정리 요약본 CheatSheet http://datamining.rutgers.edu/teaching/spring2010/CNA/lecture16.pdf
  - Network Switch - Concept and Working - Filtering | Flooding | Forwarding | Learning | Hacks and Security - Second Channe
    -  https://youtu.be/Xh6Y_Wto33w?si=oBotzeYev7MYff4w
  - Switching in networking in Hindi | CCNA | Network Kings
    - https://youtu.be/I1McrK2XILk?si=md3UMmakfdzzf5n7


- Network Devices – How Hubs and Switches Work and How to Secure Them
  - https://www.freecodecamp.org/news/how-hub-switch-work-and-how-to-protect-them/
 
- Network Devices (Hub, Repeater, Bridge, Switch, Router, Gateways and Brouter)
  - https://www.geeksforgeeks.org/network-devices-hub-repeater-bridge-switch-router-gateways/

<hr>

# IP

- IP에서 MTU개념 이해(In computer networking, the maximum transmission unit (MTU))
  - Pv4 allows fragmentation which divides the datagram into pieces ...
    - https://en.wikipedia.org/wiki/Maximum_transmission_unit

- ```ifconfig``` macOS에서 test함

```bash
$ ifconfig
lo0: flags=8049<UP,LOOPBACK,RUNNING,MULTICAST> mtu 16384
	options=1203<RXCSUM,TXCSUM,TXSTATUS,SW_TIMESTAMP>
	inet 127.0.0.1 netmask 0xff000000
	inet6 ::1 prefixlen 128
	inet6 fe80::1%lo0 prefixlen 64 scopeid 0x1
	nd6 options=201<PERFORMNUD,DAD>
gif0: flags=8010<POINTOPOINT,MULTICAST> mtu 1280
```

```
1.4 Operation (IP)
The internet protocol implements two basic functions: addressing and fragmentation.
```

https://datatracker.ietf.org/doc/html/rfc791

- IPv4 Header
  - http://www.ktword.co.kr/test/view/view.php?m_temp1=1859
  - https://en.wikipedia.org/wiki/Internet_Protocol_version_4

<hr>

- MAC Address

<img src="https://www.baeldung.com/wp-content/uploads/sites/4/2023/02/MACFormat.png" />

<table border="1">
    <tr>
    <td colspan="6" align="center">MAC<br>Media Access Control Address</td>
    </tr>
    <tr align="center">
        <td>A1</td>
        <td>B2</td>
        <td>C3</td>
        <td>D4</td>
        <td>E5</td>
        <td>F6</td>
    </tr>
    <tr align="center">
        <td colspan="3">Organizationally<br>Unique Identifier<br>24bits<br>Manufacturer<br>Identification</td>
        <td colspan="3">Universally<br>Administered Address<br>24bits<br>NIC's Unique Number</td>
    </tr>
    <tr align="center">
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
</table>

- 출처 : https://www.baeldung.com/cs/understanding-mac-addresses


- 출처 : https://developnote-blog.tistory.com/m/entry/MAC-Address
<img src="https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcgsHfe%2FbtrvGYkUObJ%2FE14EXjH0MA56GFN0kklbJ0%2Fimg.png" />
- 24bit + 24bit

- Universal/local and individual/group bits in MAC addresses
<table>
<caption>Universal/local and individual/group bits in MAC addresses
</caption>
<tbody><tr>
<th style="background:#EAECF0;background:linear-gradient(to top right,#EAECF0 49%,#AAA 49.5%,#AAA 50.5%,#EAECF0 51%);line-height:1.2;padding:0.1em 0.4em;"><div style="margin-left:2em;text-align:right">U/L</div><div style="margin-right:2em;text-align:left">I/G</div>
</th>
<th>Universally administered
</th>
<th>Locally administered
</th></tr>
<tr>
<th>Unicast (individual)
</th>
<td><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>0</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>4</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>8</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>C</u></b>-XX-XX-XX-XX-XX</i></span></span>
</td>
<td><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>2</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>6</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>A</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>E</u></b>-XX-XX-XX-XX-XX</i></span></span>
</td></tr>
<tr>
<th>Multicast (group)
</th>
<td><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>1</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>5</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>9</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>D</u></b>-XX-XX-XX-XX-XX</i></span></span>
</td>
<td><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>3</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>7</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>B</u></b>-XX-XX-XX-XX-XX</i></span></span><br><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>F</u></b>-XX-XX-XX-XX-XX</i></span></span>
</td></tr></tbody>
</table>

- SLAP quadrants for unicast local MAC addresses

<table>
<caption>SLAP quadrants for unicast local MAC addresses
</caption>
<tbody><tr>
<th>MAC address
</th>
<th>Quadrant name
</th>
<th>Identifier
</th>
<th>Usage
</th></tr>
<tr>
<th><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>A</u></b>-XX-XX-XX-XX-XX</i></span></span>
</th>
<td>Extended local
</td>
<td>ELI
</td>
<td>Assigned by IEEE, but uses a unique 3-octet company ID (CID) instead of an OUI.
</td></tr>
<tr>
<th><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>E</u></b>-XX-XX-XX-XX-XX</i></span></span>
</th>
<td>Standard assigned
</td>
<td>SAI
</td>
<td>For use in the forthcoming IEEE P802.1CQ specification, to be assigned dynamically by the Block Address Registration and Claiming (BARC) protocol.
</td></tr>
<tr>
<th><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>2</u></b>-XX-XX-XX-XX-XX</i></span></span>
</th>
<td>Administratively assigned
</td>
<td>AAI
</td>
<td>Can be randomly or arbitrarily assigned to devices.
</td></tr>
<tr>
<th><span style="padding-right: 1px;"><span class="macaddr"><i>X<b><u>6</u></b>-XX-XX-XX-XX-XX</i></span></span>
</th>
<td><i>Reserved</i>
</td>
<td><i>Reserved</i>
</td>
<td>Reserved for future use, but may be used similarly to AAI until an IEEE specification utilizes this space.
</td></tr></tbody>
</table>

- 출처 : https://en.wikipedia.org/wiki/MAC_address

- ARP 이해하기 https://aws-hyoh.tistory.com/entry/ARP-%EC%89%BD%EA%B2%8C-%EC%9D%B4%ED%95%B4%ED%95%98%EA%B8%B0
```
arp -a
? (1xx.30.1.xx) at 3f:8e:d3:fe:dc:8b on en0 ifscope [ethernet]
? (1xx.30.1.100) at 36:e7:f3:fd:7b:af on en0 ifscope [ethernet]
? (1xx.30.1.128) at 0:1d:ec:5c:1f:6e on en0 ifscope [ethernet]
? (1xx.30.1.254) at 88:3c:1c:f0:f6:8f on en0 ifscope [ethernet]
? (1xx.30.1.255) at ff:ff:ff:ff:ff:ff on en0 ifscope [ethernet]
mdns.mcast.net (224.0.0.251) at 1:0:5f:0:f:fb on en0 ifscope permanent [ethernet]
? (2xx.2xx.2xx.2xx) at 1:0:fe:7f:ff:fa on en0 ifscope permanent [ethernet]
```

# Address (Class A, B, C)
https://en.wikipedia.org/wiki/Classful_network


https://limkydev.tistory.com/168

<img src="https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Ft1.daumcdn.net%2Fcfile%2Ftistory%2F99068D495BE8101D34" />

<img src="https://xn--3e0bx5euxnjje69i70af08bea817g.xn--3e0b707e/images/ip_as/imgipasSys03.gif" />

출처 : https://한국인터넷정보센터.한국/jsp/resources/ipv4Info.jsp

<table>
<tbody><tr>
<th>Class
</th>
<th>Leading bits
</th>
<th>Size of <i>network number</i> bit field
</th>
<th>Size of <i>rest</i> bit field
</th>
<th>Number of networks
</th>
<th>Addresses per network
</th>
<th>Total addresses in class
</th>
<th>Start address
</th>
<th>End address
</th>
<th>Default <a href="/wiki/Subnet_mask" class="mw-redirect" title="Subnet mask">subnet mask</a> in <a href="/wiki/Dot-decimal_notation" title="Dot-decimal notation">dot-decimal notation</a>
</th>
<th><a href="/wiki/CIDR_notation" class="mw-redirect" title="CIDR notation">CIDR notation</a>
</th></tr>
<tr>
<td>Class A
</td>
<td>0
</td>
<td>8
</td>
<td>24
</td>
<td>128 (2<sup>7</sup>)
</td>
<td>16,777,216 (2<sup>24</sup>)
</td>
<td>2,147,483,648 (2<sup>31</sup>)
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">0.0.0.0</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">127.255.255.255</span></span></i><sup id="cite_ref-7" class="reference"><a href="#cite_note-7">[a]</a></sup>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">255.0.0.0</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">/</span><span style="padding-right: 1px;">8</span></span></i>
</td></tr>
<tr>
<td>Class B
</td>
<td>10
</td>
<td>16
</td>
<td>16
</td>
<td>16,384 (2<sup>14</sup>)
</td>
<td>65,536 (2<sup>16</sup>)
</td>
<td>1,073,741,824 (2<sup>30</sup>)
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">128.0.0.0</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">191.255.255.255</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">255.255.0.0</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">/</span><span style="padding-right: 1px;">16</span></span></i>
</td></tr>
<tr>
<td>Class C
</td>
<td>110
</td>
<td>24
</td>
<td>8
</td>
<td>2,097,152 (2<sup>21</sup>)
</td>
<td>256 (2<sup>8</sup>)
</td>
<td>536,870,912 (2<sup>29</sup>)
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">192.0.0.0</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">223.255.255.255</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">255.255.255.0</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">/</span><span style="padding-right: 1px;">24</span></span></i>
</td></tr>
<tr>
<td>Class D (<a href="/wiki/Multicast" title="Multicast">multicast</a>)
</td>
<td>1110
</td>
<td>not defined
</td>
<td>not defined
</td>
<td>not defined
</td>
<td>not defined
</td>
<td>268,435,456 (2<sup>28</sup>)
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">224.0.0.0</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">239.255.255.255</span></span></i>
</td>
<td>not defined
</td>
<td>/4<sup id="cite_ref-8" class="reference"><a href="#cite_note-8">[7]</a></sup>
</td></tr>
<tr>
<td>Class E (reserved)
</td>
<td>1111
</td>
<td>not defined
</td>
<td>not defined
</td>
<td>not defined
</td>
<td>not defined
</td>
<td>268,435,456 (2<sup>28</sup>)
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">240.0.0.0</span></span></i>
</td>
<td><i><span class="ipaddr"><span style="padding-right: 1px;">255.255.255.255</span></span></i><sup id="cite_ref-9" class="reference"><a href="#cite_note-9">[b]</a></sup>
</td>
<td>not defined
</td>
<td>not defined
</td></tr></tbody>
</table>

<hr>

# 예약된 IP 주소

- 출처 : https://ko.ipshu.com/whats_pubip_prvip_revip#google_vignette

<table>
<tbody><tr>
<th>주소 블록</th>
<th>주소 범위</th>
<th>주소 수</th>
<th>범위</th>
<th>설명</th>
</tr>
<tr>
<td>0.0.0.0/8</td>
<td>0.0.0.0–0.255.255.255</td>
<td>16,777,216</td>
<td>소프트웨어</td>
<td>현재 네트워크 (소스 주소로만 유효)</td>
</tr>
<tr>
<td>10.0.0.0/8</td>
<td>10.0.0.0–10.255.255.255</td>
<td>16,777,216</td>
<td>개인 네트워크</td>
<td>개인 네트워크 내의 로컬 통신에 사용됩니다.</td>
</tr>
<tr>
<td>100.64.0.0/10</td>
<td>100.64.0.0–100.127.255.255</td>
<td>4,194,304</td>
<td>개인 네트워크</td>
<td>캐리어 급 NAT를 사용할 때 서비스 제공 업체와 가입자 사이의 통신을위한 공유 주소 공간.</td>
</tr>
<tr>
<td>127.0.0.0/8</td>
<td>127.0.0.0–127.255.255.255</td>
<td>16,777,216</td>
<td>호스트</td>
<td>로컬 호스트로의 루프백 주소에 사용됩니다.</td>
</tr>
<tr>
<td>169.254.0.0/16</td>
<td>169.254.0.0–169.254.255.255</td>
<td>65,536</td>
<td>서브넷</td>
<td>일반적으로 DHCP 서버에서 검색되는 것과 같이 IP 주소가 지정되어 있지 않은 경우 단일 링크에서 두 호스트 간의 링크 로컬 주소에 사용됩니다.</td>
</tr>
<tr>
<td>172.16.0.0/12</td>
<td>172.16.0.0–172.31.255.255</td>
<td>1,048,576</td>
<td>개인 네트워크</td>
<td>개인 네트워크 내의 로컬 통신에 사용됩니다.</td>
</tr>
<tr>
<td>192.0.0.0/24</td>
<td>192.0.0.0–192.0.0.255</td>
<td>256</td>
<td>개인 네트워크</td>
<td>IETF 프로토콜 할당.</td>
</tr>
<tr>
<td>192.0.2.0/24</td>
<td>192.0.2.0–192.0.2.255</td>
<td>256</td>
<td>선적 서류 비치</td>
<td>TEST-NET-1, 문서 및 예제로 지정되었습니다.</td>
</tr>
<tr>
<td>192.88.99.0/24</td>
<td>192.88.99.0–192.88.99.255</td>
<td>256</td>
<td>인터넷</td>
<td>예약석. IPv6-IPv4 릴레이 (IPv6 주소 블록 2002 :: / 16 포함).</td>
</tr>
<tr>
<td>192.168.0.0/16</td>
<td>192.168.0.0–192.168.255.255</td>
<td>65,536</td>
<td>개인 네트워크</td>
<td>개인 네트워크 내의 로컬 통신에 사용됩니다.</td>
</tr>
<tr>
<td>198.18.0.0/15</td>
<td>198.18.0.0–198.19.255.255</td>
<td>131,072</td>
<td>개인 네트워크</td>
<td>두 개의 개별 서브넷 간 네트워크 간 통신 벤치 마크 테스트에 사용됩니다.</td>
</tr>
<tr>
<td>198.51.100.0/24</td>
<td>198.51.100.0–198.51.100.255</td>
<td>256</td>
<td>선적 서류 비치</td>
<td>TEST-NET-2, 문서 및 예제로 지정됩니다.</td>
</tr>
<tr>
<td>203.0.113.0/24</td>
<td>203.0.113.0–203.0.113.255</td>
<td>256</td>
<td>선적 서류 비치</td>
<td>TEST-NET-3, 문서 및 예제로 지정됩니다.</td>
</tr>
<tr>
<td>224.0.0.0/4</td>
<td>224.0.0.0–239.255.255.255</td>
<td>268,435,456</td>
<td>인터넷</td>
<td>IP 멀티 캐스트에 사용 중입니다. (이전 클래스 D 네트워크).</td>
</tr>
<tr>
<td>240.0.0.0/4</td>
<td>240.0.0.0–255.255.255.254</td>
<td>268,435,455</td>
<td>인터넷</td>
<td>나중에 사용하기 위해 예약되어 있습니다. (이전 클래스 E 네트워크).</td>
</tr>
<tr>
<td>255.255.255.255/32</td>
<td>255.255.255.255</td>
<td>1</td>
<td>서브넷</td>
<td>"제한된 브로드 캐스트"대상 주소로 예약되어 있습니다.</td>
</tr>
</tbody></table>


# linux에서 network 변경 ```nmtui```

- https://easyitwanner.tistory.com/113

# Port

- https://en.wikipedia.org/wiki/Port_(computer_networking)
  - For TCP and UDP, a port number is a 16-bit unsigned integer, thus ranging from 0 to 65535.

| 포트종류       | 번호 범위     |
| -------------- |-------------:|
| Well known ports| 0 ~ 1023     |
| registered ports| 1024 ~ 49151 |
| dynamic ports   | 49152 ~ 65535|

- The registered ports are those from 1024 through 49151. IANA maintains the official list of well-known and registered ranges.

- The dynamic or private ports are those from 49152 through 65535. One common use for this range is for ephemeral ports. 

| 포트 No.       | 설명    |
| -------------- |-------------:|
| 443<br>1194|  OpenVPN TCP connections3<br>OpenVPN UDP connections     |
| 1433<br>1434 | MSSQLSERVER instance (static TCP port) <br> UDP connections|
| 3306   | MySQL|
| 6379   | Redis|
| 8080   | Http 대체|
| 27017   | MonggoDB|




- https://support.nordvpn.com/hc/en-us/articles/19683394518161-OpenVPN-connection-on-NordVPN
- https://blog.devart.com/sql-server-ports.html
- https://dev.mysql.com/doc/mysql-port-reference/en/mysql-port-reference-tables.html
- https://redis.io/docs/management/config/
- https://www.speedguide.net/port.php?port=8080
- https://www.mongodb.com/docs/manual/reference/default-mongodb-port/

- 잘 알려진 포트 & 등록된 포트 모아보기
  - https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.txt

<table class="wikitable">
<caption>Notable well-known port numbers
</caption>
<tbody><tr>
<th scope="col">Number
</th>
<th scope="col">Assignment
</th></tr>
<tr>
<th scope="row">20
</th>
<td><a href="/wiki/File_Transfer_Protocol" title="File Transfer Protocol">File Transfer Protocol</a> (FTP) Data Transfer
</td></tr>
<tr>
<th scope="row">21
</th>
<td><a href="/wiki/File_Transfer_Protocol" title="File Transfer Protocol">File Transfer Protocol</a> (FTP) Command Control
</td></tr>
<tr>
<th scope="row">22
</th>
<td><a href="/wiki/Secure_Shell" title="Secure Shell">Secure Shell</a> (SSH) Secure Login
</td></tr>
<tr>
<th scope="row">23
</th>
<td><a href="/wiki/Telnet" title="Telnet">Telnet</a> remote login service, unencrypted text messages
</td></tr>
<tr>
<th scope="row">25
</th>
<td><a href="/wiki/Simple_Mail_Transfer_Protocol" title="Simple Mail Transfer Protocol">Simple Mail Transfer Protocol</a> (SMTP) email delivery
</td></tr>
<tr>
<th scope="row">53
</th>
<td><a href="/wiki/Domain_Name_System" title="Domain Name System">Domain Name System</a> (DNS) service
</td></tr>
<tr>
<th scope="row">67, 68
</th>
<td><a href="/wiki/Dynamic_Host_Configuration_Protocol" title="Dynamic Host Configuration Protocol">Dynamic Host Configuration Protocol</a> (DHCP)
</td></tr>
<tr>
<th scope="row">80
</th>
<td><a href="/wiki/Hypertext_Transfer_Protocol" class="mw-redirect" title="Hypertext Transfer Protocol">Hypertext Transfer Protocol</a> (HTTP) used in the <a href="/wiki/World_Wide_Web" title="World Wide Web">World Wide Web</a>
</td></tr>
<tr>
<th scope="row">110
</th>
<td><a href="/wiki/Post_Office_Protocol" title="Post Office Protocol">Post Office Protocol</a> (POP3)
</td></tr>
<tr>
<th scope="row">119
</th>
<td><a href="/wiki/Network_News_Transfer_Protocol" title="Network News Transfer Protocol">Network News Transfer Protocol</a> (NNTP)
</td></tr>
<tr>
<th scope="row">123
</th>
<td><a href="/wiki/Network_Time_Protocol" title="Network Time Protocol">Network Time Protocol</a> (NTP)
</td></tr>
<tr>
<th scope="row">143
</th>
<td><a href="/wiki/Internet_Message_Access_Protocol" title="Internet Message Access Protocol">Internet Message Access Protocol</a> (IMAP) Management of digital mail
</td></tr>
<tr>
<th scope="row">161
</th>
<td><a href="/wiki/Simple_Network_Management_Protocol" title="Simple Network Management Protocol">Simple Network Management Protocol</a> (SNMP)
</td></tr>
<tr>
<th scope="row">194
</th>
<td><a href="/wiki/Internet_Relay_Chat" title="Internet Relay Chat">Internet Relay Chat</a> (IRC)
</td></tr>
<tr>
<th scope="row">443
</th>
<td><a href="/wiki/HTTP_Secure" class="mw-redirect" title="HTTP Secure">HTTP Secure</a> (HTTPS) HTTP over TLS/SSL
</td></tr>
<tr>
<th scope="row">546, 547
</th>
<td><a href="/wiki/DHCPv6" title="DHCPv6">DHCPv6</a> IPv6 version of DHCP
</td></tr></tbody></table>

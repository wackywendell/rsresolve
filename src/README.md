```
$ sudo apt install libpcap-dev

$ dig datadoghq.com @8.8.8.8
$ sudo tcpdump --interface=wlp0s20f3 -n host 8.8.8.8 -w dnsqueries.pcap
```

```
❯ dig datadoghq.com @8.8.8.8 +qr

; <<>> DiG 9.16.8-Ubuntu <<>> datadoghq.com @8.8.8.8 +qr
;; global options: +cmd
;; Sending:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 22848
;; flags: rd ad; QUERY: 1, ANSWER: 0, AUTHORITY: 0, ADDITIONAL: 1

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 4096
; COOKIE: d8f6b80c1a874b52
;; QUESTION SECTION:
;datadoghq.com.			IN	A

;; QUERY SIZE: 54

;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 22848
;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 1

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 512
;; QUESTION SECTION:
;datadoghq.com.			IN	A

;; ANSWER SECTION:
datadoghq.com.		60	IN	A	13.35.73.80
datadoghq.com.		60	IN	A	13.35.73.123
datadoghq.com.		60	IN	A	13.35.73.129
datadoghq.com.		60	IN	A	13.35.73.88

;; Query time: 28 msec
;; SERVER: 8.8.8.8#53(8.8.8.8)
;; WHEN: Sat Apr 16 20:25:31 EDT 2022
;; MSG SIZE  rcvd: 106
```
<p align="center"><a href="https://github.com/GyulyVGC/sniffnet"><img alt="Sniffnet" src="resources/header_repository.png" width="100%"/></a></p>


<hr>

<p align="center"> Application to comfortably monitor your network traffic </p>
<p align="center"> Multithreaded, cross-platform, reliable </p>

<p align="center">
<a href="https://crates.io/crates/sniffnet"><img alt="" src="https://img.shields.io/crates/v/sniffnet?color=orange&logo=rust"/></a>
&nbsp;
<a href="https://github.com/GyulyVGC/sniffnet"><img alt="" src="https://img.shields.io/github/stars/gyulyvgc/sniffnet?logo=github"/></a>
</p>


<hr>

<p align="center"><img alt="" src="resources/sniffnet_run_example.gif" width="85%"/></p>

<div align="center">
<p>Sniffnet is a simple yet insightful application to let you have a glance into your network traffic 
in a straightforward and appealing way </p>
</div>

<hr>


## Install and Run

If you have [Rust installed](https://www.rust-lang.org/tools/install) on your machine, the application binary can be installed with: 
```sh
cargo install sniffnet
```

On NetBSD a pre-compiled binary is available from the official repositories. To install this simply run:
```sh
pkgin install sniffnet
```

Or, if you prefer to build it from source:
```sh
cd /usr/pkgsrc/net/sniffnet
make install
```

Otherwise, you can install Sniffnet through the installers available in the [latest release](https://github.com/GyulyVGC/sniffnet/releases).


<details>

  <summary>Windows dependencies&emsp;<img alt="" src="https://user-images.githubusercontent.com/100347457/193474292-d84f2a96-f445-40ac-8930-9d0f00a3c2bb.png" width="35px"/></summary>

  In order to correctly run Sniffnet on Windows systems you need to:

  - Install [Npcap](https://npcap.com/#download).

  - Download the [Npcap SDK](https://npcap.com/#download).

  - Add the SDK's ```/Lib``` or ```/Lib/x64``` folder to your ```LIB``` environment variable.
    
</details>


<details>

  <summary>Linux dependencies&emsp;<img alt="" src="https://user-images.githubusercontent.com/100347457/193474239-c48d37af-d4c1-4a94-9207-0d46c6d75f1f.png" width="35px"/></summary>
 
  In order to correctly run Sniffnet on Linux systems, install the libraries and header files for the libpcap library: 
  
```sh
sudo apt-get install libpcap-dev
```

  Note that if you are not running as root, you need to set capabilities to inspect a network adapter: 
  
```sh
sudo setcap cap_net_raw,cap_net_admin=eip <your/Sniffnet/executable/path>
```
    
</details>


<details>

  <summary>MacOS dependencies&emsp;<img alt="" src="https://user-images.githubusercontent.com/100347457/193474398-7637e269-3e92-44bc-87c0-8ea18ca95693.png" width="35px"/></summary>

  MacOS natively has all the dependencies you need to build and run Sniffnet!
    
</details>


## Features

- choose a network adapter to inspect
<p align="center"><img alt="" src="resources/adapters_view.png" width="50%"/></p>

- select filters to apply to the observed traffic
<p align="center"><img alt="" src="resources/filters_view.png" width="60%"/></p>
  
- view real-time charts about traffic intensity (bytes and packets per second, incoming and outgoing)
<p align="center"><img alt="" src="resources/charts_view.png" width="80%"/></p>

- view overall statistics about the filtered traffic
<p align="center"><img alt="" src="resources/packets_view.png" width="50%"/></p>

- view most relevant connections (most recent, most packets, most bytes)
<p align="center"><img alt="" src="resources/report_view.png" width="95%"/></p>

- save complete textual report with detailed information for each connection:
  * source and destination IP addresses
  * source and destination ports
  * carried protocols
  * amount of exchanged packets and bytes
  * initial and final timestamp of information exchange
  

## Supported application layer protocols

<details>

  <summary>See details</summary>
  
  <br>
  
  Please, note that application layer protocols are just inferred from the transport port numbers.
  
  <br>
  
<div align="center">

|Port number(s)|Application protocol  |  Description |
|--|--|--|
| 20, 21 | FTP |File Transfer Protocol |
|22|SSH |Secure Shell |
|23|Telnet |Telnet |
|25|SMTP |Simple Mail Transfer Protocol |
|49|TACACS |Terminal Access Controller Access-Control System |
|53|DNS |Domain Name System |
|67, 68|DHCP |Dynamic Host Configuration Protocol |
|69|TFTP |Trivial File Transfer Protocol |
|80, 8080|HTTP |Hypertext Transfer Protocol |
|109, 110|POP |Post Office Protocol |
|123|NTP |Network Time Protocol |
|137, 138, 139|NetBIOS |NetBIOS |
|143, 220|IMAP |Internet Message Access Protocol |
|161, 162, 199|SNMP |Simple Network Management Protocol |
|179|BGP |Border Gateway Protocol |
|389|LDAP |Lightweight Directory Access Protocol |
|443|HTTPS |Hypertext Transfer Protocol over SSL/TLS |
|636|LDAPS |Lightweight Directory Access Protocol over TLS/SSL |
|989, 990|FTPS |File Transfer Protocol over TLS/SSL |
|993|IMAPS |Internet Message Access Protocol over TLS/SSL |
|995|POP3S |Post Office Protocol 3 over TLS/SSL |
|1900|SSDP |Simple Service Discovery Protocol |
|5222|XMPP |Extensible Messaging and Presence Protocol |
|5353|mDNS |Multicast DNS |

</div>

</details>


## Troubleshooting

<details>

  <summary>See details</summary>

### Missing dependencies

Most of the errors that can occur are likely due to your system missing required pcap dependencies,
necessary to correctly analyze a network adapter. 

Check the [Install and Run](#install-and-run) section for instructions on how to proceed.

For a Windows reference, you can check [issue #1](https://github.com/GyulyVGC/sniffnet/issues/1).

### Installers incompatibilities

If you have problems after having installed Sniffnet through the provided installers,
it could be due to your OS not being compatible with the pre-built binaries I generated for you.

Reach me out, and I'll try to generate an installer for your specific operating system.

### ***In any case don't hesitate to [open an issue](https://github.com/GyulyVGC/sniffnet/issues), and I will do my best to help you!***

</details>


## Contribute

Do you want to improve Sniffnet? Check [here](https://github.com/GyulyVGC/sniffnet/blob/main/CONTRIBUTING.md) 


## Stargazers

<a href="https://github.com/GyulyVGC/sniffnet/stargazers"><img alt="" src="https://reporoster.com/stars/dark/GyulyVGC/sniffnet"/></a>

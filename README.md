# README: yb-overview

This is the sourcecode for a tool that reads the YugabyteDB master http port(s), and generates an overview of the YugabyteDB cluster status as registered by the masters.

By default, the yb-overview tool tries to use localhost:7000.
Optionally, a list of masters (address:port) separated by a comma can be provided via the `-m` switch:

```
./yb-overview -m 192.168.66.80:7000,192.168.66.81:7000,192.168.66.82:7000
```

This is how the output looks like:

```
fritshoogland@MacBook-Pro-van-Frits % ./ybtool

Master UUID                      RPC Host:Port        State         Role
7d484fdc8aa14fd59fb43052aac21329 yb-1.local:7100      ALIVE         FOLLOWER
0c4e103e31ca4c9280447c22e628ae04 yb-2.local:7100      ALIVE         LEADER
e35833c9dcb74677810bc4658ea4d1a9 yb-3.local:7100      ALIVE         FOLLOWER

HTTP Host:Port       Heartbeat delay Status   Rd op/s  Wr op/s     Uptime SST tot sz SST uncomp sz SST #files     Memory
yb-1.local:9000                 0.2s ALIVE          0        0      86332  161.93 MB     260.64 MB         11   63.22 MB
yb-2.local:9000                 0.3s ALIVE          0        0     194144  162.14 MB     260.70 MB         12   70.64 MB
yb-3.local:9000                 0.2s ALIVE          0        0      95766  158.84 MB     255.79 MB         11   83.22 MB

Hostname             Path                    Used MB   Total MB  Used %
yb-1.local:9000      /mnt/d0                    4351      10228  42.55%
yb-2.local:9000      /mnt/d0                    3020      10228  29.53%
yb-3.local:9000      /mnt/d0                     451      10228  4.42%

```
- The first table is the masters overview, equivalent to yb-admin list_all_masters output.  
- The second table is the tablet servers overview, equivalent to the yb-admin list_all_tablet_servers output, excluding the UUIDs.
- The third table is available via http://(master|tablet server):(7000|9000)/drives for each individual server.

# Special features
## master failure
yb-overview tries to obtain the information from the master addresses in the exact order specified. If it doesn't succeed, it tries the next specified address, however it will notify the failure:
(please note it only requires one master to read the master status data)
```
fritshoogland@MacBook-Pro-van-Frits % ./ybtool -m 192.168.66.80:7000,192.168.66.81:7000,192.168.66.82:7000
Warning: master not responding on: 192.168.66.80:7000

Master UUID                      RPC Host:Port        State         Role
7d484fdc8aa14fd59fb43052aac21329 yb-1.local:7100      NETWORK_ERROR UNKNOWN
0c4e103e31ca4c9280447c22e628ae04 yb-2.local:7100      ALIVE         LEADER
e35833c9dcb74677810bc4658ea4d1a9 yb-3.local:7100      ALIVE         FOLLOWER
...
```
The first line raises a warning, and the masters overview also shows the master in the state NETWORK_ERROR.

## tablet server failure
yb-overview reads the tablet server information from the master. A tablet server is considered "dead" when it doesn't provide heartbeats for 60 seconds by default. Once that happens, it will show be visible with the status DEAD:
```
fritshoogland@MacBook-Pro-van-Frits % ./ybtool -m 192.168.66.80:7000,192.168.66.81:7000,192.168.66.82:7000

Master UUID                      RPC Host:Port        State         Role
7d484fdc8aa14fd59fb43052aac21329 yb-1.local:7100      ALIVE         FOLLOWER
0c4e103e31ca4c9280447c22e628ae04 yb-2.local:7100      ALIVE         LEADER
e35833c9dcb74677810bc4658ea4d1a9 yb-3.local:7100      ALIVE         FOLLOWER

HTTP Host:Port       Heartbeat delay Status   Rd op/s  Wr op/s     Uptime SST tot sz SST uncomp sz SST #files     Memory
yb-1.local:9000                61.2s DEAD           0        0          0        0 B           0 B          0        0 B
yb-2.local:9000                 0.3s ALIVE          0        0     197454  162.14 MB     260.70 MB         12   66.02 MB
yb-3.local:9000                 0.2s ALIVE          0        0      99079  158.84 MB     255.79 MB         11   79.65 MB

Hostname             Path                    Used MB   Total MB  Used %
yb-2.local:9000      /mnt/d0                    3031      10228  29.64%
yb-3.local:9000      /mnt/d0                     772      10228  7.55%

Missing (dead) tablet server: bba84d237ec5482e95393a03d009a79c

Under replicated tablets
Type  Keyspacename         Tablename                      State      Tablet replicas
ysql  yugabyte             utl_file_dir                   RUNNING    VOTER:yb-3.local:9100 LEADER,VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             utl_file_dir                   RUNNING    VOTER:yb-3.local:9100 LEADER,VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             utl_file_dir                   RUNNING    LEADER,VOTER:yb-3.local:9100 VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             config                         RUNNING    LEADER,VOTER:yb-3.local:9100 VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             config                         RUNNING    VOTER:yb-3.local:9100 LEADER,VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             config                         RUNNING    LEADER,VOTER:yb-3.local:9100 VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             results                        RUNNING    LEADER,VOTER:yb-3.local:9100 VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             results                        RUNNING    LEADER,VOTER:yb-3.local:9100 VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             results                        RUNNING    VOTER:yb-3.local:9100 LEADER,VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             benchmark_table                RUNNING    LEADER,VOTER:yb-3.local:9100 VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             benchmark_table                RUNNING    LEADER,VOTER:yb-3.local:9100 VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ysql  yugabyte             benchmark_table                RUNNING    VOTER:yb-3.local:9100 LEADER,VOTER:yb-2.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    LEADER,VOTER:yb-2.local:9100 VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    LEADER,VOTER:yb-2.local:9100 VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    VOTER:yb-2.local:9100 LEADER,VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    VOTER:yb-2.local:9100 LEADER,VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    LEADER,VOTER:yb-2.local:9100 VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    LEADER,VOTER:yb-2.local:9100 VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    VOTER:yb-2.local:9100 LEADER,VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    VOTER:yb-2.local:9100 LEADER,VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    VOTER:yb-2.local:9100 LEADER,VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    VOTER:yb-2.local:9100 LEADER,VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    LEADER,VOTER:yb-2.local:9100 VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
ycql  system               transactions                   RUNNING    LEADER,VOTER:yb-2.local:9100 VOTER:yb-3.local:9100 VOTER:yb-1.local:9100
```
When a tablet server is dead, it will additionally show the UUID of the tablet server.  
Additionally, for all the tablets that have a replica on the dead tablet server, these will show up as under replicated. 
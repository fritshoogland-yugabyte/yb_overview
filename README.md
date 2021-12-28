#README: yb-overview

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


Under replicated tablets
Type  Keyspacename         Tablename                      State      Tablet replicas

```
- The first table is the masters overview, equivalent to yb-admin list_all_masters output.  
- The second table is the tablet servers overview, equivalent to the yb-admin list_all_tablet_servers output, excluding the UUIDs.
- The third table is available via http://(master|tablet server):(7000|9000)/drives for each individual server.
- The fourth table is not available via the web UI or yb-admin, only via the master JSON endpoint.
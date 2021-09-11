How does docker work?  That's what I am trying to figure out, this is a barebone 
implementation of a container system.

The goal is to run an appication in an environment where it can not interact with
other processes or data.  Not sure how simple/hard it would be to escape the
sandbox though.  I've heard people say, that docker wasn't build with security in
mind.

### Development Environment

~~~none
meson setup build --cross-file cross.ini
cd build/
~~~

### Build Instructions

~~~none
meson compile
sudo ./jail
~~~

How does docker work?  That's what I am trying to figure out, by writing my own
container implementation.

### Details

The goal is to run an application in an environment where it can not interact with
other processes or files.  The stdin, stdout and stderr file descriptors remain
accessible and can be used to communicate with the host.

The application itself can only see itself as `/sbin/init` and has no access to
other files.

From the perspective of the application it run as the init process with process id 1.
It looks like, it is executed by the root user with user id 0 and group id 0.

If the user that executes `jail` has supplementary groups, they will appear as group
`nobody` with group id 65534.  There are also a few restrictions to which system
calls can be used, in particular `setgroups` can not be used to get rid of the
additional groups.

The application itself, needs to be statically linked, because no dynamic linker
is avaliable.

### Examples

```none
$ jail ./example
UID=0 (eUID=0)
GID=0 (eGID=0)
PID=1 PPID=0
Walking directory '/':
  /
  /sbin
  /sbin/init
```

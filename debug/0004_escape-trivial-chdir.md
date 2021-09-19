commitid 1d2988a04c127b550c04802b98bb42a82da81af8

It appears, we are able to escape the jail with `chdir("..")`.  It should
not be this easy.

### Notes

-   I thought, that `pivot_root` would prevent this exact situation.

-   If we use a different directory for `put_old`, this trick is no longer
    possible.

-   I've tried several things, but was unable to unmount the old root.  I
    receive `EBUSY`, even if `execve()` is called before.

### Ideas

-   Try `umount` the bind from `/tmp/jail.XXXXXX` before `execve`.

### Theories

-   I suspect, that the old root has to be unmounted somehow, otherwise it
    is still possible to escape.

-   I suspect, that the executable itself is on the old root and thus holds
    the mount busy.

### Actions

-   Use `umount2(".", MNT_DETACH)` like described in `pivot_root(2)` somehow, I
    misremembered this to be just `umount(".")`.  This also explains what why the
    `execve()` is necessary.  The executable that runs `pivot_root` is still
    sitting in the old root, we can therefor not unmount it completely.

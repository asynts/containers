#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <sys/mount.h>
#include <sys/syscall.h>

// Refer to documentation in Rust bindings: 'asynts_jail_sys::ChildArgumentsFFI'.
struct child_args {
    char *root_directory;
};

int child_main_impl(struct child_args *args)
{
    // Do not propagate changes to mounts to other namespaces.  Note that we are in
    // a new namespace because of 'CLONE_NEWNS'.
    {
        int retval = mount(NULL, "/", NULL, MS_REC|MS_PRIVATE, NULL);
        assert(retval == 0);
    }

    // To be able to use 'pivot_root', the new root needs to be a mount point.
    {
        int retval = mount(args->root_directory, args->root_directory, NULL, MS_BIND, NULL);
        assert(retval == 0);
    }

    // Change the '/' mount point to new root.
    {
        int retval;

        retval = chdir(args->root_directory);
        assert(retval == 0);

        // For some reason, the Linux kernel provides this weird API where the old root
        // is made avaliable in the new root.  I don't completely understand why, I suspect
        // that all the mounts are tied to this mount and stuff would break otherwise?  But
        // then again, we do drop it completely when we 'execve()'?
        //
        // FIXME: Can we pass 'put_old=NULL'?
        // FIXME: Figure out what this system call does internally.
        retval = syscall(SYS_pivot_root, ".", ".");
        assert(retval == 0);

        // We do not need to unmount '.' since we call execve later.
        //
        // FIXME: Verify that this is actually correct!
    }

    // Now change what '/' means in the path resolution process.
    {
        int retval = chroot(".");
        assert(retval == 0);
    }

    // Everything has been prepared; launch the application.
    {
        char *argv[] = {
            strdup("/application"),
            NULL
        };
        char *envp[] = {
            NULL
        };

        // This will remove the last reference to the old root mount at '.'; it will automatically
        // be unmounted.
        //
        // FIXME: Verify that this is actually correct!
        execve("/application", argv, envp);

        // Never reached.
        assert(0);
    }

    // We do have to explicitly call 'exit()' here, because this process was created using
    // 'clone()'.  Some cleanup, like flushing the standard output for example, would not
    // happen otherwise.
    //
    // FIXME: Since we never reach this point, this is pointless now, but when I do error handling
    //        it would be lovely, if I remember to do this correctly.
    exit(0);
}

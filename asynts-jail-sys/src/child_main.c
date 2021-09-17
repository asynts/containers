#include <assert.h>
#include <stdlib.h>
#include <sys/mount.h>

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

    // To be able to use 'pivot_root', the target directory needs to be a mount point.
    {
        int retval = mount(args->root_directory, args->root_directory, NULL, MS_BIND, NULL);

        // FIXME: Currently, we fail here, are we already to restricted to use this function?
        assert(retval == 0);
    }

    // FIXME: https://github.com/asynts/jail/blob/2e872feea951746c18a4f29f74755e8fb075a696/jail.cpp

    return 0;
}

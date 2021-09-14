// Launch application in jail.

#include <unistd.h>
#include <assert.h>
#include <sched.h>
#include <sys/wait.h>
#include <sys/mount.h>
#include <sys/syscall.h>

#include <string>
#include <filesystem>

#include <fmt/core.h>

// FIXME: To which extend can we use this to run untrusted code?
//
//        Can we mount procfs back in?

// FIXME: If we didn't remove the mount of the old root, would this
//        cause issues?  Could we escape the sandbox?
//
//        If we change the old parameter to new 'root' directory, we
//        gain access to everything!

// FIXME: Error handling

char container_directory[] = "/tmp/container.XXXXXXXX";

struct {
    char bottom[0x2000];
    char top[];
} child_stack;

int child_main(void *) {
    // Do not propagate changes to mounts to other namespaces.  Note that we are in
    // a new namespace because of 'CLONE_NEWNS'.
    {
        int retval = mount(nullptr, "/", nullptr, MS_REC|MS_PRIVATE, nullptr);
        assert(retval == 0);
    }

    // To be able to use 'pivot_root', the target directory needs to be a mount point.
    {
        int retval = mount(container_directory, container_directory, nullptr, MS_BIND, nullptr);
        assert(retval == 0);
    }

    // Change the '/' mount point to the container directory.
    {
        int retval;
        
        retval = chdir(container_directory);
        assert(retval == 0);

        // This sets our container directory as root mount, but it also makes the old root
        // mount avaliable at '/'.
        retval = syscall(SYS_pivot_root, ".", ".");
        assert(retval == 0);

        // We do not need to unmount '.' since we call execve later.
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
            nullptr
        };
        char *envp[] = {
            nullptr
        };

        // This will remove the last reference to the old root mount at '.'; it will automatically
        // be unmounted.
        execve("/application", argv, envp);

        // Never reached.
        assert(0);
    }
}

int main() {
    // Many of the following operations require root privileges.
    if (geteuid() != 0) {
        assert(0);
    }

    // Prepare container directory.
    {
        char *retval = mkdtemp(container_directory);
        assert(retval != NULL);
        fmt::print("creating container in {}\n", container_directory);

        std::filesystem::copy(
            "/home/me/dev/containers/build/application",
            std::filesystem::path{container_directory} / "application"
        );
    }

    // Create new process in new namespaces.
    pid_t child_pid = -1;
    {
        child_pid = clone(
            child_main,
            child_stack.top,
            CLONE_NEWCGROUP|CLONE_NEWIPC|CLONE_NEWNET|CLONE_NEWNS|CLONE_NEWPID|CLONE_NEWUSER|CLONE_NEWUTS,
            nullptr
        );
        assert(child_pid >= 0);
    }

    // Wait for child to exit.
    while (waitpid(child_pid, nullptr, 0) != child_pid)
        ;
}

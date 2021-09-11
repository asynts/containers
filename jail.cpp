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
        // FIXME: What would happen if we didn't do this?

        int retval = mount(nullptr, "/", nullptr, MS_REC|MS_PRIVATE, nullptr);
        assert(retval == 0);
    }

    // To be able to use 'pivot_root', the target directory needs to be a mount point.
    {
        // FIXME: What does this do precisely?

        int retval = mount(container_directory, container_directory, nullptr, MS_BIND, nullptr);
        assert(retval == 0);
    }

    // Make the container directory the root '/'.
    {
        int retval;
        
        retval = chdir(container_directory);
        assert(retval == 0);

        // FIXME: Why swap the directores instead of just replacing it?

        // This sets our container directory as root mount, but it also makes the old root
        // mount avaliable at '/'.
        retval = syscall(SYS_pivot_root, ".", ".");
        assert(retval == 0);

        // FIXME: Do we need another 'chroot("/")' here?

        // FIXME: If we didn't remove the mount here, could we escape by remounting '/' to
        //        something else?

        // Get rid of the old mount, this is a bit weird since old and new mount are identical.
        retval = umount2(".", MNT_DETACH);
        assert(retval == 0);
    }

    // Everything has been prepared; launch the application.
    {
        char *argv[] = {
            strdup("/app"),
            nullptr
        };
        char *envp[] = {
            nullptr
        };

        execve("/app", argv, envp);

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
            "/home/me/dev/containers/target/x86_64-unknown-linux-musl/debug/asynts-containers-system",
            std::filesystem::path{container_directory} / "app"
        );
    }

    // Create new process in new namespaces.
    pid_t child_pid = -1;
    {
        // FIXME: Verify that all flags make sense and that none are missing.
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

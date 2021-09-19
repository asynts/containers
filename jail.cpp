// Usage: jail <file> <args>...

#include <sched.h>
#include <unistd.h>
#include <assert.h>
#include <fcntl.h>
#include <signal.h>
#include <stdlib.h>

#include <linux/sched.h>

#include <sys/syscall.h>
#include <sys/wait.h>
#include <sys/mount.h>

#include <fmt/core.h>

#include <filesystem>

// FIXME: this is almost a C program, no?

// FIXME: Error handling

// FIXME: Consistency: ' and `

// FIXME: Documentation

static char *jaildir;
static char **jailargv;

void prepare_jaildir(char *pathname) {
    char tempdir[] = "/tmp/jail.XXXXXX";
    {
        char *retval = mkdtemp(tempdir);
        assert(retval != NULL);
    }

    {
        char *retval = strdup(tempdir);
        assert(retval != NULL);

        jaildir = retval;
    }

    std::filesystem::copy(pathname, std::filesystem::path{jaildir} / "app");
}

// This function is used to write the '/proc/<pid>/uid_map' and
// '/proc/<pid>/gid_map' files.  Refer to 'user_namespaces(7)'.
static void write_map_file(const char *path, int true_id) {
    std::string idmap_content = fmt::format("0 {} 1", true_id);

    int idmap_fd;
    {
        int retval = open(path, O_WRONLY);
        assert(retval >= 0);

        idmap_fd = retval;
    }

    {
        ssize_t retval = write(idmap_fd, idmap_content.data(), idmap_content.size());
        assert(retval >= 0);
        assert((size_t)retval == idmap_content.size());
    }

    {
        int retval = close(idmap_fd);
        assert(retval == 0);
    }
}

// After executing this function, it will look like we are root and have all
// capabilities, however, this only applies in a newly created user namespace.
void become_root_in_new_namespace() {
    uid_t true_effective_uid = geteuid();
    gid_t true_effective_gid = getegid();

    // Create a new user namespace and obtain all capabilities in this namespace.
    {
        int retval = unshare(CLONE_NEWUSER);
        assert(retval == 0);
    }

    // This is an oddity of Linux; seems to be a workaround for some security
    // issue.  Essentially, we need to forbid the 'setgroups' system call, which
    // will be allowed again when '/proc/self/gid_map' has been written.
    //
    // This is particular annoying, because we can not leave supplementatry groups
    // so any other groups, e.g. 'wheel' are mapped to 'nobody'.
    {
        int fd;
        {
            int retval = open("/proc/self/setgroups", O_WRONLY);
            assert(retval >= 0);

            fd = retval;
        }

        {
            ssize_t retval = write(fd, "deny", 4);
            assert(retval >= 0);
            assert(retval == strlen("deny"));
        }

        {
            int retval = close(fd);
            assert(retval == 0);
        }
    }

    // Pretend that we are the root user with UID=0 and GID=0.  If we did not do
    // this, we would lose all capabilities when we call 'execve()'.
    write_map_file("/proc/self/uid_map", true_effective_uid);
    write_map_file("/proc/self/gid_map", true_effective_gid);
}

// This function only returns in the child.  It now has PID=1 does not share any
// namespaces with the parent, other than the user namespace which was already
// created.
void clone_into_new_namespaces() {
    struct clone_args clone_args = {0};
    clone_args.flags = CLONE_NEWCGROUP | CLONE_NEWIPC | CLONE_NEWNET | CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWUTS;
    clone_args.exit_signal = SIGCHLD;

    int pid;
    {
        int retval = syscall(SYS_clone3, &clone_args, sizeof(clone_args));
        assert(retval >= 0);

        pid = retval;
    }

    if (pid == 0) {
        // Only the child returns from this function.
        return;
    } else {
        // Wait for the child process to finish.
        for(;;) {
            int wstatus;

            int retval = waitpid(pid, &wstatus, 0);

            if (retval == -1 && errno == EINTR)
                continue;

            assert(retval == pid);
            exit(WEXITSTATUS(wstatus));
        }
    }
}

// FIXME: What does this do exactly; which scenario is prevented?
void disable_mount_propagation() {
    // Do not propagate changes to mounts to other namespaces.  Note that we are in
    // a new namespace because of 'CLONE_NEWNS'.
    {
        int retval = mount(nullptr, "/", nullptr, MS_REC|MS_PRIVATE, nullptr);
        assert(retval == 0);
    }
}

// After this function completed, the filesystem root is moved into a temporary
// directory, and it should not be possible for the application to escape.
void set_root_to_new_tempdir() {
    // To be able to use 'pivot_root', the target directory needs to be a mount point.
    {
        int retval = mount(jaildir, jaildir, nullptr, MS_BIND, nullptr);
        assert(retval == 0);
    }

    // From now on, we only work on this directory.
    {
        int retval = chdir(jaildir);
        assert(retval == 0);
    }

    {
        // This sets our jaildir as root.  According to the documentation, the old
        // root is still avaliable somehow, but should cease to exist when we call
        // 'execve()'.
        //
        // FIXME: Verify that it is not possible to remount the old root.
        int retval = syscall(SYS_pivot_root, ".", ".");
        assert(retval == 0);
    }

    // Now change what '/' means in the path resolution process.  My understanding is,
    // that this is only for backward compatibility.
    {
        int retval = chroot(".");
        assert(retval == 0);
    }

    // We need to unmount the old root directory.  Otherwise, we could escape with
    // `chdir("..")`.  The mount point is still busy because this executable is using
    // it until we `execve()`.
    {
        int retval = umount2(".", MNT_DETACH);
        assert(retval == 0);
    }
}

// Everything has been prepared; launch the application.
void execute_application() {
    char *envp[] = {
        nullptr
    };

    execve("/app", jailargv, envp);

    // Never reached.
    assert(0);
}

int main(int argc, char **argv) {
    // FIXME: Verify linux kernel compatebility.

    assert(argc >= 2);
    assert(argv[argc] == NULL);
    jailargv = argv + 1;

    prepare_jaildir(argv[1]);
    become_root_in_new_namespace();
    clone_into_new_namespaces();
    disable_mount_propagation();
    set_root_to_new_tempdir();
    execute_application();
}

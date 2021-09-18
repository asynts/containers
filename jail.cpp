#include <sched.h>
#include <unistd.h>
#include <assert.h>
#include <fcntl.h>

#include <fmt/core.h>

// This function is used to write the '/proc/<pid>/uid_map' and
// '/proc/<pid>/gid_map' files.  Refer to 'user_namespaces(7)'.
static void write_map_file(const char *path, int true_id) {
    std::string idmap_content = fmt::format("0 {} 1", true_id);

    int idmap_fd;
    {
        int retval = idmap_fd = open(path, O_WRONLY);
        assert(retval >= 0);
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
static void become_root_in_new_namespace() {
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
    {
        int fd;
        {
            int retval = fd = open("/proc/self/setgroups", O_WRONLY);
            assert(retval >= 0);
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

int main() {
    become_root_in_new_namespace();
}

project('containers', 'cpp',
    default_options : ['c_std=c11', 'cpp_std=c++20'])

# FIXME: Verify that we end up with musl libc

fmt_dep = dependency('fmt', native : true)
executable('jail', 'jail.cpp', dependencies : [fmt_dep], native : true)

# FIXME: Link libfmt somehow.
executable('application', 'application.cpp',
    native : false,
    link_args : ['-static'])

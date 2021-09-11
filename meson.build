project('containers', 'c', 'cpp',
    default_options : ['c_std=c11', 'cpp_std=c++20'])

# FIXME: Is it possible to statically link libc++?

musl_dep = dependency('musl')

meson.override_dependency('c_stdlib', musl_dep)

# FIXME: Verify that we end up with musl libc

fmt_dep = dependency('fmt', native : true)
executable('jail', 'jail.cpp', dependencies : [fmt_dep], native : true)

executable('application', 'application.c',
    native : false,
    link_args : ['-static'])

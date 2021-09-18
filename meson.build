project('jail', 'c', 'cpp',
    default_options : ['c_std=c11', 'cpp_std=c++20'])

fmt_dep = dependency('fmt')

executable('jail', 'jail.cpp',
    dependencies : [fmt_dep])

executable('example', 'example.c',
    link_args : ['-static'])

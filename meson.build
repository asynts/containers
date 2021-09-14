project('containers', 'cpp',
    default_options : ['cpp_std=c++20'])

libfmt_dep = dependency('fmt')

executable('jail', 'jail.cpp',
  dependencies : [libfmt_dep])

# FIXME: We do need a Rust application, otherwise, static linking is difficult.
executable('application', 'application.cpp',
  link_args : ['-static'])

project('spacelink', 'rust',
          version: '0.1.0',
    meson_version: '>= 1.1.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

rootdir = ''
prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')
datadir = prefix / get_option('datadir')

# build dependencies
cargo_bin = find_program('cargo', required: true)
find_program('ninja', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)
appstream_cli = find_program('appstreamcli', required: false)

dependency('gio-2.0', version: '>= 2.80')
dependency('glib-2.0', version: '>= 2.80')

# dbus system directory
dbus_dep = dependency('dbus-1')
dbus_interfaces_dir = dbus_dep.get_variable('interfaces_dir', pkgconfig_define: ['datadir', datadir])
dbus_system_bus_services_dir = dbus_dep.get_variable('system_bus_services_dir', pkgconfig_define: ['datadir', datadir])

# systemd unit / service files
systemd_systemdsystemunitdir = get_option('systemdsystemunitdir')
install_systemdunitdir = (systemd_systemdsystemunitdir != 'no')

if install_systemdunitdir and systemd_systemdsystemunitdir == ''
  systemd_dep = dependency('systemd', not_found_message: 'systemd required but not found, please provide a valid systemd user unit dir or disable it')
  systemd_systemdsystemunitdir = systemd_dep.get_variable('systemdsystemunitdir', pkgconfig_define: ['root_prefix', get_option('prefix')])
endif

# haptic feedback for mobile devices (via Purism's feedbackd)
dependency('libfeedback-0.0', version: '>= 0', required: get_option('use-feedbackd'))

# app global variables
app_name = 'Spacelink'
base_id = 'com.maxrdz.Spacelink'
base_binary = 'spacelink'
base_daemon_binary = base_binary + 'd'
daemon_dbus_name = base_id + '.Daemon'
git_url = 'https://gitlab.com/maxrdz/spacelink'

# Cargo build options and environment variables
cargo_opts = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opts += [ '--target-dir', meson.project_build_root()  / 'target' ]
cargo_opts += [ '--color=always' ]

cargo_env = environment()
cargo_env.set('CARGO_HOME', meson.project_build_root() / 'cargo-home')

if get_option('profile') == 'release'
  cargo_opts += [ '--release' ]
  rust_target = 'release'

  app_id = base_id

  binary = base_binary
  daemon_binary = base_daemon_binary

  message('Building Spacelink for Release.')
else
  rust_target = 'debug'

  app_id = '@0@.Debug'.format(base_id)

  binary = '@0@-debug'.format(base_binary)
  daemon_binary = '@0@-debug'.format(base_daemon_binary)

  message('Building Spacelink for Debug.')
endif

target_dir = meson.project_build_root() / 'target' / rust_target

# ---------- Cross Compilation ---------- #

if get_option('target') != ''
  cargo_opts += [ '--target', get_option('target') ]

  target_dir = 'target' / get_option('target') / rust_target

  # For cross compiling, we use the cross-rs
  # project for building our application via Docker.
  # This makes the cross compilation process simpler,
  # and requires less setup on the developer's machine.

  docker_bin = find_program(['docker', 'podman']) # podman also works
  cargo_bin = find_program('cross')

  if get_option('cross-verbose')
    cargo_opts += [ '-v' ]
  endif

  # Emulate arm64 instead of using native cross compilation tools
  # (why emulate? because cross compilation is such a pain)
  run_command(
    docker_bin, 'run', '--privileged',
    '--rm', 'tonistiigi/binfmt', '--install', 'arm64',
    check: true,
  )
endif

# Install Directories
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir = meson.project_source_root() / 'i18n'

base_dir = join_paths(rootdir, prefix)

bin_src = meson.project_build_root() / binary
bin_dst = base_dir / 'bin' / binary

subdir('resources')

if get_option('daemon-only') != true
  subdir('spacelink')
endif
subdir('spacelinkd')

clippy = custom_target(
  'clippy',
  env: cargo_env,
  output: 'clippy',
  command: [
    cargo_bin, 'clippy', '--color=always', cargo_opts,
  ],
)

run_target(
  'linting',
  env: cargo_env,
  command: [
    cargo_bin, 'fmt', '--all', '--', '--color=always', '--check',
  ],
  depends: clippy,
)

cargo_deny = custom_target(
  'cargo-deny',
  env: cargo_env,
  output: 'cargo-deny',
  command: [
    cargo_bin, 'install', 'cargo-deny', '--locked',
  ],
)

run_target(
  'deny',
  env: cargo_env,
  command: [
    cargo_bin, 'deny', '--manifest-path',
    meson.project_source_root() / 'Cargo.toml', 'check',
  ],
  depends: cargo_deny
)

run_target(
  'cargo-clean',
  env: cargo_env,
  command: [
    cargo_bin, 'clean',
  ],
)

message('Setting up Git pre-commit hook..')
run_command('cp', '-f', 'build-aux/git/pre-commit.hook', '.git/hooks/pre-commit', check: false)

meson.add_dist_script(
  'build-aux/meson/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.project_source_root()
)

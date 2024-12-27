# ----- Generated Source Files ----- #

conf = configuration_data()
conf.set_quoted('DBUS_NAME', daemon_dbus_name)

configure_file(
  input: 'src/meson.rs.in',
  output: 'meson.rs',
  configuration: conf
)

# Copy the meson.rs source file to the source directory.
run_command(
  'cp',
  meson.project_build_root() / base_daemon_binary / 'meson.rs',
  meson.project_source_root() / base_daemon_binary / 'src' / 'meson.rs',
  check: true
)

# ----- Cargo Build ----- #

crate_cargo_opts = [ '--package', base_daemon_binary ]
feature_flags = []

# Convert FF list to argument string for --features option
cargo_ff_arg = ''
first_flag = false
foreach ff : feature_flags
  if first_flag == false
    cargo_ff_arg += ff
    first_flag = true
  else
    cargo_ff_arg += ',' + ff
  endif
endforeach

if feature_flags.length() > 0
  crate_cargo_opts += [ '--no-default-features', '--features', cargo_ff_arg ]
else
  message('Building Spacelink Daemon with default feature flags.')
endif

crate_build = custom_target(
  'spacelinkd',
  build_by_default: true,
  build_always_stale: true,
  output: 'spacelinkd-cargo-build', # see `spacelinkd-wrapper` target
  console: true,
  env: cargo_env,
  command: [
    cargo_bin, 'build', cargo_opts, crate_cargo_opts,
  ]
)

# Replace below with `--artifact-dir` cargo option once stable.
# Issue: https://github.com/rust-lang/cargo/issues/6790
custom_target(
  'spacelinkd-wrapper',
  build_by_default: true,
  build_always_stale: true,
  output: base_daemon_binary,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  env: cargo_env,
  command: [
    'cp', target_dir / base_daemon_binary, '@BUILD_ROOT@' / daemon_binary,
  ],
  depends: crate_build,
)

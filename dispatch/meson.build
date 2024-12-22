project_name = meson.project_name()

# ----- Generated Source Files ----- #

pkgdatadir = get_option('prefix') / get_option('datadir') / project_name

conf = configuration_data()
conf.set_quoted('APP_ID', app_id)
conf.set_quoted('APP_NAME', project_name)
conf.set_quoted('APP_REPO', git_url)
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('LOCALEDIR', get_option('prefix') / get_option('localedir'))
conf.set_quoted('PKGDATADIR', pkgdatadir)

configure_file(
  input: 'src/meson.rs.in',
  output: 'meson.rs',
  configuration: conf
)

# Copy the meson.rs source file to the source directory.
run_command(
  'cp',
  meson.project_build_root() / base_binary / 'meson.rs',
  meson.project_source_root() / base_binary / 'src' / 'meson.rs',
  check: true
)

desktop = app_id + '.desktop'
desktop_src = 'resources' / desktop
desktop_dst = datadir / 'applications'

appdata = app_id + '.metainfo.xml'
appdata_dst = datadir / 'appdata'

dbus_activation_file = daemon_dbus_name + '.service'
daemon_service = app_name + '.service'

subdir('icons')

config = configuration_data()
config.set('APP_NAME', app_name)
config.set('APP_ID', app_id)
config.set('ICON', app_id)
config.set('BINARY', binary)
config.set('PROJECT_GIT', git_url)
config.set('DAEMON_BINARY', daemon_binary)
config.set('DBUS_NAME', daemon_dbus_name)

# ---------- DBus Activation File --------- #

configure_file(
  input: 'app.dbus.service.in',
  output: dbus_activation_file,
  configuration: config,
  install_dir: dbus_system_bus_services_dir,
)

# ---------- Daemon Service File --------- #

if install_systemdunitdir
  configure_file(
    input: 'app.service.in',
    output: daemon_service,
    configuration: config,
    install: true,
    install_dir: systemd_systemdsystemunitdir,
  )
endif

# ---------- Desktop File --------- #

desktop_file = configure_file(
  input: 'app.desktop.in',
  output: desktop,
  configuration: config
)

install_data(desktop_file, install_dir: desktop_dst)

if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file
    ],
  )
endif

# ----------- App Data ----------- #

appdata_file = configure_file(
  input: 'app.metainfo.xml.in',
  output: appdata,
  configuration: config
)

install_data(appdata_file, install_dir: appdata_dst)

if appstream_util.found()
  test(
    'validate-appdata', appstream_util,
    args: [
      'validate', '--nonet', appdata_file
    ],
  )
endif

if appstream_cli.found()
  test(
    'validate-appstreamcli', appstream_cli,
    args: [
      'validate', '--no-color', '--no-net', appdata_file
    ],
  )
endif

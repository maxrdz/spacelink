desktop = app_id + '.desktop'
desktop_src = 'resources' / desktop
desktop_dst = datadir / 'applications'

appdata = app_id + '.metainfo.xml'
appdata_dst = datadir / 'appdata'

subdir('icons')

config = configuration_data()
config.set('APP_NAME', app_name)
config.set('APP_ID', app_id)
config.set('ICON', app_id)
config.set('BINARY', binary)
config.set('PROJECT_GIT', git_url)

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

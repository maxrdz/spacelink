if get_option('profile') == 'release'
  icon_src_name = 'icon.svg'
else
  icon_src_name = 'icon-debug.svg'
endif

icons_src = 'hicolor'
icons_dst = iconsdir / 'hicolor'

icon_svg_src = icons_src / 'scalable' / 'apps' / icon_src_name
icon_svg_dst = icons_dst / 'scalable' / 'apps'

install_data(
  icon_svg_src,
  install_dir: icon_svg_dst,
  rename: app_id + '.svg',
)

#install_data(
#  icon_svg_src / app_id + '-symbolic.svg',
#  install_dir: icon_svg_dst,
#  rename: app_id + '-symbolic.svg',
#)

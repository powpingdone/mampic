install_data(
  '@0@.svg'.format(application_id),
  install_dir: iconsdir / 'hicolor' / 'scalable' / 'apps'
)

install_data(
  '@0@-symbolic.svg'.format(base_id),
  install_dir: iconsdir / 'hicolor' / 'symbolic' / 'apps',
  rename: '@0@-symbolic.svg'.format(application_id)
)

my_cons = files(
  'no-servers.svg',
  'ampache-logo.svg',
  'mpd-logo.svg',
  'subsonic-logo.svg',
  'error-symbolic.svg',
  'view-more-symbolic.svg',
  'play-symbolic.svg',
)

install_data(
  my_cons,
  install_dir: iconsdir / 'hicolor' / 'symbolic' / 'apps',
)

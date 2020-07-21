// use panbuild::manifests::debian::{parse, dump};
//
const debian_control_example: &str = r###"
Source: package_name
Section: x11
Priority: optional
Maintainer: me <me@cloud.com>
Build-Depends:
 debhelper (>= 12),
 gtk-doc-tools,
 libsecret-1-dev,
 libfeedback-dev,
 libgnome-desktop-3-dev,
 libhandy-0.0-dev (>= 0.0.12),
 libpam0g-dev,
# to run the tests
 at-spi2-core,
 gnome-themes-extra-data,
 phoc,
 xauth,
Standards-Version: 3.2.2
Homepage: https://code.cloud.com/projects/package_name

Package: other_package_name
Architecture: any
Depends:
 ${misc:Depends},
 ${shlibs:Depends},
 fonts-lato,
 gsettings-desktop-schemas,
 phoc (>= 0.4.0),
Recommends:
 feedbackd,
 iio-sensor-proxy,
 gnome-session,
 phoc,
Provides:
 notification-daemon,
 polkit-1-auth-agent,
Description: Here's a description of the sub-package
 on multiple lines.

"###;

#[cfg(test)]
mod debian_tests {
    #[test]
    pub fn test_parse() {

    }

    #[test]
    pub fn test_dump() {

    }
}

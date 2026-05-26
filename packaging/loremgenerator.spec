%define _name loremgenerator
%define _version 2.6.14
%define _release 55
%define debug_package %{nil}

Name: %{_name}
Version: %{_version}
Release: %{_release}
Summary: Lorem Ipsum Generator
License: MIT
Group: Applications/Utilities
URL: https://github.com/XRayAdams/loremgenerator-rs
BugURL: https://github.com/XRayAdams/loremgenerator-rs/issues
Vendor: Konstantin Adamov

Source0: %{_name}-%{_version}.tar.gz
Source1: app.rayadams.loremgenerator.desktop
Source2: app.rayadams.loremgenerator.png
Source3: app.rayadams.loremgenerator.metainfo.xml

Requires: gtk4, libadwaita, libstdc++

%description
A simple and free utility to generate standard Lorem Ipsum text

%prep
%setup -q -n release

%build
# This section is intentionally left blank as we are packaging a pre-compiled Flutter application.

%install
rm -rf %{buildroot}

# Install binary
install -D -m 755 %{_name} %{buildroot}%{_bindir}/%{_name}

# Install locale files
find locale -name "*.mo" | while read mo; do \
    install -D -m 644 "$mo" %{buildroot}/usr/share/${mo}; \
done

# Copy the desktop file
install -D -m 644 %{SOURCE1} %{buildroot}/usr/share/applications/%{_name}.desktop

# Copy the application icon
install -D -m 644 %{SOURCE2} %{buildroot}/usr/share/icons/hicolor/256x256/apps/%{_name}.png

# Copy meta info
install -D -m 644 %{SOURCE3} %{buildroot}%{_datadir}/metainfo/%{name}.metainfo.xml
%files
%{_bindir}/%{_name}
/usr/share/locale/*/LC_MESSAGES/loremgenerator.mo
/usr/share/applications/%{_name}.desktop
/usr/share/icons/hicolor/256x256/apps/%{_name}.png
%{_datadir}/metainfo/%{name}.metainfo.xml

%changelog
*loghere
- Initial RPM release

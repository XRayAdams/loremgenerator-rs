%define _name loremgenerator
%define _version 2.7.18
%define _release 60
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
Source4: README.txt
Source5: LICENSE

Requires: gtk4

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

# Copy documentation and license
install -D -m 644 %{SOURCE4} %{buildroot}%{_docdir}/%{name}/README.txt
install -D -m 644 %{SOURCE5} %{buildroot}%{_licensedir}/%{name}/LICENSE

%find_lang %{_name}

%files -f %{_name}.lang
%doc %{_docdir}/%{name}/README.txt
%license %{_licensedir}/%{name}/LICENSE
%{_bindir}/%{_name}
/usr/share/applications/%{_name}.desktop
/usr/share/icons/hicolor/256x256/apps/%{_name}.png
%{_datadir}/metainfo/%{name}.metainfo.xml

%changelog
*loghere
- Initial RPM release

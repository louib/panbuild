# 2flatpak
![GitHub CI badge](https://github.com/louib/2flatpak/workflows/pep8/badge.svg)
![GitHub CI badge](https://github.com/louib/2flatpak/workflows/flatpak-build/badge.svg)

2flatpak is a repository of scripts to generate flatpak manifests from other build systems.

## Install

## Python3 virtual env
Make sure you have your Python stuff installed.
```
sudo apt-get install python3 python3-pip python3-virtualenv
```

```
python3 -m venv env
source env/bin/activate
pip3 install -r requirements.txt
```

## Building the flatpak
```
flatpak-builder --force-clean build net.louib.2flatpak.yaml
```

## Other related tools
* https://github.com/flatpak/flatpak-builder-tools

## License

??

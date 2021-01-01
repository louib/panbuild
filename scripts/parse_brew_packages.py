"""
Parses the brew JSON packages dumps into panbuild's project format.
"""
import json
import sys
# import os

import fileinput


if __name__ == '__main__':
    brew_packages = json.loads(sys.stdin.read())

    source_packages = {}
    for package in brew_packages:
        current_package_name = package['name']
        print(current_package_name)

        current_package = {}
        current_package['name'] = current_package_name
        current_package['id'] = current_package_name
        current_package['description'] = package['desc']
        current_package['keywords'] = []
        current_package['versions'] = []
        if package.get('versions'):
            if package['versions'].get('stable'):
                current_package['versions'].append(package['versions'].get('stable'))
            if package['versions'].get('devel'):
                current_package['versions'].append(package['versions'].get('devel'))
        if package.get('version'):
            current_package['versions'].append(package['version'])

        current_package['artifact_names'] = []
        for alias in package.get('aliases', []):
            current_package['artifact_names'].append(alias)
        current_package['maintainers'] = []
        current_package['web_urls'] = []
        current_package['web_urls'].append(package['homepage'])
        current_package['vcs_urls'] = []
        current_package['is_core'] = False
        current_package['layer'] = 4
        source_packages[current_package_name] = current_package

    filtered_packages = []
    for package_name in source_packages.keys():
        source_package = source_packages[package_name]
        # We don't really need those that don't have a git url,
        # at least for now.
        if 'vcs_urls' not in source_package:
            continue
        source_package['vcs_urls'] = list(set(source_package['vcs_urls']))
        filtered_packages.append(source_package)

    print(json.dumps(filtered_packages, indent=2))

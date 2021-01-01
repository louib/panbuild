"""
Parses the native debian sources format into panbuild's project format.
"""
import json
# import os

import fileinput


if __name__ == '__main__':
    source_packages = {}
    current_package_name = ""
    current_package = {}

    filtered_packages = []

    for line in fileinput.input():
        line_parts = line.split(':')
        if len(line_parts) < 2:
            if current_package_name:
                source_packages[current_package_name] = current_package
                current_package_name = ""
                current_package = {}
            continue

        field_name = line_parts[0]
        field_value = ''.join(line_parts[1:])
        field_value = field_value.strip(' \n')

        if current_package_name:
            if field_name == 'Package':
                # Finish the current package and start a new one.
                source_packages[current_package_name] = current_package
                current_package_name = ""
                current_package = {}
                continue
            if field_name == 'Vcs-Git':
                current_package['vcs_urls'].append(field_value)
            if field_name == 'Vcs-Browser' or field_name == 'Homepage':
                current_package['web_urls'].append(field_value)
            if field_name == 'Maintainer':
                current_package['maintainers'].append(field_value)
            if field_name == 'Binary':
                for artifact_name in field_value.split(','):
                    current_package['artifact_names'].append(artifact_name.strip())
            if field_name == 'Version':
                current_package['versions'].append(field_value)
        else:
            if field_name == 'Package':
                current_package_name = field_value
                current_package['name'] = current_package_name
                current_package['id'] = current_package_name
                current_package['description'] = ""
                current_package['summary'] = ""
                current_package['keywords'] = []
                current_package['versions'] = []
                current_package['artifact_names'] = []
                current_package['maintainers'] = []
                current_package['web_urls'] = []
                current_package['vcs_urls'] = []
                current_package['is_core'] = False
                current_package['layer'] = 4
                continue

    for package_name in source_packages.keys():
        source_package = source_packages[package_name]
        # We don't really need those that don't have a git url,
        # at least for now.
        if 'vcs_urls' not in source_package:
            continue
        source_package['vcs_urls'] = list(set(source_package['vcs_urls']))
        filtered_packages.append(source_package)

    print(json.dumps(filtered_packages, indent=2))

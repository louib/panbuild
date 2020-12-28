"""
Gets all the known available projects
"""
# import json
# import os

# import yaml
import fileinput


if __name__ == '__main__':
    source_packages = {}
    current_package_name = ""
    current_package = {}

    for line in fileinput.input():
        line_parts = line.split(':')
        if len(line_parts) < 2:
            if current_package_name:
                source_packages[current_package_name] = current_package
                current_package_name = ""
                current_package = {}
            continue

        field_name = line_parts[0]
        field_value = line_parts[1]

        if current_package_name:
            if field_name == 'Package:':

                print("lol")
        else:
            if field_name == 'Package:':
                print("lol")

    # output = yaml.yaml()
    print(source_packages)

"""
Parses the brew JSON packages dumps into panbuild's project format.
"""
import json
import sys


if __name__ == '__main__':
    brew_packages = json.loads(sys.stdin.read())

    projects = {}
    for package in brew_packages:
        current_project_name = package['name']

        current_project = {}
        current_project['name'] = current_project_name
        current_project['id'] = current_project_name
        current_project['description'] = package['desc']
        current_project['keywords'] = []
        current_project['versions'] = []
        if package.get('versions'):
            if package['versions'].get('stable'):
                current_project['versions'].append(package['versions'].get('stable'))
            if package['versions'].get('devel'):
                current_project['versions'].append(package['versions'].get('devel'))
        if package.get('version'):
            current_project['versions'].append(package['version'])

        current_project['artifact_names'] = []
        for alias in package.get('aliases', []):
            current_project['artifact_names'].append(alias)
        current_project['maintainers'] = []

        current_project['web_urls'] = []
        current_project['web_urls'].append(package['homepage'])
        current_project['vcs_urls'] = []
        stable_url = package.get('urls', {}).get('stable', {})
        if stable_url:
            url = stable_url.get('url', '')
            tag = stable_url.get('tag', '')
            # TODO could the revision be useful?
            # revision = stable_url.get('revision', '')
            if url.endswith('.git'):
                current_project['vcs_urls'].append(url)
            else:
                current_project['web_urls'].append(url)
            if tag:
                current_project['versions'].append(tag)

        current_project['is_core'] = False
        current_project['layer'] = 4
        projects[current_project_name] = current_project

    print(json.dumps(list(projects.values()), indent=2))

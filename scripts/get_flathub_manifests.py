import sys

import requests


def get_all_flathub_repositories():
    projects_url = "https://api.github.com/orgs/flathub/repos?type=all&per_page=100"
    projects = []

    next_page_url = projects_url
    while next_page_url:
        print("Fetching page", next_page_url, file=sys.stderr)
        response = requests.get(next_page_url)

        try:
            # Making sure there was no error.
            response.raise_for_status()
        except Exception as e:
            print(e, file=sys.stderr)
            sys.exit(1)

        github_projects = response.json()
        for project in github_projects:
            clone_url = project.get('clone_url', '')
            if not clone_url:
                continue
            print(clone_url)

        link_header = response.headers.get('link')
        if not link_header:
            next_page_url = None
            break

        for link in link_header.split(','):
            [url, rel] = link.split(';')
            if rel.strip() == 'rel="next"':
                # Removing leading < and trailing >
                next_page_url = url.strip()[1:-1]
                break

    return projects


if __name__ == '__main__':
    get_all_flathub_repositories()

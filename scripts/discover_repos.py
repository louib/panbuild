"""
Gets all the known available projects
"""
# import json
import os

# from bs4 import BeautifulSoup
import yaml
import requests


PROJECT_FIELDS = [
    'name',
    'description',
    'long_description',
    'urls',
    'vcs_urls',  # (URLs to git, cvs, hg, bazaar or any other VCS)
    'versions',
    'tags',
    'aliases'
]

PROJECTS_DIR = './projects'


def normalize_project_name(project_name):
    response = project_name
    response = response.lower()
    # at least replacing spaces and tabs.
    response = response.replace(' ', '-')
    response = response.replace('\t', '-')
    response = response.replace('\n', '-')
    response = response.replace('é', 'e')
    response = response.replace('è', 'e')
    response = response.replace('ê', 'e')
    response = response.replace('ó', 'o')
    response = response.replace('ë', 'e')
    response = response.replace('à', 'a')
    return response


def get_all_projects_from_github():
    projects_url = "https://api.github.com/repositories?".format()
    projects = []

    next_page_url = projects_url
    while next_page_url:
        print("Calling projects endpoint at " + next_page_url)
        response = requests.get(next_page_url)

        try:
            # Making sure there was no error.
            response.raise_for_status()
        except Exception as e:
            print(e)
            break

        github_projects = response.json()
        projects.extend(github_projects)
        print("Projects endpoint returned {0} projects ".format(len(github_projects)))

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


def get_projects_from_github():
    github_projects = get_all_projects_from_github()
    projects = []

    for github_project in github_projects:
        # We only consider original projects.
        if github_project.get('fork'):
            continue

        print(github_project)

        # TODO We require at least 1 fork for the project to be considered.

        project = {}
        project['name'] = normalize_project_name(github_project['name'])
        project['description'] = github_project['description']
        project['urls'] = []
        project['urls'].append(github_project['html_url'])
        project['urls'] = sorted(project['urls'])
        project['vcs_urls'] = []
        if github_project.get('full_name'):
            # GitHub always has the same format for those URLs.
            project['vcs_urls'].append('https://github.com/{0}.git'.format(github_project.get('full_name')))
            project['vcs_urls'].append('git@github.com:{0}.git'.format(github_project.get('full_name')))
        project['vcs_urls'] = sorted(project['vcs_urls'])

        # TODO we must use a different endpoint for GitHub
        project['tags'] = []

        # TODO use the README for the long description

        projects.append(project)

    return projects


def get_projects_from_savannah(savannah_url):
    # FIXME must be authenticated to use simple=false
    projects_endpoint = "https://{savannah_url}/projects?visibility=public&simple=false"
    # Example initial request for the full list of projects
    # https://savannah.nongnu.org/search/index.php?type_of_search=soft&words=%%%
    # Subsequent paged requests look like this
    # https://savannah.nongnu.org/search/?type_of_search=soft&words=%2A&offset=25&max_rows=25#results

    # Any URL that is of type /projects/project_name is a candidate project.
    return requests.get(projects_endpoint)


def get_project_from_savannah_page(project_page_url):
    """
    URLs are of the type https://savannah.nongnu.org/projects/project_page
    """

    # Project Homepage
    # Download Area for the releases and downloadable files
    #


def get_updated_project(current_project, new_project):
    """
    Get the updated project base on the newly discovered project.
    """

    if not new_project.get('description') or not current_project.get('description'):
        current_project['description'] = new_project['description']
    elif new_project['description'].strip() not in current_project['description'].strip():
        current_project['description'] += '\n' + new_project['description']

    for tag in new_project.get('tags', []):
        if 'tags' not in current_project:
            current_project['tags'] = []
        if tag in current_project['tags']:
            continue
        current_project['tags'].append(tag)
        current_project['tags'] = sorted(list(set(current_project['tags'])))
    for url in new_project.get('urls', []):
        if 'urls' not in current_project:
            current_project['urls'] = []
        if url in current_project['urls']:
            continue
        current_project['urls'].append(url)
        current_project['urls'] = sorted(list(set(current_project['urls'])))
    for url in new_project.get('vcs_urls', []):
        if 'vcs_urls' not in current_project:
            current_project['vcs_urls'] = []
        if url in current_project['vcs_urls']:
            continue
        current_project['vcs_urls'].append(url)
        current_project['vcs_urls'] = sorted(list(set(current_project['vcs_urls'])))

    return current_project


if __name__ == '__main__':
    projects = []

    projects.extend(get_projects_from_github())

    # You can browse all the projects at http://git.savannah.gnu.org/git/
    # projects.extend(get_projects_from_savannah("savannah.gnu.org"))
    # You can browse all the projects at http://git.savannah.nongnu.org/git/
    # projects.extend(get_projects_from_savannah("savannah.nongnu.org"))
    # projects.extend(get_projects_from_savannah("puszcza.gnu.org.ua"))  # also named ps.gnu.org.ua

    # There is a list of all the source forges online here:
    # https://wiki.p2pfoundation.net/List_of_Community-Hosted_Code_Forge_Instances
    # sourceforge?
    # Example of downloading boost https://sourceforge.net/projects/boost/files/latest/download
    # https://sourceforge.net/software/customer-service/?page=2
    # launchpad?

    # Bitbucket

    # https://repo.or.cz/
    #
    # https://codeberg.org/
    #
    # gitern.com
    #
    # sr.ht?
    # aka https://sir.hat.com"

    if not os.path.exists(PROJECTS_DIR):
        os.mkdir(PROJECTS_DIR)

    print("Saving {0} projects to database.".format(len(projects)))
    for project in projects:
        project_path = PROJECTS_DIR + '/' + project['name'] + '.yaml'

        # Skip those for now, as we're not sure if it should be happening.
        if '--' in project['name']:
            continue

        # TODO get a lock on the project file before updating or creating
        # the associated project file.

        if not os.path.exists(project_path):
            output_file = open(project_path, 'w')
            output_file.write(yaml.dump(project))
            output_file.close()
        else:
            current_project_data = open(project_path, 'r').read()
            current_project = yaml.load(current_project_data)

            for key in list(current_project.keys()):
                if key not in PROJECT_FIELDS:
                    del current_project[key]

            if current_project is None:
                # FIXME this is happening when pyyaml dumps and empty description,
                # or if the description contains a single quote and the string
                # is dumped using a single quote. So basically pyyaml cannot parse
                # what it dumped...
                print("Could not parse YAML file located at {0}.".format(project_path))
                continue

            current_project = get_updated_project(current_project, project)

            output_file = open(project_path, 'w')
            output_file.write(yaml.dump(current_project))

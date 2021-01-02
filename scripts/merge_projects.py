"""
Merge multiple projects files in a directory.
"""
import json
import re
import sys
import os

OUTPUT_FILE_NAME = 'all_projects.json'


if __name__ == '__main__':
    projects_dir = os.environ.get('PB_OUT_DIR', '')
    all_projects = []
    all_projects_filtered = []
    num_files = 0
    num_projects_initial = 0

    if not projects_dir:
        print('Must define PB_OUT_DIR!')
        sys.exit(1)

    if not os.path.isdir(projects_dir):
        print('{} is not a directory!'.format(projects_dir))
        sys.exit(1)

    for file_path in os.listdir(projects_dir):
        # Not merge the projects from our own output!
        if OUTPUT_FILE_NAME in file_path:
            continue

        try:
            file_full_path = projects_dir + file_path
            print('opening file {}'.format(file_full_path))
            projects = json.loads(open(file_full_path, 'r').read())
            all_projects.extend(projects)
            num_files += 1
            num_projects_initial += len(projects)
        except Exception as e:
            print(e)
            continue

    print('Loaded {} projects for {} files.'.format(len(all_projects), num_files))

    for project in all_projects:
        if not len(project.get('vcs_urls', [])):
            continue
        all_projects_filtered.append(project)
    print('{} projects already have vcs urls.'.format(len(all_projects_filtered)))
    all_projects_filtered = []

    for project in all_projects:
        for web_url in project.get('web_urls', []):
            if re.match(r"https://github.com/([\w\d\.]+)/([\w\d\.]+)", web_url):
                project['vcs_urls'].append(web_url)
            if re.match(r"https://gitlab.com/([\w\d\.]+)/([\w\d\.]+)", web_url):
                project['vcs_urls'].append(web_url)

        project['vcs_urls'] = list(set(project.get('vcs_urls', [])))
        if not len(project.get('vcs_urls', [])):
            continue
        all_projects_filtered.append(project)
    print('Writing {} projects.'.format(len(all_projects_filtered)))

    out_file_path = projects_dir + OUTPUT_FILE_NAME
    open(out_file_path, 'w').write(json.dumps(all_projects, indent=2))

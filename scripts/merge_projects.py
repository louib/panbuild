"""
Merge multiple projects files in a directory.
"""
import json
import sys
import os


if __name__ == '__main__':
    projects_dir = os.environ['OUT_DIR']
    all_projects = []
    all_projects_filtered = []
    num_files = 0
    num_projects_initial = 0

    if not projects_dir:
        print('Must define OUT_DIR!')
        sys.exit(1)

    if not os.is_dir(projects_dir):
        print('{} is not a directory!'.format(projects_dir))
        sys.exit(1)

    for file_path in os.listdir(projects_dir):
        try:
            projects = json.loads(open(file_path, 'r'))
            all_projects.concat(projects)
            num_files += 1
            num_projects_initial += len(projects)
        except Exception:
            sys.exit(1)

    for project in all_projects:
        if not len(project.get('vcs_urls', [])):
            continue
        all_projects_filtered.append(project)

    print('Loaded {} projects for {} files.'.format(len(all_projects), num_files))
    print('Writing {} projects.'.format(len(all_projects_filtered)))
    # print(json.dumps(all_projects, indent=2))

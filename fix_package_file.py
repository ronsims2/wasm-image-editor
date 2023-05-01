#!/usr/bin/python3
from argparse import ArgumentParser
import json
from pathlib import Path
import os


parser = ArgumentParser('This script will add a main entry in the package.json.')
parser.add_argument('--pack', help='The path to your package.json.')
parser.add_argument('--main', help='Optional name of the main JS file.  When omitted, this script will guess the name.')
args = parser.parse_args()

package_file_path = args.pack

proj_dir = Path(__file__).parent
package_file_path = proj_dir / 'pkg/package.json'

print(proj_dir.name)

main_entry = args.main if args.main else '{}.js'.format(proj_dir.name.replace('-', '_'))

package_data = None
with open(package_file_path, 'r') as f:
    package_data = json.loads(f.read())

    if package_data.get('main') is None:
        package_data['main'] = main_entry

with open(package_file_path, 'w') as f:
    f.write(json.dumps(package_data, indent=4))

print('package update, main entry:', main_entry)


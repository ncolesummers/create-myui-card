#!/usr/bin/env zx
/* eslint-disable no-undef */
const branch = await $`git branch --show-current`;
const preId = 'rc';
const flags = [
    // don't commit at all since we're running this as a pre-commit hook.
    '--commit-hooks=false',
    // when I tried running this as a post-commit hook, git ran in an infinite loop, even with commit hooks disabled.
    '--git-tag-version=false'
];
let didStuff = false;

echo('Running version bump git hook');
if (branch.stdout.includes('sprints/')) {
    echo('Bumping version as release candidate for sprint branch');
    await $`npm version prerelease --preid=${preId} ${flags}`;
    didStuff = true;
} else if (branch.stdout.includes('test')) {
    echo('Bumping version for test branch');
    await $`npm version patch ${flags}`;
    didStuff = true;
} else {
    echo('No version bump for this branch');
}

if (didStuff) {
    echo('Adding package-lock.json and package.json to commit');
    await $`git add package-lock.json package.json`;
}
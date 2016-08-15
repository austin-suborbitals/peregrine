#!/bin/bash

set -e

#. ./scripts/travis-doc-upload.cfg
PROJECT_OWNER=austin-suborbitals
PROJECT_NAME=peregrine
SSH_KEY_TRAVIS_ID=345c4e0f5347


if [[ "$TRAVIS_BRANCH" != "master" || "$TRAVIS_PULL_REQUEST" = "false" ]]; then
    echo "refusing to build docs for branch or PR"
    exit 0
fi

eval key=\$encrypted_${SSH_KEY_TRAVIS_ID}_key
eval iv=\$encrypted_${SSH_KEY_TRAVIS_ID}_iv

mkdir -p ~/.ssh
openssl aes-256-cbc -K $key -iv $iv -in scripts/prg_doc_upload.enc -out ~/.ssh/id_rsa -d
chmod 600 ~/.ssh/id_rsa

git clone --branch gh-pages git@github.com:$PROJECT_OWNER/$PROJECT_NAME deploy_docs

cd deploy_docs
git config user.name "Travis-Generated Documentation"
git config user.email "$PROJECT_NAME@travis-ci.org"
mv ../target/doc/* ./

# create an index.html redirect
echo "<meta http-equiv=\"refresh\" content=\"0; URL='$PROJECT_NAME/index.html'\" />" > index.html

git add --all .
git commit -qm "doc upload for $PROJECT_NAME ($TRAVIS_REPO_SLUG)"
git push -q origin gh-pages
